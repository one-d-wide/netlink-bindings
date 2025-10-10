use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    gen_ops::OpHeader,
    gen_utils::{kebab_to_rust, kebab_to_type, lifetime_needed_attrs},
    gen_writable::writable_type,
    parse_spec::{AttrSet, Spec},
    Context,
};

pub fn gen_request(
    tokens: &mut TokenStream,
    _ctx: &mut Context,
    spec: &Spec,
    requests: &[(String, Option<OpHeader>)],
) {
    if spec.operations.list.is_empty() {
        return;
    }

    if spec.protocol.as_ref().is_some_and(|s| s == "netlink-raw") {
        gen_request_chained(tokens);
    }

    let name = format_ident!("Request");
    let mut op_funcs = TokenStream::new();
    for (op_name, header) in requests {
        let req = format_ident!("Request{}", kebab_to_type(op_name));
        let op = format_ident!("{}", kebab_to_rust(op_name));

        if let Some(header) = header.as_ref().filter(|h| h.construct_header.is_none()) {
            let header = writable_type(&header.name);
            op_funcs.extend(quote! {
                pub fn #op(self, header: &#header) -> #req<'buf> {
                    let mut res = #req::new(self, header);
                    res.request.do_writeback(res.protocol(), #op_name, #req::lookup);
                    res
                }
            });
        } else {
            op_funcs.extend(quote! {
                pub fn #op(self) -> #req<'buf> {
                    let mut res = #req::new(self);
                    res.request.do_writeback(res.protocol(), #op_name, #req::lookup);
                    res
                }
            });
        };
    }

    tokens.extend(quote! {
        use crate::traits::LookupFn;
        use crate::utils::RequestBuf;

        #[derive(Debug)]
        pub struct #name<'buf> {
            buf: RequestBuf<'buf>,
            flags: u16,
            writeback: Option<&'buf mut Option<RequestInfo>>
        }

        #[allow(unused)]
        #[derive(Debug, Clone)]
        pub struct RequestInfo {
            protocol: Protocol,
            flags: u16,
            name: &'static str,
            lookup: LookupFn,
        }

        impl #name<'static> {
            pub fn new() -> Self {
                Self::new_from_buf(Vec::new())
            }

            pub fn new_from_buf(buf: Vec<u8>) -> Self {
                Self {
                    flags: 0,
                    buf: RequestBuf::Own(buf),
                    writeback: None,
                }
            }

            pub fn into_buf(self) -> Vec<u8> {
                match self.buf {
                    RequestBuf::Own(buf) => buf,
                    _ => unreachable!(),
                }
            }
        }

        impl<'buf> #name<'buf> {
            pub fn new_with_buf(buf: &'buf mut Vec<u8>) -> Self {
                buf.clear();
                Self::new_extend(buf)
            }

            pub fn new_extend(buf: &'buf mut Vec<u8>) -> Self {
                Self {
                    flags: 0,
                    buf: RequestBuf::Ref(buf),
                    writeback: None,
                }
            }

            fn do_writeback(&mut self, protocol: Protocol, name: &'static str, lookup: LookupFn) {
                let Some(writeback) = &mut self.writeback else { return };
                **writeback = Some(RequestInfo {
                    protocol,
                    flags: self.flags,
                    name,
                    lookup,
                })
            }

            pub fn buf(&self) -> &Vec<u8> {
                self.buf.buf()
            }

            pub fn buf_mut(&mut self) -> &mut Vec<u8> {
                self.buf.buf_mut()
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
            pub fn into_encoder(self) -> #encoder<RequestBuf<'r>> {
                #encoder::#new_encoder(self.request.buf)
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

pub fn gen_request_chained(tokens: &mut TokenStream) {
    tokens.extend(quote! {
        #[derive(Debug)]
        pub struct ChainedFinal<'a> {
            inner: Chained<'a>,
        }

        #[derive(Debug)]
        pub struct Chained<'a> {
            buf: RequestBuf<'a>,
            first_seq: u32,
            lookups: Vec<(&'static str, LookupFn)>,

            last_header_offset: usize,
            last_kind: Option<RequestInfo>,
        }

        impl<'a> ChainedFinal<'a> {
            pub fn into_chained(self) -> Chained<'a> {
                self.inner
            }

            pub fn buf(&self) -> &Vec<u8> {
                self.inner.buf()
            }

            pub fn buf_mut(&mut self) -> &mut Vec<u8> {
                self.inner.buf_mut()
            }

            fn get_index(&self, seq: u32) -> Option<u32> {
                let min = self.inner.first_seq;
                let max = min.wrapping_add(self.inner.lookups.len() as u32);
                return if min <= max {
                    (min..max).contains(&seq).then(|| seq - min)
                } else if min <= seq {
                    Some(seq - min)
                } else if seq < max {
                    Some(u32::MAX - min + seq)
                } else {
                    None
                }
            }
        }

        impl crate::traits::NetlinkChained for ChainedFinal<'_> {
            fn protonum(&self) -> u16 {
                PROTONUM
            }

            fn payload(&self) -> &[u8] {
                self.buf()
            }

            fn chain_len(&self) -> usize {
                self.inner.lookups.len()
            }

            fn get_index(&self, seq: u32) -> Option<usize> {
                self.get_index(seq).map(|n| n as usize)
            }

            fn name(&self, index: usize) -> &'static str {
                self.inner.lookups[index].0
            }

            fn lookup(&self, index: usize) -> LookupFn {
                self.inner.lookups[index].1
            }
        }

        impl Chained<'static> {
            pub fn new(first_seq: u32) -> Self {
                Self::new_from_buf(Vec::new(), first_seq)
            }

            pub fn new_from_buf(buf: Vec<u8>, first_seq: u32) -> Self {
                Self {
                    buf: RequestBuf::Own(buf),
                    first_seq,
                    lookups: Vec::new(),
                    last_header_offset: 0,
                    last_kind: None,
                }
            }

            pub fn into_buf(self) -> Vec<u8> {
                match self.buf {
                    RequestBuf::Own(buf) => buf,
                    _ => unreachable!(),
                }
            }
        }

        impl<'a> Chained<'a> {
            pub fn new_with_buf(buf: &'a mut Vec<u8>, first_seq: u32) -> Self {
                Self {
                    buf: RequestBuf::Ref(buf),
                    first_seq,
                    lookups: Vec::new(),
                    last_header_offset: 0,
                    last_kind: None,
                }
            }

            pub fn finalize(mut self) -> ChainedFinal<'a> {
                self.update_header();
                ChainedFinal { inner: self }
            }

            pub fn request(&mut self) -> Request<'_> {
                self.update_header();

                self.last_header_offset = self.buf().len();
                self.buf_mut().extend_from_slice(PushNlmsghdr::new().as_slice());

                let mut request = Request::new_extend(self.buf.buf_mut());

                self.last_kind = None;
                request.writeback = Some(&mut self.last_kind);

                request
            }

            pub fn buf(&self) -> &Vec<u8> {
                self.buf.buf()
            }

            pub fn buf_mut(&mut self) -> &mut Vec<u8> {
                self.buf.buf_mut()
            }

            fn update_header(&mut self) {
                let Some(RequestInfo { protocol, flags, name, lookup }) = self.last_kind else {
                    if !self.buf().is_empty() {
                        // Remove reserved space if request wasn't written
                        assert_eq!(self.last_header_offset + PushNlmsghdr::len(), self.buf().len());
                        self.buf.buf_mut().truncate(self.last_header_offset);
                    }
                    return;
                };

                let header_offset = self.last_header_offset;
                let request_type = match protocol {
                    Protocol::Raw { request_type, .. } => request_type,
                    Protocol::Generic(_) => unreachable!(),
                };

                let index = self.lookups.len();
                let seq = self.first_seq.wrapping_add(index as u32);
                self.lookups.push((name, lookup));

                let buf = self.buf_mut();
                align(buf);

                let mut header = PushNlmsghdr::new();
                header.set_len((buf.len() - header_offset) as u32);
                header.set_type(request_type);
                header.set_flags(flags | consts::NLM_F_REQUEST as u16 | consts::NLM_F_ACK as u16);
                header.set_seq(seq);

                buf[header_offset..(header_offset+16)].clone_from_slice(header.as_slice());
            }
        }
    });
}
