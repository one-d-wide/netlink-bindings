use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gen_ops::OpHeader,
    gen_utils::{kebab_to_rust, kebab_to_type, lifetime_needed_attrs},
    gen_writable::writable_type,
    parse_spec::{AttrSet, Spec},
    Context,
};

pub fn gen_request_struct(
    tokens: &mut TokenStream,
    _ctx: &mut Context,
    spec: &Spec,
    requests: &[(String, Option<OpHeader>)],
) {
    if spec.operations.list.is_empty() {
        return;
    }

    let name = format_ident!("Request");
    let mut op_funcs = TokenStream::new();
    for (op, header) in requests {
        let req = format_ident!("Request{}", kebab_to_type(op));
        let op = format_ident!("{}", kebab_to_rust(op));

        if let Some(header) = header.as_ref().filter(|h| h.construct_header.is_none()) {
            let header = writable_type(&header.name);
            op_funcs.extend(quote! {
                pub fn #op(self, header: &#header) -> #req<'buf> {
                    #req::new(self, header)
                }
            });
        } else {
            op_funcs.extend(quote! {
                pub fn #op(self) -> #req<'buf> {
                    #req::new(self)
                }
            });
        };
    }

    let name_buf = format_ident!("RequestBuf");
    tokens.extend(quote! {
        #[derive(Debug)]
        enum #name_buf<'buf> {
            Ref(&'buf mut Vec<u8>),
            Own(Vec<u8>)
        }

        #[derive(Debug)]
        pub struct #name<'buf> {
            buf: #name_buf<'buf>,
            flags: u16,
        }

        // TODO: looks like an overkill
        impl #name<'static> {
            pub fn new() -> Self {
                Self {
                    flags: 0,
                    buf: #name_buf::Own(Vec::new()),
                }
            }

            pub fn from_buf(buf: Vec<u8>) -> Self {
                Self {
                    flags: 0,
                    buf: #name_buf::Own(buf),
                }
            }

            pub fn into_buf(self) -> Vec<u8> {
                match self.buf {
                    #name_buf::Own(buf) => buf,
                    _ => unreachable!(),
                }
            }
        }

        impl<'buf> #name<'buf> {
            pub fn new_with_buf(buf: &'buf mut Vec<u8>) -> Self {
                buf.clear();
                Self {
                    flags: 0,
                    buf: #name_buf::Ref(buf),
                }
            }

            fn buf(&self) -> &Vec<u8> {
                match &self.buf {
                    #name_buf::Ref(buf) => buf,
                    #name_buf::Own(buf) => buf,
                }
            }

            fn buf_mut(&mut self) -> &mut Vec<u8> {
                match &mut self.buf {
                    #name_buf::Ref(buf) => buf,
                    #name_buf::Own(buf) => buf,
                }
            }

            #[doc = "Set [`libc::NLM_F_CREATE`] flag"]
            pub fn set_create(mut self) -> Self {
                self.flags |= consts::NLM_F_CREATE as u16;
                self
            }

            #[doc = "Set [`libc::NLM_F_EXCL`] flag"]
            pub fn set_excl(mut self) -> Self {
                self.flags |= consts::NLM_F_EXCL as u16;
                self
            }

            #[doc = "Set [`libc::NLM_F_REPLACE`] flag"]
            pub fn set_replace(mut self) -> Self {
                self.flags |= consts::NLM_F_REPLACE as u16;
                self
            }

            #[doc = "Set [`libc::NLM_F_CREATE`] and [`libc::NLM_F_REPLACE`] flag"]
            pub fn set_change(self) -> Self {
                self.set_create().set_replace()
            }

            #[doc = "Set [`libc::NLM_F_APPEND`] flag"]
            pub fn set_append(mut self) -> Self {
                self.flags |= consts::NLM_F_APPEND as u16;
                self
            }

            #[doc = "Set [`libc::NLM_F_DUMP`] flag"]
            fn set_dump(mut self) -> Self {
                self.flags |= consts::NLM_F_DUMP as u16;
                self
            }

            #op_funcs
        }
    });
}

