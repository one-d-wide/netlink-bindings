use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::{
    gen_debug_impl::gen_introspect_attrs,
    gen_defs::GenImplStruct,
    gen_iterable::{array_iterable_name, gen_iterable_attrs, iterable_name},
    gen_ops::OpHeader,
    gen_sub_message::sub_message_name,
    gen_utils::{doc_attr, kebab_to_rust, kebab_to_type, sanitize_ident},
    parse_spec::{AttrProp, AttrSet, AttrType, IndexedArrayType, Spec},
    Context,
};

pub fn shorthand_name(name: &str) -> Ident {
    format_ident!("get_{}", kebab_to_rust(name))
}

pub fn gen_attrsets(spec: &Spec, ctx: &mut Context) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    for set in &spec.attribute_sets {
        gen_attrset(&mut tokens, spec, ctx, set, None, None);
    }

    tokens
}

pub fn gen_attrset(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    set: &AttrSet,
    fixed_header: Option<&OpHeader>,
    superset: Option<&AttrSet>,
) {
    let type_name = format_ident!("{}", kebab_to_type(&set.name));

    let mut variants = TokenStream::new();
    let mut shorthands = TokenStream::new();

    let mut m = GenImplStruct {
        off: 0,
        alignment: 0,
        lifetime_needed: false,
        type_name: type_name.clone(),
    };

    let mut visited = HashSet::new();
    for attr in &set.attributes {
        if visited.contains(&attr.name) {
            continue;
        }
        visited.insert(attr.name.clone());

        gen_attr(tokens, &mut variants, &mut shorthands, spec, &mut m, attr);
    }

    let lifetime_mark = if m.lifetime_needed {
        quote!(<'a>)
    } else {
        quote!()
    };

    tokens.extend(quote! {
        #[derive(Clone)]
        pub enum #type_name #lifetime_mark {
            #variants
        }
    });

    let iter = iterable_name(&set.name);
    tokens.extend(quote! {
        impl<'a> #iter<'a> {
            #shorthands
        }
    });

    gen_iterable_attrs(tokens, spec, ctx, &mut m, set, fixed_header, superset);
    gen_introspect_attrs(tokens, spec, ctx, &m, set);
    crate::gen_lookup::gen_lookup(tokens, spec, ctx, &m, set, fixed_header);
}

pub fn gen_attr(
    _tokens: &mut TokenStream,
    variants: &mut TokenStream,
    shorthands: &mut TokenStream,
    _spec: &Spec,
    m: &mut GenImplStruct,
    attr: &AttrProp,
) {
    if matches!(attr.r#type, AttrType::Unused) {
        return;
    }

    doc_attr(attr, |doc| variants.extend(quote!(#[doc = #doc])));

    let variant_name = sanitize_ident(&kebab_to_type(&attr.name));

    let (rust_type, lifetime_needed) = gen_attr_type(attr);

    m.lifetime_needed |= lifetime_needed;

    variants.extend(quote! {
        #variant_name(#rust_type),
    });

    let type_name = &m.type_name;
    let attrs_str = format!("{type_name}");
    let attr_str = kebab_to_type(&attr.name);
    let get_name = shorthand_name(&attr.name);

    doc_attr(attr, |doc| shorthands.extend(quote!(#[doc = #doc])));

    // TODO: consider propagating errors
    match &attr.r#type {
        _ if attr.multi_attr.unwrap_or(false) => {
            let lifetime = if m.lifetime_needed {
                quote!(<'a>)
            } else {
                quote!()
            };
            shorthands.extend(quote! {
                pub fn #get_name(&self) -> MultiAttrIterable<Self, #type_name #lifetime, #rust_type> {
                    MultiAttrIterable::new(self.clone(), |variant| {
                        if let #type_name::#variant_name(val) = variant {
                            Some(val)
                        } else {
                            None
                        }
                    })
                }
            });
        }
        AttrType::Nest { .. } => shorthands.extend(quote! {
            pub fn #get_name(&self) -> Result<#rust_type, ErrorContext> {
                let mut iter = self.clone();
                iter.pos = 0;
                for attr in iter {
                    if let #type_name::#variant_name(val) = attr? {
                        return Ok(val);
                    }
                }
                Err(ErrorContext::new_missing(
                    #attrs_str,
                    #attr_str,
                    self.orig_loc,
                    self.buf.as_ptr() as usize,
                ))
            }
        }),
        AttrType::IndexedArray { sub_type } => {
            let item_type = match sub_type {
                IndexedArrayType::Plain { attr } => {
                    let (rust_type, _) = gen_attr_type(attr);
                    rust_type
                }
                IndexedArrayType::Nest { nested_attributes } => {
                    let iter = iterable_name(nested_attributes);
                    quote!(#iter<'a>)
                }
                other => unreachable!("{other:?}"),
            };
            shorthands.extend(quote! {
                pub fn #get_name(&self) -> Result<ArrayIterable<#rust_type, #item_type>, ErrorContext> {
                    for attr in self.clone() {
                        if let #type_name::#variant_name(val) = attr? {
                            return Ok(ArrayIterable::new(val));
                        }
                    }
                    Err(ErrorContext::new_missing(
                        #attrs_str,
                        #attr_str,
                        self.orig_loc,
                        self.buf.as_ptr() as usize,
                    ))
                }
            })
        }
        _ => shorthands.extend(quote! {
            pub fn #get_name(&self) -> Result<#rust_type, ErrorContext> {
                let mut iter = self.clone();
                iter.pos = 0;
                for attr in iter {
                    if let #type_name::#variant_name(val) = attr? {
                        return Ok(val);
                    }
                }
                Err(ErrorContext::new_missing(
                    #attrs_str,
                    #attr_str,
                    self.orig_loc,
                    self.buf.as_ptr() as usize,
                ))
            }
        }),
    }
}

