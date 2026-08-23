use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::{
    gen_cstruct::struct_type,
    gen_iterable::{gen_decoder_new_impl, iterable_name, DecoderNewImpl},
    gen_ops::{gen_op_header, OpHeader},
    gen_utils::{kebab_to_rust, kebab_to_type},
    gen_writable::writable_type,
    parse_spec::Spec,
    Context,
};

#[derive(Hash, PartialEq)]
pub struct WrapperInfo<'a> {
    pub is_dump: bool,
    pub request_value: u16,
    pub request_name: String,
    pub reply_name: String,
    pub request_header: Option<OpHeader>,
    pub reply_header: Option<OpHeader>,
    pub needs_value: bool,
    pub transparent_attrs: Option<(&'a str, &'a str)>,
    pub op_info: OpInfo,
}

#[derive(Clone, Hash, PartialEq)]
pub struct OpInfo {
    pub name: String,
    pub header: Option<OpHeader>,
    pub needs_value: bool,
    pub no_ack: bool,
    pub doc: Option<String>,
}

pub fn gen_request(tokens: &mut TokenStream, _ctx: &mut Context, spec: &Spec, requests: &[OpInfo]) {
    if spec
        .operations
        .list
        .iter()
        .all(|ops| ops.r#do.is_none() && ops.dump.is_none())
        && spec.operations.fallback_attrs.is_none()
    {
        return;
    }

    if spec.protocol.as_ref().is_some_and(|s| s == "netlink-raw") {
        gen_request_chained(tokens, requests);
    }

    let name = format_ident!("Request");
    let mut op_funcs = TokenStream::new();
    for OpInfo {
        name,
        header,
        needs_value,
        doc,
        ..
    } in requests
    {
        let req = format_ident!("{}", kebab_to_type(name));
        let op = format_ident!("{}", kebab_to_rust(name));

        let mut op_args = quote!();
        let mut new_args = quote!();

        if *needs_value {
            let request_value_type = if spec.is_genetlink() {
                quote!(u8)
            } else {
                quote!(u16)
            };
            op_args = quote!(, request_type: #request_value_type);
            new_args = quote!(, request_type);
        }

        if let Some(header) = header.as_ref().filter(|h| h.op_header_value.is_none()) {
            let header = struct_type(spec, &header.name);
            op_args = quote!(#op_args, header: &#header);
            new_args = quote!(#new_args, header);
        };

        if let Some(doc) = doc {
            op_funcs.extend(quote! {
                #[doc = #doc]
            });
        }

        op_funcs.extend(quote! {
            pub fn #op(self #op_args) -> #req<'buf> {
                let mut res = #req::new(self #new_args);
                res.request.do_writeback(res.protocol(), #name, #req::lookup);
                res
            }
        });
    }

    let mut set_dump = quote!();
    if spec.operations.list.iter().any(|ops| ops.dump.is_some()) {
        set_dump = quote! {
            #[doc = "Set `NLM_F_DUMP` flag"]
            fn set_dump(mut self) -> Self {
                self.flags |= consts::NLM_F_DUMP as u16;
                self
            }
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

            pub fn new_from_buf(mut buf: Vec<u8>) -> Self {
                buf.clear();
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

            #[doc = "Set `NLM_F_CREATE` flag"]
            pub fn set_create(mut self) -> Self {
                self.flags |= consts::NLM_F_CREATE as u16;
                self
            }

            #[doc = "Set `NLM_F_EXCL` flag"]
            pub fn set_excl(mut self) -> Self {
                self.flags |= consts::NLM_F_EXCL as u16;
                self
            }

            #[doc = "Set `NLM_F_REPLACE` flag"]
            pub fn set_replace(mut self) -> Self {
                self.flags |= consts::NLM_F_REPLACE as u16;
                self
            }

            #[doc = "Set `NLM_F_CREATE` and `NLM_F_REPLACE` flag"]
            pub fn set_change(self) -> Self {
                self.set_create().set_replace()
            }

            #[doc = "Set `NLM_F_APPEND` flag"]
            pub fn set_append(mut self) -> Self {
                self.flags |= consts::NLM_F_APPEND as u16;
                self
            }

            #[doc = "Set `self.flags |= flags`"]
            pub fn set_flags(mut self, flags: u16) -> Self {
                self.flags |= flags;
                self
            }

            #[doc = "Set `self.flags ^= self.flags & flags`"]
            pub fn unset_flags(mut self, flags: u16) -> Self {
                self.flags ^= self.flags & flags;
                self
            }

            #set_dump

            #op_funcs
        }
    });
}

#[allow(clippy::too_many_arguments)]
pub fn gen_request_wrapper(
    tokens: &mut TokenStream,
    _ctx: &mut Context,
    spec: &Spec,
    info: &WrapperInfo,
) {
    if spec.operations.list.is_empty() && spec.operations.fallback_attrs.is_none() {
        return;
    }

    let WrapperInfo {
        is_dump,
        request_value,
        request_name,
        reply_name,
        request_header,
        reply_header,
        needs_value,
        transparent_attrs,
        op_info,
    } = info;

    let mut op_info_doc = quote!();
    if let Some(doc) = &op_info.doc {
        op_info_doc = quote!(#[doc = #doc]);
    }
    let name = format_ident!("{}", kebab_to_type(request_name));

    let (reply_decoder, decoder_new);
    let (request_name, reply_name) = if let Some(transparent_attrs) = transparent_attrs {
        reply_decoder = quote!(Self);
        decoder_new = quote!(decode);
        *transparent_attrs
    } else {
        reply_decoder = format_ident!("{}", kebab_to_type(reply_name)).into_token_stream();
        decoder_new = quote!(new);
        (request_name.as_str(), reply_name.as_str())
    };
    let encoder = writable_type(request_name);
    let decoder_iter = iterable_name(reply_name);
    let encoder_iter = iterable_name(request_name);
    let request_decoder = format_ident!("{}", kebab_to_type(request_name));

    let mut new_args = quote!();
    let mut header_args = quote!();
    let mut store_request_type = quote!();
    let mut request_type_field = quote!();
    let mut request_value = quote!(#request_value);
    if *needs_value {
        let request_value_type = if spec.is_genetlink() {
            quote!(u8)
        } else {
            quote!(u16)
        };
        new_args = quote!(#new_args, request_type: #request_value_type);
        header_args = quote!(#header_args, request_type);

        if !spec.is_genetlink() {
            request_type_field = quote!(request_type: #request_value_type,);
            store_request_type = quote!(, request_type);
            request_value = quote!(self.request_type);
        }
    }

    let mut encoder_new = quote!(new);
    if request_header.is_some() {
        encoder_new = quote!(new_without_header);
    };

    let mut header_encoder = quote!(#encoder);
    let mut request = quote!(request);
    if *is_dump {
        request = quote!(request.set_dump());
    };

    let mut write_header_impl = quote!();
    let mut decode_impl = quote!();
    let mut decode_reply = quote!(#reply_decoder::#decoder_new);
    let mut decode_request = quote!(#request_decoder::#decoder_new);

    if let Some((transparent_request_attrs, transparent_reply_attrs)) = transparent_attrs {
        header_encoder = quote!(Self);
        encoder_new = quote!(new);
        if let Some(fixed_header) = request_header {
            let request_type_ident = format_ident!("request_type").to_token_stream();
            let header = struct_type(spec, &fixed_header.name);
            let header_var = format_ident!("header");
            if let Some(fill) = &fixed_header.op_header_value {
                let fill = gen_op_header(
                    fill,
                    &header_var,
                    needs_value.then_some(&request_type_ident),
                );
                write_header_impl = quote! {
                    fn write_header<Prev: Pusher>(prev: &mut Prev #new_args) {
                        let mut #header_var = #header::new();
                        #fill
                        prev.as_vec_mut().extend(#header_var.as_slice());
                    }
                };
            } else {
                write_header_impl = quote! {
                    fn write_header<Prev: Pusher>(prev: &mut Prev, #header_var: &#header) {
                        prev.as_vec_mut().extend(#header_var.as_slice());
                    }
                };
            }
        };

        let reply_attrs = spec.find_attr(transparent_reply_attrs);
        let DecoderNewImpl {
            return_type: reply_return_typen,
            body: reply_body,
            ..
        } = gen_decoder_new_impl(spec, reply_attrs, reply_header.as_ref());

        let request_attrs = spec.find_attr(transparent_request_attrs);
        let DecoderNewImpl {
            return_type: request_return_typen,
            body: request_body,
            ..
        } = gen_decoder_new_impl(spec, request_attrs, request_header.as_ref());

        if transparent_reply_attrs == transparent_request_attrs
            && Option::zip(request_header.as_ref(), reply_header.as_ref())
                .is_some_and(|(l, r)| l.name == r.name)
        {
            decode_impl = quote! {
                pub fn decode_request<'a>(buf: &'a [u8]) -> #request_return_typen {
                    #request_body
                }
            };

            decode_request = quote!(Self::decode_request);
            decode_reply = quote!(Self::decode_request);
        } else {
            decode_impl = quote! {
                pub fn decode_request<'a>(buf: &'a [u8]) -> #request_return_typen {
                    #request_body
                }
                fn decode_reply<'a>(buf: &'a [u8]) -> #reply_return_typen {
                    #reply_body
                }
            };

            decode_request = quote!(Self::decode_request);
            decode_reply = quote!(Self::decode_reply);
        }
    }

    let (request_type, reply_type, map_decoder, new);
    if let Some(request_header) = request_header {
        if request_header.op_header_value.is_some() {
            reply_type = quote!(#decoder_iter<'buf>);
            request_type = quote!(#encoder_iter<'buf>);
            map_decoder = quote!();
            new = quote! {
                pub fn new(mut request: Request<'r> #new_args) -> Self {
                    #header_encoder::write_header(request.buf_mut() #header_args);
                    Self { request: #request #store_request_type }
                }
                pub fn encode_request<'buf>(buf: &'buf mut Vec<u8> #new_args) -> #encoder<&'buf mut Vec<u8>> {
                    #header_encoder::write_header(buf #header_args);
                    #encoder::#encoder_new(buf)
                }
            };
        } else {
            let request_header = struct_type(spec, &request_header.name);
            let reply_header = struct_type(spec, &reply_header.as_ref().unwrap().name);
            reply_type = quote!((#reply_header, #decoder_iter<'buf>));
            request_type = quote!((#request_header, #encoder_iter<'buf>));
            map_decoder = quote!(.1);
            new = quote! {
                pub fn new(mut request: Request<'r> #new_args, header: &#request_header) -> Self {
                    #header_encoder::write_header(request.buf_mut(), header);
                    Self { request: #request #store_request_type }
                }
                pub fn encode_request<'buf>(buf: &'buf mut Vec<u8> #new_args, header: &#request_header) -> #encoder<&'buf mut Vec<u8>> {
                    #header_encoder::write_header(buf, header);
                    #encoder::#encoder_new(buf)
                }
            };
        }
    } else {
        reply_type = quote!(#decoder_iter<'buf>);
        request_type = quote!(#encoder_iter<'buf>);
        map_decoder = quote!();
        new = quote! {
            pub fn new(request: Request<'r> #new_args) -> Self {
                Self { request: #request #store_request_type }
            }
            pub fn encode_request<'buf>(buf: &'buf mut Vec<u8> #new_args) -> #encoder<&'buf mut Vec<u8>> {
                #encoder::#encoder_new(buf)
            }
        };
    };

    let proto = if spec.name == "nlctrl" {
        // Generic control socket is special
        quote!(Protocol::Raw {
            protonum: 0x10,
            request_type: 0x10
        })
    } else if spec.protocol.as_ref().unwrap().starts_with("genetlink") {
        let proto = &spec.name;
        quote!(Protocol::Generic(#proto.as_bytes()))
    } else if let Some(protonum) = spec.protonum {
        quote!(Protocol::Raw { protonum: #protonum, request_type: #request_value })
    } else {
        panic!("The protocol is not genetlink and the protonum isn't specified")
    };

    if decode_impl.is_empty() {
        decode_impl = quote! {
            pub fn decode_request<'buf>(buf: &'buf [u8]) -> #request_type {
                #decode_request(buf)
            }
        };
    }

    tokens.extend(quote! {
        #op_info_doc
        #[derive(Debug)]
        pub struct #name<'r> {
            request: Request<'r>,
            #request_type_field
        }

        impl<'r> #name<'r> {
            #new
            pub fn encode(&mut self) -> #encoder<&mut Vec<u8>> {
                #encoder::#encoder_new(self.request.buf_mut())
            }
            pub fn into_encoder(self) -> #encoder<RequestBuf<'r>> {
                #encoder::#encoder_new(self.request.buf)
            }
            #decode_impl
            #write_header_impl
        }

        impl NetlinkRequest for #name<'_> {
            fn protocol(&self) -> Protocol {
                #proto
            }
            fn flags(&self) -> u16 {
                self.request.flags
            }
            fn payload(&self) -> &[u8] {
                self.request.buf()
            }

            // type RequestType<'buf> = #request_type;
            // fn decode_request<'buf>(buf: &'buf [u8]) -> Self::RequestType<'buf> {
            //     #decode_request(buf)
            // }

            type ReplyType<'buf> = #reply_type;
            fn decode_reply<'buf>(buf: &'buf [u8]) -> Self::ReplyType<'buf> {
                #decode_reply(buf)
            }

            fn lookup(
                buf: &[u8],
                offset: usize,
                missing_type: Option<u16>,
            ) -> (Vec<(&'static str, usize)>, Option<&'static str>) {
                #decode_request(buf)#map_decoder.lookup_attr(offset, missing_type)
            }
        }
    });
}

pub fn gen_request_chained(tokens: &mut TokenStream, requests: &[OpInfo]) {
    let mut supports_ack = quote!();
    if requests.iter().any(|r| r.no_ack) {
        let no_ack_requests = requests
            .iter()
            .filter(|r| r.no_ack)
            .map(|r| format_ident!("{}", kebab_to_type(&r.name)))
            .fold(quote!(), |acc, r| {
                quote! {
                    #acc
                    f if f == #r::lookup as *const LookupFn => false,
                }
            });

        supports_ack = quote! {
            fn supports_ack(&self, index: usize) -> Option<bool> {
                Some(match self.inner.lookups.get(index)?.1 as *const LookupFn {
                    #no_ack_requests
                    _ => true,
                })
            }
        };
    }

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

            #supports_ack
        }

        impl Chained<'static> {
            pub fn new(first_seq: u32) -> Self {
                Self::new_from_buf(Vec::new(), first_seq)
            }

            pub fn new_from_buf(mut buf: Vec<u8>, first_seq: u32) -> Self {
                buf.clear();
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
                buf.clear();
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
                self.buf_mut().extend_from_slice(Nlmsghdr::new().as_slice());

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
                        assert_eq!(self.last_header_offset + Nlmsghdr::len(), self.buf().len());
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

                let header = Nlmsghdr {
                    len: (buf.len() - header_offset) as u32,
                    r#type: request_type,
                    flags: flags | consts::NLM_F_REQUEST as u16 | consts::NLM_F_ACK as u16,
                    seq,
                    pid: 0,
                };

                buf[header_offset..(header_offset+16)].clone_from_slice(header.as_slice());
            }
        }
    });
}
