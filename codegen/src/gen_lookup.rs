use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gen_defs::GenImplStruct,
    gen_ops::OpHeader,
    gen_utils::{kebab_to_type, lifetime_needed_attrs, sanitize_ident},
    gen_writable::writable_type,
    parse_spec::{AttrSet, AttrType, IndexedArrayType, Spec},
    Context,
};

pub fn gen_lookup(
    tokens: &mut TokenStream,
    _spec: &Spec,
    _ctx: &mut Context,
    m: &GenImplStruct,
    set: &AttrSet,
    fixed_header: Option<&OpHeader>,
) {
    let mut variants = TokenStream::new();

    let type_name = &m.type_name;

    let mut has_inner_lookup = false;

    for next in &set.attributes {
        let name_str = kebab_to_type(&next.name);
        let name = sanitize_ident(&kebab_to_type(&next.name));
        let val_name = format_ident!("val");

        let act = match &next.r#type {
            AttrType::Unused => continue,
            AttrType::IndexedArray {
                sub_type: IndexedArrayType::Nest { .. },
            } => {
                has_inner_lookup = true;
                quote! {
                    for entry in #val_name {
                        let Ok(attr) = entry else { break };
                        (stack, missing) = attr.lookup_attr(offset, missing_type);
                        if !stack.is_empty() {
                            break;
                        }
                    }
                    if !stack.is_empty() {
                        stack.push((#name_str, last_off));
                        break;
                    }
                }
            }
            AttrType::Nest { .. } => {
                has_inner_lookup = true;
                quote! {
                    (stack, missing) = #val_name.lookup_attr(offset, missing_type);
                    if !stack.is_empty() {
                        break;
                    }
                }
            }
            _ => quote! {
                if last_off == offset {
                    stack.push((#name_str, last_off));
                    break;
                }
            },
        };
        variants.extend(quote! {
            #type_name::#name(#val_name) => {
                #act
            },
        });
    }

    let (impl_lifetime, iterable_lifetime, lifetime) = if lifetime_needed_attrs(set) {
        let l = quote!('a);
        (quote!(<#l>), l.clone(), quote!(<#l>))
    } else {
        (quote!(), quote!('_), quote!())
    };

    let correct_for_header_len = if let Some(fixed_header) = fixed_header {
        let name = writable_type(&fixed_header.name);
        quote!(+ #name::len())
    } else {
        quote!()
    };

    let (missing_decl, missing_ret) = if has_inner_lookup {
        (quote!(let mut missing = None;), quote!(missing))
    } else {
        (quote!(), quote!(None))
    };

    let name_str = kebab_to_type(&set.name);
    let lookup_inner = if !variants.is_empty() {
        quote! {
            if cur > offset || cur + self.buf.len() < offset {
                return (stack, None);
            }

            let mut attrs = self.clone();
            let mut last_off = cur + attrs.pos;
            #missing_decl
            while let Some(attr) = attrs.next() {
                let Ok(attr) = attr else { break };
                match attr {
                    #variants
                    _ => {},
                };
                last_off = cur + attrs.pos;
            }

            if !stack.is_empty() {
                stack.push((#name_str, cur));
            }
            (stack, #missing_ret)
        }
    } else {
        quote!((stack, None))
    };

    tokens.extend(quote! {
        impl #impl_lifetime Iterable<#iterable_lifetime, #type_name #lifetime> {
            // TODO: report parsing errors
            pub fn lookup_attr(&self, offset: usize, missing_type: Option<u16>) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
                let mut stack = Vec::new();
                let cur = self.calc_offset(self.buf.as_ptr() as usize);
                if cur == offset #correct_for_header_len {
                    stack.push((#name_str, offset));
                    return (stack, missing_type.and_then(|t| #type_name::attr_from_type(t)));
                }
                #lookup_inner
            }
        }
    });
}
