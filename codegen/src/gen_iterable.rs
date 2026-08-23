use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::Ident;

use crate::{
    gen_attrs::GenAttrs,
    gen_attrs::{gen_attr_type, gen_attr_type_name},
    gen_cstruct::struct_type,
    gen_ops::OpHeader,
    gen_sub_message::{self},
    gen_utils::{kebab_to_rust, kebab_to_type, lifetime_needed_attrs, sanitize_ident},
    parse_spec::{
        AttrProp, AttrSet, AttrType, ByteOrder, DefType, Definition, IndexedArrayType, Spec,
    },
    Context,
};

pub fn iterable_name(name: &str) -> Ident {
    format_ident!("Iterable{}", kebab_to_type(name))
}

pub fn array_iterable_name(name: &str) -> Ident {
    format_ident!("IterableArray{}", kebab_to_type(name))
}

pub fn gen_iterable_attrs(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    m: &mut GenAttrs,
    set: &AttrSet,
    fixed_header: Option<&OpHeader>,
) {
    let mut variants = TokenStream::new();

    let mut type_to_str = TokenStream::new();

    let type_name = &m.type_name;

    let mut selectors_seen = HashSet::new();
    let mut selectors_def = quote!();
    let mut selectors_init = quote!();
    let mut selectors_save = quote!();

    let mut id: u16 = 0;
    for next in &set.attributes {
        id += 1;
        if let Some(val) = &next.value {
            id = *val;
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

                // // TODO: how to handle multiple definitions of selector attribute
                // let get_selector = format_ident!("get_{}", kebab_to_rust(selector));

                let (sub, selector_type) =
                    gen_sub_message::sub_message(spec, set, sub_message, selector);
                let name = gen_sub_message::gen_sub(tokens, spec, ctx, sub, selector_type.clone());

                let sel_type;
                let sel_cast;
                match &selector_type {
                    gen_sub_message::SelectorType::U32 { .. } => {
                        sel_type = quote!(u32);
                        sel_cast = quote!(*sel);
                    }
                    gen_sub_message::SelectorType::CStr => {
                        sel_type = quote!(&'a [u8]);
                        sel_cast = quote!(sel.to_bytes());
                    }
                }
                let selector_var = format_ident!("selector_{}", kebab_to_rust(selector));
                let selector_name = sanitize_ident(&kebab_to_type(selector));

                if selectors_seen.insert(selector.clone()) {
                    selectors_def = quote! {
                        #selectors_def
                        #selector_var: Option<#sel_type>,
                    };
                    selectors_init = quote! {
                        #selectors_init
                        , #selector_var: None
                    };
                    selectors_save = quote! {
                        if let #type_name::#selector_name(sel) = &res {
                            self.#selector_var = Some(#sel_cast);
                        }
                    };
                }

                push(quote! {{
                    // let Ok(selector) = self.#get_selector() else { break };
                    let Some(selector) = self.#selector_var else { break };
                    match #name::select_with_loc(selector, #buf_name, self.orig_loc) {
                        Some(sub) => Some(sub),
                        None if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                        None => continue,
                    }
                }});
            }
            AttrType::IndexedArray { sub_type } => {
                gen_iterable_array(tokens, ctx, spec, sub_type);
                let parse = gen_iterable_parse(spec, next);
                push(parse);
            }
            _ => {
                let parse = gen_iterable_parse(spec, next);
                push(parse);
            }
        }
    }

    let attr_from_type = if type_to_str.is_empty() {
        quote! {
            fn attr_from_type(r#type: u16) -> Option<&'static str> {
                None
            }
        }
    } else if let Some(superset) = &set.subset_of {
        let superset = spec.find_attr(superset);
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

    let mut lifetime = quote!();
    if lifetime_needed_attrs(set) {
        lifetime = quote!(<'a>)
    };

    let new_impl = gen_decoder_new_impl(spec, set, fixed_header).full;

    let mut impl_lifetime = quote!();
    if lifetime_needed_attrs(set) {
        impl_lifetime = quote!(<'_>)
    };

    tokens.extend(quote! {
        impl #type_name #impl_lifetime {
            #new_impl
            #attr_from_type
        }
    });

    let name_str = format!("{type_name}");
    let item = quote!(#type_name #lifetime);
    let iter = iterable_name(&name_str);
    tokens.extend(quote! {
        #[derive(Clone, Copy, Default)]
        pub struct #iter<'a> {
            buf: &'a [u8],
            // Current position of the iterable in the [`buf`]
            pos: usize,
            // Pointer to the beginning of the first slice in the chain.
            // Only used in calculating byte offset for error context.
            orig_loc: usize,
            #selectors_def
        }

        impl<'a> #iter<'a> {
            fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
                Self { buf, pos: 0, orig_loc #selectors_init }
            }

            pub fn get_buf(&self) -> &'a [u8] {
                self.buf
            }
        }

        impl<'a> Iterator for #iter<'a> {
            type Item = Result<#item, ErrorContext>;

            fn next(&mut self) -> Option<Self::Item> {
                let mut pos;
                let mut r#type;

                loop {
                    pos = self.pos;
                    r#type = None;

                    if self.buf.len() == self.pos {
                        return None;
                    }

                    let Some((header, next)) = chop_header(self.buf, &mut self.pos) else {
                        self.pos = self.buf.len();
                        break;
                    };
                    r#type = Some(header.r#type);

                    // TODO: check nested flag
                    let res = match header.r#type {
                        #variants
                        n if cfg!(any(test, feature = "deny-unknown-attrs")) => break,
                        n => continue,
                    };

                    #selectors_save

                    return Some(Ok(res));
                }

                Some(Err(ErrorContext::new(
                    #name_str,
                    r#type.and_then(|t| #type_name::attr_from_type(t)),
                    self.orig_loc,
                    self.buf.as_ptr().wrapping_add(pos) as usize,
                )))
            }
        }
    });
}

