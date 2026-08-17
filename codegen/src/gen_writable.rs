use std::{collections::HashSet, ffi::CString};

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::{
    gen_cstruct::{gen_struct_len, struct_type},
    gen_ops::{gen_op_header, OpHeader},
    gen_sub_message::{self, SelectorType},
    gen_utils::{doc_attr, kebab_to_rust, kebab_to_type},
    parse_spec::{AttrProp, AttrSet, AttrType, ByteOrder, IndexedArrayType, Spec},
    Context,
};

pub fn writable_type(name: &str) -> Ident {
    format_ident!("Push{}", kebab_to_type(name))
}

pub fn writable_val_attr(name: &str) -> Ident {
    format_ident!("push_{}", kebab_to_rust(&name))
}

pub fn writable_array_attr(name: &str) -> Ident {
    format_ident!("array_{}", kebab_to_rust(&name))
}

pub fn writable_nested_attr(name: &str) -> Ident {
    format_ident!("nested_{}", kebab_to_rust(&name))
}

pub fn writable_func(spec: &Spec, set: &AttrSet, next: &AttrProp) -> Vec<Ident> {
    let res = match &next.r#type {
        AttrType::Unused => return Vec::new(),
        AttrType::SubMessage {
            sub_message,
            selector,
        } => {
            if !gen_sub_message::is_supported(set, next, selector) {
                return Vec::new();
            }

            let sub = spec.find_sub_message(sub_message);

            let mut res = Vec::new();
            for sub_attr in &sub.formats {
                res.push(gen_sub_message::sub_message_push_name(
                    &next.name,
                    &sub_attr.value,
                ));
            }
            return res;
        }
        AttrType::IndexedArray { .. } => writable_array_attr(&next.name),
        AttrType::Nest { .. } => writable_nested_attr(&next.name),
        _ => writable_val_attr(&next.name),
    };

    vec![res]
}

