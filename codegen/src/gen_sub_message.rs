use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::{
    gen_iterable::iterable_name,
    gen_utils::{kebab_to_rust, kebab_to_type},
    gen_writable::writable_type,
    parse_spec::{AttrProp, AttrSet, Spec, SubMessage, SubMessageFormat},
    Context, WARNING,
};

pub fn sub_message_name(name: &str) -> Ident {
    format_ident!("{}", kebab_to_type(name))
}

pub fn sub_message_push_name(attr_name: &str, value_name: &str) -> Ident {
    format_ident!(
        "nested_{}_{}",
        kebab_to_rust(attr_name),
        kebab_to_rust(value_name)
    )
}

pub fn is_supported(set: &AttrSet, attr: &AttrProp, selector: &str) -> bool {
    if !set.attributes.iter().any(|a| a.name == selector) {
        println!("{WARNING} Sub message attribute {:?} has selector {:?} outside of parent {:?} (currently not supported)", attr.name, selector, set.name);
        return false;
    }

    true
}

pub fn sub_message<'a>(
    spec: &'a Spec,
    current_attrs: &AttrSet,
    sub_message: &str,
    selector: &str,
) -> (&'a SubMessage, SelectorType) {
    let selector_type = current_attrs
        .attributes
        .iter()
        .find(|attr| attr.name == *selector)
        .and_then(|attr| attr.r#enum.clone())
        .map(|enum_name| SelectorType::U32 { enum_name })
        .unwrap_or(SelectorType::CStr);

    let sub = spec.find_sub_message(sub_message);

    (sub, selector_type)
}

#[derive(Debug, Clone)]
pub enum SelectorType {
    U32 { enum_name: String },
    CStr,
}

#[derive(Debug)]
pub struct SubContext {
    pub type_name: Ident,
    pub buf_name: Ident,
    pub loc_name: Ident,
    pub selector_type: SelectorType,
}

pub fn gen_sub(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    sub: &SubMessage,
    selector_type: SelectorType,
) -> Ident {
    let type_name = sub_message_name(&sub.name);
    let type_name_str = type_name.to_string();

    if ctx.generated_sub_messages.contains(&type_name_str) {
        return type_name;
    }
    ctx.generated_sub_messages.insert(type_name_str);

    let mut variants = TokenStream::new();
    let mut selects = TokenStream::new();

    let mut sub_ctx = SubContext {
        type_name: type_name.clone(),
        buf_name: format_ident!("buf"),
        loc_name: format_ident!("loc"),
        selector_type,
    };

    for sub_attr in &sub.formats {
        gen_sub_attr(
            tokens,
            &mut variants,
            &mut selects,
            spec,
            &mut sub_ctx,
            sub_attr,
        );
    }

    tokens.extend(quote! {
        #[derive(Debug, Clone)]
        pub enum #type_name<'a> {
            #variants
        }
    });

    let (sel_type, sel) = match &sub_ctx.selector_type {
        SelectorType::U32 { .. } => (quote!(u32), quote!()),
        SelectorType::CStr => (quote!(&'a CStr), quote!(.to_bytes())),
    };

    tokens.extend(quote! {
        impl<'a> #type_name<'a> {
            fn select_with_loc(selector: #sel_type, buf: &'a [u8], loc: usize) -> Option<Self> {
                // Null character not included
                match selector #sel {
                    #selects
                    _ => None,
                }
            }
        }
    });

    type_name
}

pub fn gen_sub_attr(
    _tokens: &mut TokenStream,
    variants: &mut TokenStream,
    select: &mut TokenStream,
    spec: &Spec,
    sub_context: &mut SubContext,
    sub_attr: &SubMessageFormat,
) {
    let select_ident = format_ident!("{}", kebab_to_type(&sub_attr.value));
    let SubContext {
        type_name,
        buf_name,
        loc_name,
        selector_type,
    } = &sub_context;

    let mut attrs_type = None;
    if let Some(attrs_name) = &sub_attr.attribute_set {
        let iter = iterable_name(attrs_name);
        attrs_type = Some((iter.clone(), quote!(#iter<'a>)));
    }

    let mut header_type = None;
    if let Some(fixed_header) = &sub_attr.fixed_header {
        spec.find_def(fixed_header);
        header_type = Some(writable_type(fixed_header).into_token_stream());
    }

    let value: TokenStream = match selector_type {
        SelectorType::U32 { enum_name } => {
            let enum_name = format_ident!("{}", kebab_to_type(enum_name));
            let value_name = format_ident!("{}", kebab_to_type(&sub_attr.value));
            quote!(val if val == #enum_name::#value_name as u32)
        }
        SelectorType::CStr => {
            Literal::byte_string(sub_attr.value.clone().as_bytes()).into_token_stream()
        }
    };

    // TODO: don't panic on length mismatch
    match (header_type, attrs_type) {
        (Some(h), None) => {
            select.extend(quote! {
                #value => Some(#type_name::#select_ident(#h::new_from_slice(#buf_name)?)),
            });
            variants.extend(quote!(#select_ident(#h),));
        }
        (None, Some((iter, a))) => {
            select.extend(quote! {
                #value => Some(#type_name::#select_ident(#iter::with_loc(#buf_name, #loc_name))),
            });
            variants.extend(quote!(#select_ident(#a),));
        }
        (Some(h), Some((iter, a))) => {
            select.extend(quote! {
                #value => {
                    let (header, attrs) = #buf_name.split_at(#buf_name.len().min(#h::len()));
                    Some(#type_name::#select_ident(
                        #h::new_from_slice(header)?,
                        #iter::with_loc(attrs, #loc_name),
                    ))
                },
            });
            variants.extend(quote!(#select_ident(#h, #a),));
        }
        (None, None) => {
            variants.extend(quote!(#select_ident(),));
        }
    };
}
