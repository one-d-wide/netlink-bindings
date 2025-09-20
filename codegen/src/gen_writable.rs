use std::{collections::HashSet, ffi::CString};

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::{
    gen_defs::GenImplStruct,
    gen_ops::OpHeader,
    gen_sub_message::{self, SelectorType},
    gen_utils::{doc_attr, kebab_to_rust, kebab_to_type},
    parse_spec::{AttrProp, AttrSet, AttrType, ByteOrder, DefType, IndexedArrayType, Spec},
    Context,
};

pub fn writable_type(name: &str) -> Ident {
    format_ident!("Push{}", kebab_to_type(name))
}

pub fn writable_array_type(attr: &IndexedArrayType) -> Ident {
    let rust_type = match attr {
        IndexedArrayType::Plain { attr } => match &attr.r#type {
            AttrType::U32 => "U32".into(),
            AttrType::Binary {
                r#struct: Some(r#struct),
                ..
            } => writable_type(r#struct).to_string(),
            AttrType::Binary { .. } => "Binary".into(),
            other => unreachable!("{other:?}"),
        },
        IndexedArrayType::Nest { nested_attributes } => kebab_to_type(nested_attributes),
        other => unreachable!("{other:?}"),
    };
    format_ident!("PushArray{}", rust_type)
}

pub fn gen_writable(spec: &Spec, ctx: &mut Context) -> TokenStream {
    let mut tokens = TokenStream::new();

    for set in &spec.attribute_sets {
        gen_writable_attrset(&mut tokens, ctx, spec, set, None);
    }

    for def in &spec.definitions {
        let DefType::Struct { members, .. } = &def.def else {
            continue;
        };
        gen_writable_struct(&mut tokens, spec, &def.name, members);
    }

    tokens
}

