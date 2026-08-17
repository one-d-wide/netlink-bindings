use std::ffi::CString;

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::{
    gen_utils::{escape_md, kebab_to_type, kebab_to_upper, sanitize_ident},
    parse_spec::{CBitFieldType, ConstValue, DefType, Definition, EnumEntry, Spec},
    Context,
};

#[derive(Debug)]
pub struct GenImplStruct {
    pub off: usize,
    pub bit_off: usize,
    pub last_bit_type: Option<(CBitFieldType, usize, Ident)>,
    pub alignment: usize,
    /// Alignment, considering that structs will be packed(4)
    pub final_alignment: usize,
    pub has_struct_field: bool,
    pub derive_debug: bool,
}

pub fn gen_defs(spec: &Spec, ctx: &mut Context) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    for def in &spec.definitions {
        // Structs are generated in gen_writable
        if matches!(&def.def, DefType::Struct { .. }) {
            continue;
        }

        gen_def(&mut tokens, ctx, def);
    }

    tokens
}

fn gen_def(tokens: &mut TokenStream, ctx: &mut Context, def: &Definition) {
    let doc = def
        .doc
        .as_ref()
        .map(|doc| escape_md(ctx, doc))
        .map(|doc| quote!(#[doc = #doc]))
        .unwrap_or(quote!());
    tokens.extend(doc.clone());

    match &def.def {
        DefType::Const { value } => {
            let const_name = format_ident!("{}", kebab_to_upper(&def.name));
            let (ty, val) = match value {
                ConstValue::U64(val) => (quote!(u64), quote!(#val)),
                ConstValue::String(val) => {
                    let cconst_name = format_ident!("{}_CSTR", kebab_to_upper(&def.name));
                    let cval = CString::new(val.as_bytes()).unwrap();

                    tokens.extend(quote! {
                        pub const #const_name: &str = #val;
                        #doc
                        pub const #cconst_name: &CStr = #cval;
                    });
                    return;
                },
            };
            tokens.extend(quote! {
                pub const #const_name: #ty = #val;
            });
        }
        DefType::Flags {
            value_start,
            entries,
        } => gen_def_enum(
            tokens,
            ctx,
            &def.name,
            *value_start,
            entries,
            0,
            |i| {
                let i = Literal::u64_unsuffixed(i);
                quote!(1 << #i)
            },
            "Flags - defines an integer enumeration, with values for each entry occupying a bit, starting from bit 0, (e.g. 1, 2, 4, 8)",
        ),
        DefType::Enum {
            value_start,
            entries,
        } => gen_def_enum(
            tokens,
            ctx,
            &def.name,
            *value_start,
            entries,
            0,
            |i| Literal::u64_unsuffixed(i).to_token_stream(),
            "Enum - defines an integer enumeration, with values for each entry incrementing by 1, (e.g. 0, 1, 2, 3)",
        ),
        ty => unreachable!("{ty:?}"),
    }
}

fn gen_def_enum(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    name: &str,
    value_start: Option<u64>,
    entries: &[EnumEntry],
    default: u64,
    update: fn(u64) -> TokenStream,
    comment: &str,
) {
    let mut variants = TokenStream::new();
    let mut from_value = TokenStream::new();

    let mut next_id: u64 = value_start.unwrap_or(default);
    for variant in entries {
        let name = match variant {
            EnumEntry::NameOnly(name) => name,
            EnumEntry::Extended { name, value, doc } => {
                if let Some(id) = value {
                    next_id = *id;
                }

                if let Some(doc) = &doc {
                    let doc = escape_md(ctx, doc);
                    variants.extend(quote!(#[doc = #doc]));
                };

                name
            }
        };
        let variant = sanitize_ident(&kebab_to_type(name));
        let expr = update(next_id);

        variants.extend(quote! {
            #variant = #expr,
        });

        if expr.clone().into_iter().count() == 1 {
            from_value.extend(quote!(#expr => Self::#variant,));
        } else {
            from_value.extend(quote!(n if n == #expr => Self::#variant,));
        }

        next_id += 1;
    }

    let type_name = format_ident!("{}", kebab_to_type(name));
    tokens.extend(quote! {
        #[doc = #comment]
        #[derive(Debug, Clone, Copy)]
        pub enum #type_name {
            #variants
        }

        impl #type_name {
            pub fn from_value(value: u64) -> Option<Self> {
                Some(match value {
                    #from_value
                    _ => return None,
                })
            }
        }
    });
}
