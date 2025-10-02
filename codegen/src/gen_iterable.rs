use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gen_attrs::gen_attr_type,
    gen_defs::GenImplStruct,
    gen_ops::OpHeader,
    gen_sub_message::{self},
    gen_utils::{kebab_to_rust, kebab_to_type, lifetime_needed_attrs, sanitize_ident},
    gen_writable::writable_type,
    parse_spec::{AttrProp, AttrSet, AttrType, ByteOrder, IndexedArrayType, Spec},
    Context,
};

pub fn gen_iterable_attrs(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    m: &mut GenImplStruct,
    set: &AttrSet,
    fixed_header: Option<&OpHeader>,
    superset: Option<&AttrSet>,
) {
    let mut variants = TokenStream::new();

    let mut type_to_str = TokenStream::new();

    let type_name = &m.type_name;

    let mut id: u16 = 0;
    for next in &set.attributes {
        id += 1;
        if let Some(val) = &next.value {
            id = val.parse().unwrap();
        }

        let name_str = kebab_to_type(&next.name);
        let name = sanitize_ident(&name_str);

        type_to_str.extend(quote! {
            #id => #name_str,
        });

        let buf_name = format_ident!("next");
        let mut push = |inner| {
            variants.extend(quote! {
                #id => #type_name::#name({
                    let res = #inner;
                    let Some(val) = res else { break };
                    val
                }),
            })
        };

        match &next.r#type {
            AttrType::Unused => {}
            AttrType::Flag => {
                variants.extend(quote! {
                    #id => #type_name::#name(()),
                });
            }
            AttrType::SubMessage {
                sub_message,
                selector,
            } => {
                if !gen_sub_message::is_supported(set, next, selector) {
                    continue;
                }

                // TODO: how to handle multiple definitions of selector attribute
                let get_selector = format_ident!("get_{}", kebab_to_rust(selector));

                let (sub, selector_type) =
                    gen_sub_message::sub_message(spec, set, sub_message, selector);
                let name = gen_sub_message::gen_sub(tokens, spec, ctx, sub, selector_type);

                push(quote! {{
                    let Ok(selector) = self.#get_selector() else { break };
                    #name::select_with_loc(selector, #buf_name, self.orig_loc)
                }});
            }
            AttrType::IndexedArray { sub_type } => {
                gen_iterable_array(tokens, ctx, spec, sub_type);
                let parse = gen_iterable_parse(next);
                push(parse);
            }
            _ => {
                let parse = gen_iterable_parse(next);
                push(parse);
            }
        }
    }

    let (impl_lifetime, iterable_lifetime, lifetime) = if lifetime_needed_attrs(set) {
        let l = quote!('a);
        (quote!(<#l>), l.clone(), quote!(<#l>))
    } else {
        (quote!(), quote!('_), quote!())
    };

    let new_impl = if let Some(fixed_header) = fixed_header {
        let header = writable_type(&fixed_header.name);
        // TODO: verify fixed header length and contents
        if fixed_header.construct_header.is_some() {
            quote! {
                pub fn new(buf: &#iterable_lifetime [u8]) -> Iterable<#iterable_lifetime, #type_name #lifetime> {
                    let mut header = #header::new();
                    header.as_mut_slice().clone_from_slice(&buf[..#header::len()]);
                    Iterable::with_loc(&buf[#header::len()..], buf.as_ptr() as usize)
                }
            }
        } else {
            quote! {
                pub fn new(buf: &#iterable_lifetime [u8]) -> (#header, Iterable<#iterable_lifetime, #type_name #lifetime>) {
                    let mut header = #header::new();
                    header.as_mut_slice().clone_from_slice(&buf[..#header::len()]);
                    (header, Iterable::with_loc(&buf[#header::len()..], buf.as_ptr() as usize))
                }
            }
        }
    } else {
        quote! {
            pub fn new(buf: &#iterable_lifetime [u8]) -> Iterable<#iterable_lifetime, #type_name #lifetime> {
                Iterable::new(buf)
            }
        }
    };

    let attr_from_type = if type_to_str.is_empty() {
        quote! {
            fn attr_from_type(r#type: u16) -> Option<&'static str> {
                None
            }
        }
    } else if let Some(superset) = superset {
        let rust_type = format_ident!("{}", kebab_to_type(&superset.name));
        quote! {
            fn attr_from_type(r#type: u16) -> Option<&'static str> {
                #rust_type::attr_from_type(r#type)
            }
        }
    } else {
        quote! {
            fn attr_from_type(r#type: u16) -> Option<&'static str> {
                let res = match r#type {
                    #type_to_str
                    _ => return None,
                };
                Some(res)
            }
        }
    };

    tokens.extend(quote! {
        impl #impl_lifetime #type_name #lifetime {
            #new_impl
            #attr_from_type
        }
    });

    let name_str = format!("{type_name}");
    let item = quote!(#type_name #lifetime);
    tokens.extend(quote! {
        impl #impl_lifetime Iterator for Iterable<#iterable_lifetime, #item> {
            type Item = Result<#item, ErrorContext>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.buf.len() == self.pos {
                    return None;
                }

                let pos = self.pos;
                let mut r#type = None;

                while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
                    r#type = Some(header.r#type);
                    // TODO: check nested flag
                    let res = match header.r#type {
                        #variants
                        n => if cfg!(any(test, feature = "deny-unknown-attrs")) {
                            break
                        } else {
                            continue
                        },
                    };

                    return Some(Ok(res));
                }

                Some(Err(self.error_context(
                    #name_str,
                    r#type.and_then(|t| #type_name::attr_from_type(t)),
                    self.buf.as_ptr().wrapping_add(pos),
                )))
            }
        }
    });
}

pub fn gen_iterable_parse(attr: &AttrProp) -> TokenStream {
    let buf_name = format_ident!("next");

    let ord = match attr.byte_order {
        ByteOrder::Host => "",
        ByteOrder::Little => "_le",
        ByteOrder::Big => "_be",
    };

    let rust_type = match &attr.r#type {
        AttrType::U8 => "u8",
        AttrType::U16 => "u16",
        AttrType::U32 if attr.is_ipv4() => {
            let parse = format_ident!("parse{ord}_u32");
            return quote! { #parse(#buf_name).map(Ipv4Addr::from_bits) };
        }
        AttrType::U32 => "u32",
        AttrType::U64 => "u64",
        AttrType::S8 => "i8",
        AttrType::S16 => "i16",
        AttrType::S32 => "i32",
        AttrType::S64 => "i64",
        AttrType::Binary { .. } if attr.is_ipv6() => {
            let parse = format_ident!("parse{ord}_u128");
            return quote! { #parse(#buf_name).map(Ipv6Addr::from_bits) };
        }
        AttrType::Binary { .. } if attr.is_ip() => {
            let parse = format_ident!("parse_ip");
            return quote! { #parse(#buf_name) };
        }
        AttrType::Binary { .. } if attr.is_sockaddr() => {
            let parse = format_ident!("parse_sockaddr");
            return quote! { #parse(#buf_name) };
        }
        AttrType::Binary {
            r#struct: Some(struct_type),
            ..
        } => {
            let struct_type = writable_type(struct_type);
            return quote! { #struct_type::new_from_slice(#buf_name) };
        }
        AttrType::String => {
            return quote! { CStr::from_bytes_with_nul(#buf_name).ok() };
        }
        AttrType::Pad { .. } | AttrType::Binary { .. } => {
            return quote! { Some(#buf_name) };
        }
        AttrType::IndexedArray { .. } => {
            return quote! { Some(Iterable::with_loc(#buf_name, self.orig_loc)) };
        }
        AttrType::Nest { .. } => {
            return quote! { Some(Iterable::with_loc(#buf_name, self.orig_loc)) };
        }
        r#type => unreachable!("{:?}", r#type),
    };

    let parse = format_ident!("parse{ord}_{rust_type}");

    quote! { #parse(next) }
}

pub fn gen_iterable_array(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    spec: &Spec,
    sub_type: &IndexedArrayType,
) {
    let (item, parse, name_str) = match sub_type {
        IndexedArrayType::Plain { attr } => {
            let (item, _) = gen_attr_type(spec, attr);
            let parse = gen_iterable_parse(attr);
            let parse = quote!({
                let Some(res) = #parse else { break };
                return Some(Ok(res));
            });
            let name_str = item.to_string();
            (item, parse, name_str)
        }
        IndexedArrayType::Nest { nested_attributes } => {
            let set = spec.find_attr(nested_attributes);
            let lifetime = if lifetime_needed_attrs(set) {
                quote!(<'a>)
            } else {
                quote!()
            };

            let type_name = format_ident!("{}", kebab_to_type(nested_attributes));
            let item = quote!(Iterable<'a, #type_name #lifetime>);
            let parse = quote!(Iterable::with_loc(next, self.orig_loc));
            let parse = quote!({
                return Some(Ok(#parse));
            });

            if !ctx.generated_array_iterable.contains(&item.to_string()) {
                tokens.extend(quote! {
                    impl<'a> #type_name #lifetime {
                        pub fn new_array(buf: &'a [u8]) -> Iterable<'a, #item> {
                            Iterable::new(buf)
                        }
                    }
                });
            }

            (item, parse, type_name.to_string())
        }
        sub_type => unreachable!("{sub_type:?}"),
    };

    if ctx.generated_array_iterable.contains(&item.to_string()) {
        return;
    }
    ctx.generated_array_iterable.insert(item.to_string());

    tokens.extend(quote! {
        impl<'a> Iterator for Iterable<'a, #item> {
            type Item = Result<#item, ErrorContext>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.buf.len() == self.pos {
                    return None;
                }

                // TODO: pass the index?
                while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
                    #parse
                }

                Some(Err(self.error_context(
                    #name_str,
                    None,
                    self.buf.as_ptr().wrapping_add(self.pos)
                )))
            }
        }
    });
}
