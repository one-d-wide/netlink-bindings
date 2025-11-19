use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gen_attrs::gen_attr_type_name,
    gen_defs::GenImplStruct,
    gen_iterable::{array_iterable_name, iterable_name},
    gen_utils::{kebab_to_type, lifetime_needed_attrs, sanitize_ident},
    parse_spec::{AttrSet, AttrType, DefType, IndexedArrayType, Spec},
    Context,
};

pub fn gen_introspect_array(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    _spec: &Spec,
    sub_type: &IndexedArrayType,
) {
    let fmt_name = format_ident!("fmt");

    let arr = match sub_type {
        IndexedArrayType::Plain { attr } => {
            let name_str = gen_attr_type_name(attr);
            array_iterable_name(&name_str)
        }
        IndexedArrayType::Nest { nested_attributes } => array_iterable_name(nested_attributes),
        sub_type => unreachable!("{sub_type:?}"),
    };

    if ctx.generated_array_introspect.contains(&arr.to_string()) {
        return;
    }
    ctx.generated_array_introspect.insert(arr.to_string());

    tokens.extend(quote! {
        impl std::fmt::Debug for #arr<'_> {
            fn fmt(&self, #fmt_name: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #fmt_name.debug_list().entries(self.clone().map(FlattenErrorContext)).finish()
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

                let as_flags = next.enum_as_flags.is_some_and(|val| val);
                let def_flags = matches!(enum_def.def, DefType::Flags { .. });
                let (formatter, from_val) = if def_flags {
                    (quote!(FormatFlags), quote!(#enum_type::from_value))
                } else if as_flags {
                    (quote!(FormatFlags), quote!(|val| #enum_type::from_value(val.trailing_zeros())))
                } else {
                    (quote!(FormatEnum), quote!(#enum_type::from_value))
                };

                let debug = if matches!(next.r#type, AttrType::IndexedArray { .. }) {
                    quote! { &MapFormatArray(#val_name, |v| #formatter(v.into(), #from_val)) }
                } else {
                    quote! { &#formatter(#val_name.into(), #from_val) }
                };

                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, #debug),
                });
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

    let impl_lifetime = if lifetime_needed_attrs(set) {
        quote!(<'a>)
    } else {
        quote!()
    };

    let iter = iterable_name(&set.name);
    let name_str = kebab_to_type(&set.name);
    tokens.extend(quote! {
        impl #impl_lifetime std::fmt::Debug for #iter<'_> {
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