pub fn gen_writable_attrset(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    spec: &Spec,
    set: &AttrSet,
    fixed_header: Option<&OpHeader>,
) {
    let type_name = writable_type(&set.name);

    let end = format_ident!("end_nested");

    tokens.extend(quote! {
        pub struct #type_name<Prev: Rec> {
            prev: Option<Prev>,
            header_offset: Option<usize>,
        }
    });

    tokens.extend(quote! {
        impl<Prev: Rec> Rec for #type_name<Prev> {
            fn as_rec_mut(&mut self) -> &mut Vec<u8> {
                self.prev.as_mut().unwrap().as_rec_mut()
            }
        }
    });

    let new_impl = if let Some(fixed_header) = fixed_header {
        let header = writable_type(&fixed_header.name);
        let header_var = format_ident!("header");
        if let Some(fill) = &fixed_header.fill {
            let fill = fill(&header_var);
            quote! {
                pub fn new(mut prev: Prev) -> Self {
                    let mut #header_var = #header::new();
                    #fill
                    prev.as_rec_mut().extend(#header_var.as_slice());
                    Self { prev: Some(prev), header_offset: None }
                }
            }
        } else {
            quote! {
                pub fn new(mut prev: Prev, #header_var: &#header) -> Self {
                    prev.as_rec_mut().extend(#header_var.as_slice());
                    Self { prev: Some(prev), header_offset: None }
                }
            }
        }
    } else {
        quote! {
            pub fn new(prev: Prev) -> Self {
                Self { prev: Some(prev), header_offset: None }
            }
        }
    };

    let mut impls = TokenStream::new();
    impls.extend(quote! {
        #new_impl
        pub fn #end(mut self) -> Prev {
            let mut prev = self.prev.take().unwrap();
            if let Some(header_offset) = &self.header_offset {
                finalize_nested_header(prev.as_rec_mut(), *header_offset);
            }
            prev
        }
    });

    let mut visited = HashSet::new();
    let mut id: u16 = 0;
    for next in &set.attributes {
        id += 1;
        if let Some(val) = &next.value {
            id = val.parse().unwrap();
        }

        if visited.contains(&next.name) {
            continue;
        }
        visited.insert(next.name.clone());

        if let AttrType::Unused = &next.r#type {
            continue;
        }

        doc_attr(next, |doc| impls.extend(quote!(#[doc = #doc])));

        if let AttrType::SubMessage {
            sub_message,
            selector,
        } = &next.r#type
        {
            if !gen_sub_message::is_supported(set, next, selector) {
                continue;
            }

            let sub = spec.find_sub_message(sub_message);

            for sub_attr in &sub.formats {
                let (_, sel_type) = gen_sub_message::sub_message(spec, set, sub_message, selector);

                let sel_val = match &sel_type {
                    SelectorType::U32 { enum_name } => {
                        let enum_type = format_ident!("{}", kebab_to_type(enum_name));
                        let val = format_ident!("{}", kebab_to_type(&sub_attr.value));
                        quote!(#enum_type::#val as u32)
                    }
                    SelectorType::CStr => {
                        let val = CString::new(sub_attr.value.clone()).unwrap();
                        quote!(#val)
                    }
                };

                let mut header_type = None;
                if let Some(fixed_header) = &sub_attr.fixed_header {
                    spec.find_def(fixed_header);
                    header_type = Some(writable_type(fixed_header).into_token_stream());
                }

                let mut attrs_type = None;
                if let Some(attrs_name) = &sub_attr.attribute_set {
                    attrs_type = Some(writable_type(attrs_name).into_token_stream());
                }

                let func = gen_sub_message::sub_message_push_name(&next.name, &sub_attr.value);
                let push_selector = format_ident!("push_{}", kebab_to_rust(selector));

                impls.extend(quote! {
                    #[doc = "Selector attribute is inserted automatically."]
                    #[doc = "At most one sub-message attribute is expected per attribute set."]
                });

                match (header_type, attrs_type) {
                    (Some(h), None) => {
                        impls.extend(quote! {
                            pub fn #func(mut self, fixed_header: &#h) -> Self {
                                self = self.#push_selector(#sel_val);
                                push_nested_header(self.as_rec_mut(), #id);
                                self.as_rec_mut().extend(fixed_header.as_slice());
                                self
                            }
                        });
                    }
                    (None, Some(nested_type)) => {
                        impls.extend(quote! {
                            pub fn #func(mut self) -> #nested_type<Self> {
                                self = self.#push_selector(#sel_val);
                                let header_offset = push_nested_header(self.as_rec_mut(), #id);
                                #nested_type { prev: Some(self), header_offset: Some(header_offset) }
                            }
                        });
                    }
                    (Some(h), Some(nested_type)) => {
                        impls.extend(quote! {
                            pub fn #func(mut self, fixed_header: &#h) -> #nested_type<Self> {
                                self = self.#push_selector(#sel_val);
                                let header_offset = push_nested_header(self.as_rec_mut(), #id);
                                self.as_rec_mut().extend(fixed_header.as_slice());
                                #nested_type { prev: Some(self), header_offset: Some(header_offset) }
                            }
                        });
                    }
                    (None, None) => {
                        impls.extend(quote! {
                            pub fn #func(mut self) -> Self {
                                self = self.#push_selector(#sel_val);
                                push_nested_header(self.as_rec_mut(), #id);
                                self
                            }
                        });
                    }
                };
            }

            continue;
        }

        if let AttrType::IndexedArray { sub_type } = &next.r#type {
            let func = format_ident!("array_{}", kebab_to_rust(&next.name));

            let array_type = gen_writable_index_array(tokens, ctx, spec, sub_type);

            impls.extend(quote! {
                pub fn #func(mut self) -> #array_type<Self> {
                    let header_offset = push_nested_header(self.as_rec_mut(), #id);

                    #array_type { prev: Some(self), header_offset: Some(header_offset), counter: 0 }
                }
            });

            continue;
        }

        if let AttrType::Nest { nested_attributes } = &next.r#type {
            let nested_type = writable_type(nested_attributes);
            let func = format_ident!("nested_{}", kebab_to_rust(&next.name));

            impls.extend(quote! {
                pub fn #func(mut self) -> #nested_type<Self> {
                    let header_offset = push_nested_header(self.as_rec_mut(), #id);

                    #nested_type { prev: Some(self), header_offset: Some(header_offset) }
                }
            });

            continue;
        }

        let func = format_ident!("push_{}", kebab_to_rust(&next.name));
        let value_name = format_ident!("value");

        let (rust_type, inner, plain, len) = gen_writable_type(next);

        let inner = if !inner.is_empty() && !matches!(next.r#type, AttrType::Flag) {
            quote!(self.as_rec_mut().extend(#inner);)
        } else {
            quote!()
        };

        impls.extend(quote! {
            pub fn #func(mut self, #value_name: #rust_type) -> Self {
                push_header(self.as_rec_mut(), #id, #len as u16);
                #inner
                #plain
                self
            }
        });
    }

    tokens.extend(quote! {
        impl<Prev: Rec> #type_name<Prev> {
            #impls
        }
        impl<Prev: Rec> Drop for #type_name<Prev> {
            fn drop(&mut self) {
                if let Some(prev) = &mut self.prev {
                    if let Some(header_offset) = &self.header_offset {
                        finalize_nested_header(prev.as_rec_mut(), *header_offset);
                    }
                }
            }
        }
    });
}

/// (rust_type, encode (immediate encoding), plain (statement before encode), len)
pub fn gen_writable_type(next: &AttrProp) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    let value_name = format_ident!("value");
    let ord = match next.byte_order {
        ByteOrder::Host => "ne",
        ByteOrder::Little => "le",
        ByteOrder::Big => "be",
    };
    let encode = format_ident!("to_{ord}_bytes");

    match &next.r#type {
        AttrType::Flag => (quote!(()), quote!(()), quote!(), quote!(0)),
        AttrType::U32 if next.is_ipv4() => (
            quote!(std::net::Ipv4Addr),
            quote! { &#value_name.to_bits().#encode() },
            quote!(),
            quote!(4),
        ),
        AttrType::Binary { .. } if next.is_ipv6() => (
            quote!(std::net::Ipv6Addr),
            quote! { &#value_name.to_bits().#encode() },
            quote!(),
            quote!(16),
        ),
        AttrType::Binary { .. } if next.is_ip() => (
            quote!(std::net::IpAddr),
            quote!(),
            quote! { encode_ip(self.as_rec_mut(), #value_name); },
            quote! {{
                match &#value_name {
                    IpAddr::V4(_) => 4,
                    IpAddr::V6(_) => 16,
                }
            }},
        ),
        AttrType::Binary { .. } if next.is_sockaddr() => (
            quote!(std::net::SocketAddr),
            quote!(),
            quote! { encode_sockaddr(self.as_rec_mut(), #value_name); },
            quote! {{
                match &#value_name {
                    SocketAddr::V4(_) => 16,
                    SocketAddr::V6(_) => 28,
                }
            }},
        ),
        AttrType::Binary {
            r#struct: Some(struct_type),
            ..
        } => {
            let struct_type = writable_type(struct_type);
            (
                quote!(#struct_type),
                quote!(#value_name.as_slice()),
                quote!(),
                quote!(#value_name.as_slice().len()),
            )
        }
        AttrType::String => (
            quote!(&CStr),
            quote!(#value_name.to_bytes_with_nul()),
            quote!(),
            quote!(#value_name.to_bytes_with_nul().len()),
        ),
        AttrType::Pad { .. } | AttrType::Binary { .. } => (
            quote!(&[u8]),
            quote!(#value_name),
            quote!(),
            quote!(#value_name.len()),
        ),
        r#type => {
            let (rust_type, len) = match r#type {
                AttrType::U8 => (quote!(u8), 1),
                AttrType::U16 => (quote!(u16), 2),
                AttrType::U32 => (quote!(u32), 4),
                AttrType::U64 => (quote!(u64), 8),
                AttrType::S8 => (quote!(i8), 1),
                AttrType::S16 => (quote!(i16), 2),
                AttrType::S32 => (quote!(i32), 4),
                AttrType::S64 => (quote!(i64), 8),
                r#type => unreachable!("{:?}", r#type),
            };
            (
                rust_type,
                quote!(#value_name.#encode()),
                quote!(),
                Literal::u16_unsuffixed(len).into_token_stream(),
            )
        }
    }
}

pub fn gen_def_struct_len(spec: &Spec, r#struct: &str) -> usize {
    let DefType::Struct { members, .. } = &spec.find_def(r#struct).def else {
        unreachable!("{:?}", r#struct);
    };

    let mut len = 0;
    for m in members {
        len += match &m.r#type {
            AttrType::U8 => 1,
            AttrType::U16 => 2,
            AttrType::U32 => 4,
            AttrType::U64 => 8,
            AttrType::S8 => 1,
            AttrType::S16 => 2,
            AttrType::S32 => 4,
            AttrType::S64 => 8,
            AttrType::Pad { len: Some(len) } => *len,
            AttrType::Binary {
                r#struct: Some(r#struct),
                ..
            } => gen_def_struct_len(spec, r#struct),
            r#type => unreachable!("{:?}", r#type),
        };
    }

    len
}

pub fn gen_def_struct_uint_writable(
    spec: &Spec,
    members: &mut TokenStream,
    debug: &mut TokenStream,
    m: &mut GenImplStruct,
    attr: &AttrProp,
) {
    let value_name = format_ident!("value");
    let getter_prefix = match attr.name.as_str() {
        "type" => "r#",
        _ => "",
    };
    let getter_name = format_ident!("{getter_prefix}{}", kebab_to_rust(&attr.name));
    let setter_name = format_ident!("set_{}", kebab_to_rust(&attr.name));

    let mut docs = TokenStream::new();
    doc_attr(attr, |doc| docs.extend(quote!(#[doc = #doc])));

    let (rust_type, len) = match &attr.r#type {
        AttrType::U8 => ("u8", size_of::<u8>()),
        AttrType::U16 => ("u16", size_of::<u16>()),
        AttrType::U32 => ("u32", size_of::<u32>()),
        AttrType::U64 => ("u64", size_of::<u64>()),
        AttrType::S8 => ("i8", size_of::<i8>()),
        AttrType::S16 => ("i16", size_of::<i16>()),
        AttrType::S32 => ("i32", size_of::<i32>()),
        AttrType::S64 => ("i64", size_of::<i64>()),
        AttrType::Pad { len: Some(len) } => {
            m.off += len;
            return;
        }
        AttrType::Binary {
            r#struct: Some(r#struct),
            ..
        } => {
            let rust_type = writable_type(r#struct);

            let len = gen_def_struct_len(spec, r#struct);
            let first = m.off;
            let last = m.off + len;

            members.extend(quote! {
                #docs
                pub fn #getter_name(&self) -> #rust_type {
                    #rust_type::new_from_slice(&self.buf[#first..#last]).unwrap()
                }
                #docs
                pub fn #setter_name(&mut self, #value_name: #rust_type) {
                    self.buf[#first..#last].copy_from_slice(&#value_name.as_slice())
                }
            });

            let name = kebab_to_rust(&attr.name);
            debug.extend(quote! {
                .field(#name, &self.#getter_name())
            });

            return;
        }
        AttrType::Binary {
            r#struct: None,
            len: Some(len),
        } => {
            let rust_type = quote!([u8; #len]);

            let first = m.off;
            let last = m.off + len;

            members.extend(quote! {
                #docs
                pub fn #getter_name(&self) -> #rust_type {
                    self.buf[#first..#last].try_into().unwrap()
                }
                #docs
                pub fn #setter_name(&mut self, #value_name: #rust_type) {
                    self.buf[#first..#last].copy_from_slice(&#value_name)
                }
            });

            let name = kebab_to_rust(&attr.name);
            debug.extend(quote! {
                .field(#name, &self.#getter_name())
            });

            return;
        }
        other => todo!("{:?}", other),
    };

    let rust_type = format_ident!("{}", rust_type);

    let first = m.off;
    let last = m.off + len;

    let ord = match attr.byte_order {
        ByteOrder::Host => "",
        ByteOrder::Little => "_le",
        ByteOrder::Big => "_be",
    };
    let parse = format_ident!("parse{ord}_{rust_type}");

    let encode_ord = match attr.byte_order {
        ByteOrder::Host => "ne",
        ByteOrder::Little => "le",
        ByteOrder::Big => "be",
    };
    let encode = format_ident!("to_{encode_ord}_bytes");

    members.extend(quote! {
        #docs
        pub fn #getter_name(&self) -> #rust_type {
            #parse(&self.buf[#first..#last]).unwrap()
        }
        #docs
        pub fn #setter_name(&mut self, #value_name: #rust_type) {
            self.buf[#first..#last].copy_from_slice(&#value_name.#encode())
        }
    });

    let name = kebab_to_rust(&attr.name);
    debug.extend(quote! {
        .field(#name, &self.#getter_name())
    });

    m.off += len;
}

pub fn gen_writable_struct(
    tokens: &mut TokenStream,
    spec: &Spec,
    name: &str,
    members: &[AttrProp],
) {
    let type_name = format_ident!("Push{}", kebab_to_type(name));
    let name_str = kebab_to_type(name);

    let mut m = GenImplStruct {
        off: 0,
        lifetime_needed: false,
        type_name: type_name.clone(),
    };

    let mut inner = TokenStream::new();
    let mut debug = TokenStream::new();
    for attr in members {
        gen_def_struct_uint_writable(spec, &mut inner, &mut debug, &mut m, attr);
    }

    let fmt_name = format_ident!("fmt");

    let len = m.off;
    let doc = format!("Original name: {:?}", name);
    tokens.extend(quote! {
        #[doc = #doc]
        #[derive(Clone)]
        pub struct #type_name {
            buf: [u8; #len],
        }

        impl #type_name {
            #[doc = "Create zero-initialized struct"]
            pub fn new() -> Self {
                Self { buf: [0u8; Self::len()] }
            }
            #[doc = "Copy from contents from other slice"]
            pub fn new_from_slice(other: &[u8]) -> Option<Self> {
                if other.len() != Self::len() {
                    return None;
                }
                let mut buf = [0u8; Self::len()];
                buf.clone_from_slice(other);
                Some(Self { buf })
            }
            pub fn as_slice(&self) -> &[u8] {
                &self.buf
            }
            pub fn as_mut_slice(&mut self) -> &mut [u8] {
                &mut self.buf
            }
            pub const fn len() -> usize {
                #len
            }
            #inner
        }

        impl std::fmt::Debug for #type_name {
            fn fmt(&self, #fmt_name: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #fmt_name
                    .debug_struct(#name_str)
                    #debug
                    .finish()
            }
        }
    });
}

pub fn gen_writable_index_array(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    _spec: &Spec,
    sub_type: &IndexedArrayType,
) -> Ident {
    let mut impls = TokenStream::new();
    let array_type = writable_array_type(sub_type);

    match sub_type {
        IndexedArrayType::Plain { attr } => {
            let (rust_type, inner, plain, len) = gen_writable_type(attr);

            let inner = if !inner.is_empty() && !matches!(attr.r#type, AttrType::Flag) {
                quote!(self.as_rec_mut().extend(#inner);)
            } else {
                quote!()
            };

            impls.extend(quote! {
                pub fn entry(mut self, value: #rust_type) -> Self {
                    let index = self.counter;
                    self.counter += 1;
                    push_header(self.as_rec_mut(), index, #len as u16);
                    #inner
                    #plain
                    self
                }
            });
        }
        IndexedArrayType::Nest { nested_attributes } => {
            let next_type = writable_type(nested_attributes);

            impls.extend(quote! {
                pub fn entry_nested(mut self) -> #next_type<Self> {
                    let index = self.counter;
                    self.counter += 1;
                    let header_offset = push_nested_header(self.as_rec_mut(), index);
                    #next_type { prev: Some(self), header_offset: Some(header_offset) }
                }
            });
        }
        other => unreachable!("{other:?}"),
    };

    if ctx.generated_arrays.contains(&array_type.to_string()) {
        return array_type;
    }
    ctx.generated_arrays.insert(array_type.to_string());

    tokens.extend(quote! {
        pub struct #array_type<Prev: Rec> {
            pub(crate) prev: Option<Prev>,
            pub(crate) header_offset: Option<usize>,
            pub(crate) counter: u16,
        }
        impl<Prev: Rec> Rec for #array_type<Prev> {
            fn as_rec_mut(&mut self) -> &mut Vec<u8> {
                self.prev.as_mut().unwrap().as_rec_mut()
            }
        }
        impl<Prev: Rec> #array_type<Prev> {
            pub fn new(prev: Prev) -> Self {
                Self { prev: Some(prev), header_offset: None, counter: 0 }
            }

            pub fn end_array(mut self) -> Prev {
                let mut prev = self.prev.take().unwrap();
                if let Some(header_offset) = &self.header_offset {
                    finalize_nested_header(prev.as_rec_mut(), *header_offset);
                }
                prev
            }

            #impls
        }
        impl<Prev: Rec> Drop for #array_type<Prev> {
            fn drop(&mut self) {
                if let Some(prev) = &mut self.prev {
                    if let Some(header_offset) = &self.header_offset {
                        finalize_nested_header(prev.as_rec_mut(), *header_offset);
                    }
                }
            }
        }
    });

    array_type
}