pub fn gen_iterable_parse(spec: &Spec, attr: &AttrProp) -> TokenStream {
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
            r#struct: Some(r#struct),
            ..
        } => {
            let struct_type = struct_type(spec, r#struct);
            return match &spec.try_find_def(r#struct) {
                Some(Definition {
                    def:
                        DefType::Struct {
                            shrinkable: true, ..
                        },
                    ..
                }) => quote! { Some(#struct_type::new_from_zeroed(#buf_name)) },
                _ => quote! { #struct_type::new_from_slice(#buf_name) },
            };
        }
        AttrType::String => {
            return quote! { CStr::from_bytes_with_nul(#buf_name).ok() };
        }
        AttrType::Pad { .. } | AttrType::Binary { .. } => {
            return quote! { Some(#buf_name) };
        }
        AttrType::IndexedArray { sub_type, .. } => {
            let arr = match sub_type {
                IndexedArrayType::Plain { attr } => {
                    let name_str = gen_attr_type_name(spec, attr);
                    array_iterable_name(&name_str)
                }
                IndexedArrayType::Nest { nested_attributes } => {
                    array_iterable_name(nested_attributes)
                }
                sub_type => unreachable!("{sub_type:?}"),
            };
            return quote! { Some(#arr::with_loc(#buf_name, self.orig_loc)) };
        }
        AttrType::Nest {
            nested_attributes, ..
        } => {
            let iter = iterable_name(nested_attributes);
            return quote! { Some(#iter::with_loc(#buf_name, self.orig_loc)) };
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
    let arr;
    let item;
    let attrs_name;
    let parse;
    let name_str;

    match sub_type {
        IndexedArrayType::Plain { attr } => {
            (item, _) = gen_attr_type(spec, attr);
            attrs_name = None;
            let parse_attr = gen_iterable_parse(spec, attr);
            parse = quote!({
                let Some(res) = #parse_attr else { break };
                return Some(Ok(res));
            });
            name_str = gen_attr_type_name(spec, attr);
            arr = array_iterable_name(&name_str);
        }
        IndexedArrayType::Nest { nested_attributes } => {
            arr = array_iterable_name(nested_attributes);

            name_str = kebab_to_type(nested_attributes);
            attrs_name = Some(format_ident!("{}", name_str));

            let iter = iterable_name(nested_attributes);
            let parse_attr = quote!(#iter::with_loc(next, self.orig_loc));
            parse = quote!({
                return Some(Ok(#parse_attr));
            });

            item = quote!(#iter<'a>);
        }
        sub_type => unreachable!("{sub_type:?}"),
    }

    if ctx.generated_array_iterable.contains(&name_str) {
        return;
    }
    ctx.generated_array_iterable.insert(name_str.clone());

    if let IndexedArrayType::Nest { nested_attributes } = sub_type {
        let set = spec.find_attr(nested_attributes);
        let lifetime = if lifetime_needed_attrs(set) {
            quote!(<'a>)
        } else {
            quote!()
        };
        tokens.extend(quote! {
            impl #lifetime #attrs_name #lifetime {
                pub fn new_array(buf: &[u8]) -> #arr<'_> {
                    #arr::with_loc(buf, buf.as_ptr() as usize)
                }
            }
        });
    }

    tokens.extend(quote! {
        #[derive(Clone, Copy, Default)]
        pub struct #arr<'a> {
            buf: &'a [u8],
            // Current position of the iterable in the [`buf`]
            pos: usize,
            // Pointer to the beginning of the first slice in the chain.
            // Only used in calculating byte offset for error context.
            orig_loc: usize,
        }

        impl<'a> #arr<'a> {
            fn with_loc(buf: &'a [u8], orig_loc: usize) -> Self {
                Self { buf, pos: 0, orig_loc }
            }

            pub fn get_buf(&self) -> &'a [u8] {
                self.buf
            }
        }

        impl<'a> Iterator for #arr<'a> {
            type Item = Result<#item, ErrorContext>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.buf.len() == self.pos {
                    return None;
                }

                // TODO: pass the index?
                while let Some((header, next)) = chop_header(self.buf, &mut self.pos) {
                    #parse
                }

                let pos = self.pos;
                self.pos = self.buf.len();

                Some(Err(ErrorContext::new(
                    #name_str,
                    None,
                    self.orig_loc,
                    self.buf.as_ptr().wrapping_add(pos) as usize,
                )))
            }
        }
    });
}

pub struct DecoderNewImpl {
    pub return_type: TokenStream,
    pub body: TokenStream,
    pub full: TokenStream,
}

pub fn gen_decoder_new_impl(
    spec: &Spec,
    set: &AttrSet,
    fixed_header: Option<&OpHeader>,
) -> DecoderNewImpl {
    let return_type;
    let body;
    let iter = iterable_name(&set.name);
    if let Some(fixed_header) = fixed_header {
        let header = struct_type(spec, &fixed_header.name);
        // TODO: verify fixed header length and contents
        if fixed_header.op_header_value.is_some() {
            return_type = quote!(#iter<'a>);
            body = quote! {
                let (_header, attrs) = buf.split_at(buf.len().min(#header::len()));
                #iter::with_loc(attrs, buf.as_ptr() as usize)
            };
        } else {
            return_type = quote!((#header, #iter<'a>));
            body = quote! {
                let (header, attrs) = buf.split_at(buf.len().min(#header::len()));
                (
                    #header::new_from_slice(header).unwrap_or_default(),
                    #iter::with_loc(attrs, buf.as_ptr() as usize),
                )
            };
        }
    } else {
        return_type = quote!(#iter<'a>);
        body = quote! {
            #iter::with_loc(buf, buf.as_ptr() as usize)
        };
    }

    let full = quote! {
        pub fn new<'a>(buf: &'a [u8]) -> #return_type {
            #body
        }
    };

    DecoderNewImpl {
        return_type,
        body,
        full,
    }
}
