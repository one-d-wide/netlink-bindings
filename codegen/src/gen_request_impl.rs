use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};

use crate::{
    gen_iterable::{gen_decoder_new_impl, iterable_name, DecoderNewImpl},
    gen_ops::OpHeader,
    gen_utils::{kebab_to_rust, kebab_to_type},
    gen_writable::writable_type,
    parse_spec::{AttrSet, Spec},
    Context,
};

pub struct OpInfo {
    pub name: String,
    pub header: Option<OpHeader>,
    pub needs_value: bool,
}

pub fn gen_request(tokens: &mut TokenStream, _ctx: &mut Context, spec: &Spec, requests: &[OpInfo]) {
    if spec.operations.list.is_empty() && spec.operations.fallback_attrs.is_none() {
        return;
    }

    if spec.protocol.as_ref().is_some_and(|s| s == "netlink-raw") {
        gen_request_chained(tokens);
    }

    let name = format_ident!("Request");
    let mut op_funcs = TokenStream::new();
    for OpInfo {
        name,
        header,
        needs_value,
    } in requests
    {
        let req = format_ident!("Request{}", kebab_to_type(name));
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

        if let Some(header) = header.as_ref().filter(|h| h.construct_header.is_none()) {
            let header = writable_type(&header.name);
            op_args = quote!(#op_args, header: &#header);
            new_args = quote!(#new_args, header);
        };

        op_funcs.extend(quote! {
            pub fn #op(self #op_args) -> #req<'buf> {
                let mut res = #req::new(self #new_args);
                res.request.do_writeback(res.protocol(), #name, #req::lookup);
                res
            }
        });
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

            #[doc = "Set `NLM_F_DUMP` flag"]
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
    _reply_set: &AttrSet,
    request_name: &str,
    reply_name: &str,
    request_header: Option<&OpHeader>,
    reply_header: Option<&OpHeader>,
    needs_value: bool,
    transparent_attrs: Option<&str>,
) {
    if spec.operations.list.is_empty() && spec.operations.fallback_attrs.is_none() {
        return;
    }

    let name = format_ident!("Request{}", kebab_to_type(request_name));
    let (decoder, decoder_new);
    let (request_name, reply_name) = if let Some(attrs) = transparent_attrs {
        decoder = quote!(Self);
        decoder_new = quote!(decode);
        (attrs, attrs)
    } else {
        decoder = format_ident!("{}", kebab_to_type(reply_name)).into_token_stream();
        decoder_new = quote!(new);
        (request_name, reply_name)
    };
    let encoder = writable_type(request_name);
    let decoder_iter = iterable_name(reply_name);
    let request_decoder = format_ident!("{}", kebab_to_type(request_name));

    let mut new_args = quote!();
    let mut header_args = quote!();
    let mut store_request_type = quote!();
    let mut request_type_field = quote!();
    let mut request_value = quote!(#request_value);
    if needs_value {
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
    if is_dump {
        request = quote!(request.set_dump());
    };

    let mut write_header_impl = quote!();
    let mut decode_impl = quote!();
    let mut decode_reply = quote!(#decoder::#decoder_new);
    let mut decode_request = quote!(#request_decoder::#decoder_new);

    if transparent_attrs.is_some() {
        header_encoder = quote!(Self);
        encoder_new = quote!(new);
        if let Some(fixed_header) = request_header {
            let request_type_ident = format_ident!("request_type").to_token_stream();
            let header = writable_type(&fixed_header.name);
            let header_var = format_ident!("header");
            if let Some(fill) = &fixed_header.construct_header {
                let fill = fill(&header_var, needs_value.then_some(&request_type_ident));
                write_header_impl = quote! {
                    fn write_header<Prev: Rec>(prev: &mut Prev #new_args) {
                        let mut #header_var = #header::new();
                        #fill
                        prev.as_rec_mut().extend(#header_var.as_slice());
                    }
                };
            } else {
                write_header_impl = quote! {
                    fn write_header<Prev: Rec>(prev: &mut Prev, #header_var: &#header) {
                        prev.as_rec_mut().extend(#header_var.as_slice());
                    }
                };
            }
        };

        let attrs = transparent_attrs.unwrap();
        let attrs = spec.find_attr(attrs);
        let DecoderNewImpl {
            return_type, body, ..
        } = gen_decoder_new_impl(attrs, reply_header);

        decode_impl = quote! {
            fn decode<'a>(buf: &'a [u8]) -> #return_type {
                #body
            }
        };

        decode_reply = quote!(Self::decode);
        decode_request = quote!(Self::decode);
    }

    let (reply_type, map_decoder, new);
    if let Some(fixed_header) = request_header {
        let header = writable_type(&fixed_header.name);
        if fixed_header.construct_header.is_some() {
            reply_type = quote!(#decoder_iter<'buf>);
            map_decoder = quote!();
            new = quote! {
                pub fn new(mut request: Request<'r> #new_args) -> Self {
                    #header_encoder::write_header(&mut request.buf_mut() #header_args);
                    Self { request: #request #store_request_type }
                }
            };
        } else {
            reply_type = quote!((#header, #decoder_iter<'buf>));
            map_decoder = quote!(.1);
            new = quote! {
                pub fn new(mut request: Request<'r> #new_args, header: &#header) -> Self {
                    #header_encoder::write_header(&mut request.buf_mut(), header);
                    Self { request: #request #store_request_type }
                }
            };
        }
    } else {
        reply_type = quote!(#decoder_iter<'buf>);
        map_decoder = quote!();
        new = quote! {
            pub fn new(request: Request<'r> #new_args) -> Self {
                Self { request: #request #store_request_type }
            }
        };
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
            #write_header_impl
            #decode_impl
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
