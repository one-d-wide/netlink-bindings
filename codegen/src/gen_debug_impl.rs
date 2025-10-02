use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gen_attrs::gen_attr_type,
    gen_defs::GenImplStruct,
    gen_utils::{kebab_to_type, lifetime_needed_attrs, sanitize_ident},
    parse_spec::{AttrSet, AttrType, DefType, IndexedArrayType, Spec},
    Context,
};

pub fn gen_introspect_array(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    spec: &Spec,
    sub_type: &IndexedArrayType,
) {
    let fmt_name = format_ident!("fmt");

    let (item, map) = match sub_type {
        IndexedArrayType::Plain { attr } => {
            let (rust_type, _) = gen_attr_type(spec, attr);
            (rust_type, quote!())
        }
        IndexedArrayType::Nest { nested_attributes } => {
            let type_name = format_ident!("{}", kebab_to_type(nested_attributes));

            let set = spec.find_attr(nested_attributes);
            let lifetime = if lifetime_needed_attrs(set) {
                quote!(<'a>)
            } else {
                quote!()
            };

            (
                quote!(Iterable<'a, #type_name #lifetime>),
                quote!(.map(FlattenErrorContext)),
            )
        }
        other => unreachable!("{other:?}"),
    };

    if ctx.generated_array_introspect.contains(&item.to_string()) {
        return;
    }
    ctx.generated_array_introspect.insert(item.to_string());

    tokens.extend(quote! {
        impl<'a> std::fmt::Debug for Iterable<'a, #item> {
            fn fmt(&self, #fmt_name: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #fmt_name.debug_list().entries(self.clone()#map).finish()
            }
        }
    });
}

pub fn gen_introspect_attrs(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    m: &GenImplStruct,
    set: &AttrSet,
) {
    let mut variants = TokenStream::new();

    let type_name = &m.type_name;
    let fmt_name = format_ident!("fmt");

    for next in &set.attributes {
        let name = sanitize_ident(&kebab_to_type(&next.name));
        let field_name = format!("{name}");
        let val_name = format_ident!("val");

        if let AttrType::IndexedArray { sub_type } = &next.r#type {
            gen_introspect_array(tokens, ctx, spec, sub_type);
        }

        match &next.r#type {
            AttrType::Unused => continue,
            _ if next.r#enum.is_some() => {
                let Some(r#enum) = &next.r#enum else {unreachable!()};
                let enum_def = spec.find_def(r#enum);
                let enum_type = format_ident!("{}", kebab_to_type(r#enum));

                let as_flags=next.enum_as_flags.is_some_and(|val| val == true);
                let def_flags = matches!(enum_def.def, DefType::Flags { .. });
                let (formatter, from_val) = if def_flags {
                    (quote!(FormatFlags), quote!(#enum_type::from_value))
                } else if as_flags {
                    (quote!(FormatFlags), quote!(|val| #enum_type::from_value(val.trailing_zeros())))
                } else {
                    (quote!(FormatEnum), quote!(#enum_type::from_value))
                };

                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, &#formatter(#val_name.into(), #from_val)),
                })
            }
            AttrType::Binary { r#struct: None, .. }
                if next.display_hint.as_ref().is_some_and(|h| h == "hex") =>
            {
                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, &FormatHex(#val_name)),
                })
            }
            _ => {
                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, &#val_name),
                })
            }
        }
    }

    let (impl_lifetime, iterable_lifetime, lifetime) = if lifetime_needed_attrs(set) {
        let l = quote!('a);
        (quote!(<#l>), l.clone(), quote!(<#l>))
    } else {
        (quote!(), quote!('_), quote!())
    };

    let name_str = kebab_to_type(&set.name);
    tokens.extend(quote! {
        impl #impl_lifetime std::fmt::Debug for Iterable<#iterable_lifetime, #type_name #lifetime> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut #fmt_name = f.debug_struct(#name_str);
                for attr in self.clone() {
                    let attr = match attr {
                        Ok(a) => a,
                        Err(err) => {
                            #fmt_name.finish()?;
                            f.write_str("Err(")?;
                            err.fmt(f)?;
                            return f.write_str(")");
                        },
                    };
                    match attr {
                        // TODO: consider moving debug formatter to the enum instead
                        #variants
                    };
                }
                #fmt_name.finish()
            }
        }
    });
}