pub fn writable_array_type(spec: &Spec, attr: &IndexedArrayType) -> Ident {
    let rust_type = match attr {
        IndexedArrayType::Plain { attr } => match &attr.r#type {
            AttrType::U32 => "U32".into(),
            AttrType::Binary {
                r#struct: Some(r#struct),
                ..
            } => struct_type(spec, r#struct).to_string(),
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
        gen_writable_attrset(&mut tokens, ctx, spec, set, None, false);
    }

    tokens
}

pub fn gen_writable_attrset(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    spec: &Spec,
    set: &AttrSet,
    fixed_header: Option<&OpHeader>,
    needs_value: bool,
) {
    let type_name = writable_type(&set.name);

    let end = format_ident!("end_nested");

    tokens.extend(quote! {
        pub struct #type_name<Prev: Pusher> {
            pub(crate) prev: Option<Prev>,
            pub(crate) header_offset: Option<usize>,
        }
    });

    tokens.extend(quote! {
        impl<Prev: Pusher> Pusher for #type_name<Prev> {
            fn as_vec_mut(&mut self) -> &mut Vec<u8> {
                self.prev.as_mut().unwrap().as_vec_mut()
            }
            fn as_vec(&self) -> &Vec<u8> {
                self.prev.as_ref().unwrap().as_vec()
            }
        }
    });

    let mut new_args = quote!();
    let mut header_args = quote!();
    let mut request_type_ident = None;
    if needs_value && spec.is_genetlink() {
        new_args = quote!(, request_type: u8);
        header_args = quote!(, request_type);
        request_type_ident = Some(format_ident!("request_type").into_token_stream());
    }

    let new_impl = if let Some(fixed_header) = fixed_header {
        let header = struct_type(spec, &fixed_header.name);
        let header_var = format_ident!("header");
        if let Some(fill) = &fixed_header.op_header_value {
            let fill = gen_op_header(fill, &header_var, request_type_ident.as_ref());
            quote! {
                pub fn new(mut prev: Prev #new_args) -> Self {
                    Self::write_header(&mut prev #header_args);
                    Self::new_without_header(prev)
                }
                fn new_without_header(prev: Prev) -> Self {
                    Self { prev: Some(prev), header_offset: None }
                }
                fn write_header(prev: &mut Prev #new_args) {
                    let mut #header_var = #header::new();
                    #fill
                    prev.as_vec_mut().extend(#header_var.as_slice());
                }
            }
        } else {
            quote! {
                pub fn new(mut prev: Prev, #header_var: &#header) -> Self {
                    Self::write_header(&mut prev, #header_var);
                    Self::new_without_header(prev)
                }
                fn new_without_header(prev: Prev) -> Self {
                    Self { prev: Some(prev), header_offset: None }
                }
                fn write_header(prev: &mut Prev, #header_var: &#header) {
                    prev.as_vec_mut().extend(#header_var.as_slice());
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
                finalize_nested_header(prev.as_vec_mut(), *header_offset);
            }
            prev
        }
    });

    let mut visited = HashSet::new();
    let mut id: u16 = 0;
    for next in &set.attributes {
        id += 1;
        if let Some(val) = &next.value {
            id = *val;
        }

        if visited.contains(&next.name) {
            continue;
        }
        visited.insert(next.name.clone());

        if let AttrType::Unused = &next.r#type {
            continue;
        }

        doc_attr(ctx, next, |doc| impls.extend(quote!(#[doc = #doc])));

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
                    header_type = Some(struct_type(spec, fixed_header).into_token_stream());
                }

                let mut attrs_type = None;
                if let Some(attrs_name) = &sub_attr.attribute_set {
                    attrs_type = Some(writable_type(attrs_name).into_token_stream());
                }

                let func = gen_sub_message::sub_message_push_name(&next.name, &sub_attr.value);
                let push_selector = writable_val_attr(selector);

                let doc_selector =
                    format!("Selector attribute `{selector}` is inserted automatically.");
                impls.extend(quote! {
                    #[doc = #doc_selector]
                    #[doc = "At most one sub-message attribute is expected per attribute set."]
                });

                match (header_type, attrs_type) {
                    (Some(h), None) => {
                        impls.extend(quote! {
                            pub fn #func(mut self, fixed_header: &#h) -> Self {
                                self = self.#push_selector(#sel_val);
                                self.header_offset = Some(push_nested_header(self.as_vec_mut(), #id));
                                self.as_vec_mut().extend(fixed_header.as_slice());
                                self
                            }
                        });
                    }
                    (None, Some(nested_type)) => {
                        impls.extend(quote! {
                            pub fn #func(mut self) -> #nested_type<PushDummy<Prev>> {
                                self = self.#push_selector(#sel_val);
                                let new_header_offset = push_nested_header(self.as_vec_mut(), #id);

                                // Pushing attributes after the sub message is a common pitfall
                                let dummy = PushDummy {
                                    prev: self.prev.take(),
                                    header_offset: self.header_offset.take(),
                                };

                                #nested_type { prev: Some(dummy), header_offset: Some(new_header_offset) }
                            }
                        });
                    }
                    (Some(h), Some(nested_type)) => {
                        impls.extend(quote! {
                            pub fn #func(mut self, fixed_header: &#h) -> #nested_type<PushDummy<Prev>> {
                                self = self.#push_selector(#sel_val);
                                let new_header_offset = push_nested_header(self.as_vec_mut(), #id);
                                self.as_vec_mut().extend(fixed_header.as_slice());

                                // Pushing attributes after the sub message is a common pitfall
                                let dummy = PushDummy {
                                    prev: self.prev.take(),
                                    header_offset: self.header_offset.take(),
                                };

                                #nested_type { prev: Some(dummy), header_offset: Some(new_header_offset) }
                            }
                        });
                    }
                    (None, None) => {
                        impls.extend(quote! {
                            pub fn #func(mut self) -> Self {
                                self = self.#push_selector(#sel_val);
                                self.header_offset = Some(push_nested_header(self.as_vec_mut(), #id));
                                self
                            }
                        });
                    }
                };
            }

            continue;
        }

        if let AttrType::IndexedArray { sub_type } = &next.r#type {
            let func = writable_array_attr(&next.name);

            let array_type = gen_writable_index_array(tokens, ctx, spec, sub_type);

            impls.extend(quote! {
                pub fn #func(mut self) -> #array_type<Self> {
                    let header_offset = push_nested_header(self.as_vec_mut(), #id);

                    #array_type { prev: Some(self), header_offset: Some(header_offset), counter: 0 }
                }
            });

            continue;
        }

        if let AttrType::Nest { nested_attributes } = &next.r#type {
            let nested_type = writable_type(nested_attributes);
            let func = writable_nested_attr(&next.name);

            impls.extend(quote! {
                pub fn #func(mut self) -> #nested_type<Self> {
                    let header_offset = push_nested_header(self.as_vec_mut(), #id);

                    #nested_type { prev: Some(self), header_offset: Some(header_offset) }
                }
            });

            continue;
        }

        let func = writable_val_attr(&next.name);
        let value_name = format_ident!("value");

        let WritableType {
            rust_type,
            encode_expr: inner,
            encode_block: plain,
            size_expr: len,
            // align,
            ..
        } = gen_writable_type(spec, next);

        let do_align = quote!();
        // TODO: is it realy needed? looks like kernel is always read u64 as unaligned anyways,
        // but what about structs?
        // let do_align = crate::gen_cstruct::gen_push_align(spec, set, next, align);

        let inner = if !inner.is_empty() && !matches!(next.r#type, AttrType::Flag) {
            quote!(self.as_vec_mut().extend(#inner);)
        } else {
            quote!()
        };

        impls.extend(quote! {
            pub fn #func(mut self, #value_name: #rust_type) -> Self {
                #do_align
                push_header(self.as_vec_mut(), #id, #len as u16);
                #inner
                #plain
                self
            }
        });

        if let AttrType::String = &next.r#type {
            // Convince method to use allow &[u8] instead of &CStr
            let func_bytes = format_ident!("{func}_bytes");
            doc_attr(ctx, next, |doc| impls.extend(quote!(#[doc = #doc])));
            impls.extend(quote! {
                pub fn #func_bytes(mut self, #value_name: &[u8]) -> Self {
                    push_header(self.as_vec_mut(), #id, (#value_name.len() + 1) as u16);
                    self.as_vec_mut().extend(#value_name);
                    self.as_vec_mut().push(0);
                    self
                }
            });
        }
    }

    tokens.extend(quote! {
        impl<Prev: Pusher> #type_name<Prev> {
            #impls
        }
        impl<Prev: Pusher> Drop for #type_name<Prev> {
            fn drop(&mut self) {
                if let Some(prev) = &mut self.prev {
                    if let Some(header_offset) = &self.header_offset {
                        finalize_nested_header(prev.as_vec_mut(), *header_offset);
                    }
                }
            }
        }
    });
}

pub struct WritableType {
    pub rust_type: TokenStream,
    pub encode_expr: TokenStream,
    pub encode_block: TokenStream,
    pub size_expr: TokenStream,
    #[allow(unused)]
    pub align: usize,
}

pub fn gen_writable_type(spec: &Spec, next: &AttrProp) -> WritableType {
    let value_name = format_ident!("value");
    let ord = match next.byte_order {
        ByteOrder::Host => "ne",
        ByteOrder::Little => "le",
        ByteOrder::Big => "be",
    };
    let encode = format_ident!("to_{ord}_bytes");

    match &next.r#type {
        AttrType::Flag => WritableType {
            rust_type: quote!(()),
            encode_expr: quote!(()),
            encode_block: quote!(),
            size_expr: quote!(0),
            align: 0,
        },
        AttrType::U32 if next.is_ipv4() => WritableType {
            rust_type: quote!(std::net::Ipv4Addr),
            encode_expr: quote! { &#value_name.to_bits().#encode() },
            encode_block: quote!(),
            size_expr: quote!(4),
            align: 4,
        },
        AttrType::Binary { .. } if next.is_ipv6() => WritableType {
            rust_type: quote!(std::net::Ipv6Addr),
            encode_expr: quote! { &#value_name.to_bits().#encode() },
            encode_block: quote!(),
            size_expr: quote!(16),
            align: 4,
        },
        AttrType::Binary { .. } if next.is_ip() => WritableType {
            rust_type: quote!(std::net::IpAddr),
            encode_expr: quote!(),
            encode_block: quote! { encode_ip(self.as_vec_mut(), #value_name); },
            size_expr: quote! {{
                match &#value_name {
                    IpAddr::V4(_) => 4,
                    IpAddr::V6(_) => 16,
                }
            }},
            align: 4,
        },
        AttrType::Binary { .. } if next.is_sockaddr() => WritableType {
            rust_type: quote!(std::net::SocketAddr),
            encode_expr: quote!(),
            encode_block: quote! { encode_sockaddr(self.as_vec_mut(), #value_name); },
            size_expr: quote! {{
                match &#value_name {
                    SocketAddr::V4(_) => 16,
                    SocketAddr::V6(_) => 28,
                }
            }},
            align: 4,
        },
        AttrType::Binary {
            r#struct: Some(r#struct),
            ..
        } => {
            let rust_type = struct_type(spec, r#struct);
            WritableType {
                rust_type: quote!(#rust_type),
                encode_expr: quote!(#value_name.as_slice()),
                encode_block: quote!(),
                size_expr: quote!(#value_name.as_slice().len()),
                align: gen_struct_len(spec, r#struct).1,
            }
        }
        AttrType::String => WritableType {
            rust_type: quote!(&CStr),
            encode_expr: quote!(#value_name.to_bytes_with_nul()),
            encode_block: quote!(),
            size_expr: quote!(#value_name.to_bytes_with_nul().len()),
            align: 1,
        },
        AttrType::Pad { .. } | AttrType::Binary { .. } => WritableType {
            rust_type: quote!(&[u8]),
            encode_expr: quote!(#value_name),
            encode_block: quote!(),
            size_expr: quote!(#value_name.len()),
            align: 1,
        },
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
            WritableType {
                rust_type,
                encode_expr: quote!(#value_name.#encode()),
                encode_block: quote!(),
                size_expr: Literal::u16_unsuffixed(len as u16).into_token_stream(),
                align: len,
            }
        }
    }
}

pub fn gen_writable_index_array(
    tokens: &mut TokenStream,
    ctx: &mut Context,
    spec: &Spec,
    sub_type: &IndexedArrayType,
) -> Ident {
    let mut impls = TokenStream::new();
    let array_type = writable_array_type(spec, sub_type);

    match sub_type {
        IndexedArrayType::Plain { attr } => {
            let WritableType {
                rust_type,
                encode_expr: inner,
                encode_block: plain,
                size_expr: len,
                ..
            } = gen_writable_type(spec, attr);

            let inner = if !inner.is_empty() && !matches!(attr.r#type, AttrType::Flag) {
                quote!(self.as_vec_mut().extend(#inner);)
            } else {
                quote!()
            };

            impls.extend(quote! {
                pub fn entry(mut self, value: #rust_type) -> Self {
                    let index = self.counter;
                    self.counter += 1;
                    push_header(self.as_vec_mut(), index, #len as u16);
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
                    let header_offset = push_nested_header(self.as_vec_mut(), index);
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
        pub struct #array_type<Prev: Pusher> {
            pub(crate) prev: Option<Prev>,
            pub(crate) header_offset: Option<usize>,
            pub(crate) counter: u16,
        }
        impl<Prev: Pusher> Pusher for #array_type<Prev> {
            fn as_vec_mut(&mut self) -> &mut Vec<u8> {
                self.prev.as_mut().unwrap().as_vec_mut()
            }
            fn as_vec(&self) -> &Vec<u8> {
                self.prev.as_ref().unwrap().as_vec()
            }
        }
        impl<Prev: Pusher> #array_type<Prev> {
            pub fn new(prev: Prev) -> Self {
                Self { prev: Some(prev), header_offset: None, counter: 0 }
            }

            pub fn end_array(mut self) -> Prev {
                let mut prev = self.prev.take().unwrap();
                if let Some(header_offset) = &self.header_offset {
                    finalize_nested_header(prev.as_vec_mut(), *header_offset);
                }
                prev
            }

            #impls
        }
        impl<Prev: Pusher> Drop for #array_type<Prev> {
            fn drop(&mut self) {
                if let Some(prev) = &mut self.prev {
                    if let Some(header_offset) = &self.header_offset {
                        finalize_nested_header(prev.as_vec_mut(), *header_offset);
                    }
                }
            }
        }
    });

    array_type
}