#[allow(clippy::too_many_arguments)]
pub fn gen_request_wrapper(
    tokens: &mut TokenStream,
    _ctx: &mut Context,
    spec: &Spec,
    is_dump: bool,
    request_value: u16,
    _request_set: &AttrSet,
    reply_set: &AttrSet,
    request_name: &str,
    reply_name: &str,
    request_header: Option<&OpHeader>,
    reply_header: Option<&OpHeader>,
) {
    if spec.operations.list.is_empty() {
        return;
    }

    let encoder = writable_type(request_name);
    let request_decoder = format_ident!("{}", kebab_to_type(request_name));
    let decoder = format_ident!("{}", kebab_to_type(reply_name));
    let name = format_ident!("Request{}", kebab_to_type(request_name));

    let (new_encoder, write_header) = if request_header.is_some() {
        (
            quote!(new_without_header),
            quote!(#encoder::write_header(&mut request.buf_mut());),
        )
    } else {
        (quote!(new), quote!())
    };

    let decoder_lifetime = if lifetime_needed_attrs(reply_set) {
        quote!(<'buf>)
    } else {
        quote!()
    };

    let request = if is_dump {
        quote!(request.set_dump())
    } else {
        quote!(request)
    };

    let (reply_type, map_decoder, new) = if let Some(fixed_header) = reply_header {
        let header = writable_type(&fixed_header.name);
        if fixed_header.construct_header.is_some() {
            (
                quote!(Iterable<'buf, #decoder #decoder_lifetime>),
                quote!(),
                quote! {
                    pub fn new(mut request: Request<'r>) -> Self {
                        #encoder::write_header(&mut request.buf_mut());
                        Self { request: #request }
                    }
                },
            )
        } else {
            (
                quote!((#header, Iterable<'buf, #decoder #decoder_lifetime>)),
                quote!(.1),
                quote! {
                    pub fn new(mut request: Request<'r>, header: &#header) -> Self {
                        #encoder::write_header(&mut request.buf_mut(), header);
                        Self { request: #request }
                    }
                },
            )
        }
    } else {
        (
            quote!(Iterable<'buf, #decoder #decoder_lifetime>),
            quote!(),
            quote! {
                pub fn new(mut request: Request<'r>) -> Self {
                    #write_header
                    Self { request: #request }
                }
            },
        )
    };

    let proto = if let Some(protonum) = spec.protonum {
        quote!(Protocol::Raw { protonum: #protonum, request_type: #request_value })
    } else if spec.name == "nlctrl" {
        // Generic control socket is special
        quote!(Protocol::Raw {
            protonum: 0x10,
            request_type: 0x10
        })
    } else if spec.protocol.as_ref().unwrap().starts_with("genetlink") {
        let proto = &spec.name;
        quote!(Protocol::Generic(#proto.as_bytes()))
    } else {
        panic!("The protocol is not genetlink and the protonum isn't specified")
    };

    tokens.extend(quote! {
        #[derive(Debug)]
        pub struct #name<'r> {
            request: Request<'r>,
        }

        impl<'r> #name<'r> {
            #new
            pub fn encode(&mut self) -> #encoder<&mut Vec<u8>> {
                #encoder::#new_encoder(self.request.buf_mut())
            }
        }

        impl NetlinkRequest for #name<'_> {
            type ReplyType<'buf> = #reply_type;

            fn protocol(&self) -> Protocol {
                #proto
            }
            fn flags(&self) -> u16 {
                self.request.flags
            }
            fn payload(&self) -> &[u8] {
                self.request.buf()
            }

            fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
                #decoder::new(buf)
            }

            fn lookup(
                buf: &[u8],
                offset: usize,
                missing_type: Option<u16>,
            ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
                #request_decoder::new(buf)#map_decoder.lookup_attr(offset, missing_type)
            }
        }
    });
}
