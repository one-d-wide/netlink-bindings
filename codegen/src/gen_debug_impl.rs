use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gen_attrs::gen_attr_type_name,
    gen_attrs::GenAttrs,
    gen_cstruct::struct_type,
    gen_iterable::{array_iterable_name, iterable_name},
    gen_utils::{kebab_to_type, lifetime_needed_attrs, sanitize_ident},
    parse_spec::{AttrSet, AttrType, DefType, IndexedArrayType, Spec},
    Context,
};

pub fn gen_debug_array(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    spec: &Spec,
    sub_type: &IndexedArrayType,
) {
    let fmt_name = format_ident!("fmt");

    let arr = match sub_type {
        IndexedArrayType::Plain { attr } => {
            let name_str = gen_attr_type_name(spec, attr);
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

pub fn gen_debug_attrs(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    m: &GenAttrs,
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
            gen_debug_array(tokens, ctx, spec, sub_type);
        }

        match &next.r#type {
            AttrType::Unused => continue,
            _ if next.r#enum.is_some() => {
                let r#enum = next.r#enum.as_ref().unwrap();
                let enum_def = spec.find_def(r#enum);
                let enum_type = format_ident!("{}", kebab_to_type(r#enum));

                let as_flags = next.enum_as_flags.is_some_and(|val| val);
                let def_flags = matches!(enum_def.def, DefType::Flags { .. });
                let (formatter, from_val);
                if def_flags {
                    formatter = quote!(FormatFlags);
                    from_val = quote!(#enum_type::from_value);
                } else if as_flags {
                    formatter = quote!(FormatFlags);
                    from_val = quote!(|val| #enum_type::from_value(val.trailing_zeros().into()));
                } else {
                    formatter = quote!(FormatEnum);
                    from_val = quote!(#enum_type::from_value);
                }

                let debug = match &next.r#type {
                    AttrType::IndexedArray { .. } => {
                        quote! { &MapFormatArray(#val_name, |v| #formatter(v.into(), #from_val)) }
                    },
                    AttrType::Binary { r#struct: Some(s), .. }
                        if s == "builtin-bitfield32" =>
                    {
                        quote! { &#formatter(#val_name.value.into(), #from_val) }
                    },
                    _ => quote! { &#formatter(#val_name.into(), #from_val) },
                };

                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, #debug),
                });
            }
            AttrType::Binary { r#struct: None, .. }
                if next.display_hint.as_ref().is_some_and(|h| h.ends_with("[]")) =>
            {
                let c_type = next.display_hint.as_ref().unwrap().strip_suffix("[]").unwrap();
                let rust_type = struct_type(spec, c_type);
                variants.extend(quote! {
                    #type_name::#name(#val_name) => {
                        let iter = #val_name.chunks(#rust_type::len()).map(|b| #rust_type::new_from_zeroed(b));
                        #fmt_name.field(#field_name, &FormatIter(iter))
                    },
                })
            }
            AttrType::Binary { r#struct: None, .. }
                if next.display_hint.as_ref().is_some_and(|h| h == "mac") =>
            {
                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, &FormatMac(#val_name)),
                })
            }
            AttrType::Binary { r#struct: None, .. }
                if next.display_hint.as_ref().is_some_and(|h| h == "hex") =>
            {
                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, &FormatHex(#val_name)),
                })
            }
            AttrType::Binary { r#struct: None, .. }
                if next.display_hint.as_ref().is_some_and(|h| h == "string") =>
            {
                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, &FormatBinStr(#val_name)),
                })
            }
            AttrType::Binary { r#struct: None, .. }
                if next.display_hint.as_ref().is_none() =>
            {
                variants.extend(quote! {
                    #type_name::#name(#val_name) => #fmt_name.field(#field_name, &FormatHexdump(#val_name)),
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
                let mut iter = #iter::with_loc(&[], self.orig_loc);

                // This pretty horrible looking hack spots the unrecognized
                // attributes using the iterator interface.
                for attr in IterateAttrs::new(self.get_buf()) {
                    iter.buf = attr;
                    iter.pos = 0;
                    let Some(attr) = iter.next() else {
                        #fmt_name.field("Err", &FormatUnrecognized(attr));
                        continue;
                    };
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