pub fn gen_attr_type_name(attr: &AttrProp) -> String {
    match &attr.r#type {
        AttrType::Unused => unreachable!(),
        AttrType::Flag => "()",
        AttrType::U8 => "u8",
        AttrType::U16 => "u16",
        AttrType::U32 if attr.is_ipv4() => "Ipv4Addr",
        AttrType::U32 => "u32",
        AttrType::U64 => "u64",
        AttrType::S8 => "i8",
        AttrType::S16 => "i16",
        AttrType::S32 => "i32",
        AttrType::S64 => "i64",
        AttrType::Binary { .. } if attr.is_ipv6() => "Ipv6Addr",
        AttrType::Binary { .. } if attr.is_ip() => "IpAddr",
        AttrType::Binary { .. } if attr.is_sockaddr() => "SocketAddr",
        AttrType::Binary {
            r#struct: Some(struct_type),
            ..
        } => return format!("Push{}", kebab_to_type(struct_type)),
        AttrType::String => "CStr",
        AttrType::Pad { .. } | AttrType::Binary { .. } => "Binary",
        AttrType::Nest { nested_attributes } => nested_attributes,
        AttrType::SubMessage { sub_message, .. } => sub_message,
        r#type => unreachable!("{:?} at {:?}", r#type, attr),
    }
    .to_string()
}

pub fn gen_attr_type(attr: &AttrProp) -> (TokenStream, bool) {
    let mut lifetime_needed = false;
    let rust_type = match &attr.r#type {
        AttrType::Unused => unreachable!(),
        AttrType::Flag => quote!(()),
        AttrType::U8 => quote!(u8),
        AttrType::U16 => quote!(u16),
        AttrType::U32 if attr.is_ipv4() => quote!(std::net::Ipv4Addr),
        AttrType::U32 => quote!(u32),
        AttrType::U64 => quote!(u64),
        AttrType::S8 => quote!(i8),
        AttrType::S16 => quote!(i16),
        AttrType::S32 => quote!(i32),
        AttrType::S64 => quote!(i64),
        AttrType::Binary { .. } if attr.is_ipv6() => quote!(std::net::Ipv6Addr),
        AttrType::Binary { .. } if attr.is_ip() => quote!(std::net::IpAddr),
        AttrType::Binary { .. } if attr.is_sockaddr() => quote!(std::net::SocketAddr),
        AttrType::Binary {
            r#struct: Some(struct_type),
            ..
        } => format_ident!("Push{}", kebab_to_type(struct_type)).into_token_stream(),
        AttrType::String => {
            lifetime_needed = true;
            quote!(&'a CStr)
        }
        AttrType::Pad { .. } | AttrType::Binary { .. } => {
            lifetime_needed = true;
            quote!(&'a [u8])
        }
        AttrType::IndexedArray { sub_type } => {
            lifetime_needed = true;

            let arr = match sub_type {
                IndexedArrayType::Plain { attr } => {
                    let rust_type = gen_attr_type_name(attr);
                    array_iterable_name(&rust_type)
                }
                IndexedArrayType::Nest { nested_attributes } => {
                    array_iterable_name(nested_attributes)
                }
                other => unreachable!("{other:?}"),
            };
            quote!(#arr<'a>)
        }
        AttrType::Nest { nested_attributes } => {
            lifetime_needed = true;

            let iter = iterable_name(nested_attributes);
            quote!(#iter<'a>)
        }
        AttrType::SubMessage { sub_message, .. } => {
            lifetime_needed = true;
            let name = sub_message_name(sub_message);
            quote!(#name <'a>)
        }
        r#type => unreachable!("{:?} at {:?}", r#type, attr),
    };

    (rust_type, lifetime_needed)
}
