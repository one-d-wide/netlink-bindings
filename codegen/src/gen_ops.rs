use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    gen_attrs::gen_attrset,
    gen_request_impl,
    gen_writable::gen_writable_attrset,
    parse_spec::{AttrProp, AttrType, Operation, OperationSpec, Request, Spec},
    Context,
};

pub fn request_kebab_name(type_name: &str, op_name: &str) -> String {
    format!("op-{type_name}-{op_name}-request")
}

pub fn reply_kebab_name(type_name: &str, op_name: &str) -> String {
    format!("op-{type_name}-{op_name}-reply")
}

pub fn gen_ops(spec: &Spec, ctx: &mut Context) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    let mut request_names = Vec::new();

    for op in &spec.operations.list {
        gen_op(&mut tokens, spec, ctx, op, &mut request_names);
    }

    gen_request_impl::gen_request_struct(&mut tokens, ctx, spec, &request_names);

    tokens
}

pub struct OpHeader {
    pub name: String,
    #[allow(clippy::type_complexity)]
    pub construct_header: Option<Box<dyn Fn(&Ident) -> TokenStream>>,
}

// TODO: enum model
fn whitelist_op_attrs(attrs: &mut Vec<AttrProp>, allowed: &[String]) {
    // request_attrs
    //     .attributes
    //     .retain(|a| op.request.attributes.contains(&a.name));
    for attr in attrs {
        if !allowed.contains(&attr.name) {
            attr.r#type = AttrType::Unused;
        }
    }
}

pub fn get_value(
    ops: &OperationSpec,
    op: &Request,
    do_op: Option<&Request>,
    do_op_other: Option<&Request>,
) -> u16 {
    let value = None
        .or(op.value.as_ref())
        .or(ops.value.as_ref())
        .or_else(|| do_op.as_ref().unwrap().value.as_ref())
        .or_else(|| do_op_other.as_ref().unwrap().value.as_ref())
        .unwrap_or_else(|| {
            panic!(
                "Operation {:?} carries fixed-header but doesn't specify value",
                ops.name
            )
        });
    let value: u16 = if let Some(value) = value.strip_prefix("0x") {
        u16::from_str_radix(value, 16).unwrap()
    } else {
        value.parse().unwrap()
    };
    value
}

pub fn gen_op(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    ops: &OperationSpec,
    request_names: &mut Vec<(String, Option<OpHeader>)>,
) {
    let Some(attrset) = &ops.attribute_set else {
        return;
    };
    let attrs = spec.find_attr(attrset);

    let fixed_header =
        |op: &Request, do_op: Option<&Request>| match (&ops.fixed_header, spec.protocol.as_deref())
        {
            (Some(h), _) => Some(OpHeader {
                name: h.clone(),
                construct_header: None,
            }),
            (None, Some("genetlink" | "genetlink-legacy")) => {
                let construct_header = {
                    let value = get_value(ops, op, do_op, None) as u8;
                    // The expected use for genlmsghdr.version was to allow versioning of the
                    // APIs provided by the subsystems. No subsystem to date made significant use
                    // of this field, so setting it to 1 seems like a safe bet.
                    // From: https://docs.kernel.org/userspace-api/netlink/intro.html#generic-netlink
                    let version: u8 = 1;
                    Box::new(move |header: &Ident| {
                        quote! {
                            #header.set_cmd(#value);
                            #header.set_version(#version);
                        }
                    }) as Box<dyn Fn(&Ident) -> TokenStream>
                };

                Some(OpHeader {
                    name: "builtin-nfgenmsg".into(),
                    construct_header: Some(construct_header),
                })
            }
            _ => None,
        };

    let doc = ops
        .doc
        .as_ref()
        .map(|doc| quote!(#[doc = #doc]))
        .unwrap_or_default();

    let mut generate = |op_name: &str, op: &Operation| {
        let mut request_attrs = attrs.clone();
        if !spec.operations.all_attrs && !ops.all_attrs {
            whitelist_op_attrs(&mut request_attrs.attributes, &op.request.attributes);
        }
        let request_name = request_kebab_name(&ops.name, op_name);
        request_attrs.name = request_name.clone();
        let request_header = || fixed_header(&op.request, ops.r#do.as_ref().map(|op| &op.request));
        request_names.push((request_name.clone(), request_header()));
        let request_header = request_header();

        tokens.extend(doc.clone());
        gen_writable_attrset(tokens, ctx, spec, &request_attrs, request_header.as_ref());

        tokens.extend(doc.clone());
        gen_attrset(
            tokens,
            spec,
            ctx,
            &request_attrs,
            request_header.as_ref(),
            Some(attrs),
        );

        let reply_header = fixed_header(&op.reply, ops.r#do.as_ref().map(|op| &op.reply));
        let mut reply_attrs = attrs.clone();
        if !spec.operations.all_attrs && !ops.all_attrs {
            whitelist_op_attrs(&mut reply_attrs.attributes, &op.reply.attributes);
        }
        let reply_name = reply_kebab_name(&ops.name, op_name);
        reply_attrs.name = reply_name.clone();

        tokens.extend(doc.clone());
        gen_writable_attrset(tokens, ctx, spec, &reply_attrs, reply_header.as_ref());

        tokens.extend(doc.clone());
        gen_attrset(
            tokens,
            spec,
            ctx,
            &reply_attrs,
            reply_header.as_ref(),
            Some(attrs),
        );

        gen_request_impl::gen_request_wrapper(
            tokens,
            ctx,
            spec,
            op_name == "dump",
            get_value(
                ops,
                &op.request,
                ops.r#do.as_ref().map(|op| &op.request),
                None,
            ),
            &request_attrs,
            &reply_attrs,
            &request_name,
            &reply_name,
            request_header.as_ref(),
            reply_header.as_ref(),
        );
    };

    if let Some(dump) = &ops.dump {
        generate("dump", dump);
    }

    if let Some(r#do) = &ops.r#do {
        generate("do", r#do);
    }
}
