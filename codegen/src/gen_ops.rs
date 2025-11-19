use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    gen_attrs::gen_attrset,
    gen_request_impl::{self, OpInfo},
    gen_writable::gen_writable_attrset,
    parse_spec::{AttrProp, AttrSet, AttrType, Operation, OperationSpec, Request, Spec},
    Context, WARNING,
};

pub fn request_kebab_name(type_name: &str, op_name: &str) -> String {
    let delim = type_name.is_empty().then_some("").unwrap_or("-");
    format!("op{delim}{type_name}-{op_name}-request")
}

pub fn reply_kebab_name(type_name: &str, op_name: &str) -> String {
    let delim = type_name.is_empty().then_some("").unwrap_or("-");
    format!("op{delim}{type_name}-{op_name}-reply")
}

pub fn gen_ops(spec: &Spec, ctx: &mut Context) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    let mut request_names = Vec::new();

    for op in &spec.operations.list {
        if op.mcgrp.is_some() && op.r#do.is_none() && op.dump.is_none() {
            println!("{WARNING} Multicast notifications are yet to be implemented");
            continue;
        }

        gen_op(&mut tokens, spec, ctx, op, &mut request_names);
    }

    if let Some(attrs) = &spec.operations.fallback_attrs {
        let ops = OperationSpec {
            name: "".into(),
            transparent: true,
            request_type_at_runtime: true,
            attribute_set: Some(attrs.clone()),
            r#do: Some(Operation::default()),
            dump: Some(Operation::default()),
            value: Some("0xfefe".into()),
            fixed_header: if spec.is_genetlink() {
                None
            } else if let Some(fixed_header) = &spec.operations.fixed_header {
                Some(fixed_header.clone())
            } else {
                panic!(
                    "Netlink-raw specification specifies fallback-attrs, but not fixed-header. Likely an error"
                )
            },
            ..Default::default()
        };
        gen_op(&mut tokens, spec, ctx, &ops, &mut request_names);
    }

    gen_request_impl::gen_request(&mut tokens, ctx, spec, &request_names);

    tokens
}

pub struct OpHeader {
    pub name: String,
    #[allow(clippy::type_complexity)]
    pub construct_header: Option<Box<dyn Fn(&Ident, Option<&TokenStream>) -> TokenStream>>,
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

pub fn parse_value(value: &str) -> u16 {
    if let Some(value) = value.strip_prefix("0x") {
        u16::from_str_radix(value, 16).unwrap()
    } else {
        value.parse().unwrap()
    }
}

pub fn get_value(ops: &OperationSpec, op: &Request) -> u16 {
    let same_ptr = |l: &Request, r: &Request| l as *const _ == r as *const _;
    let is_request = |l: &Option<Operation>| l.as_ref().is_some_and(|o| same_ptr(&o.request, op));
    let is_reply = |l: &Option<Operation>| l.as_ref().is_some_and(|o| same_ptr(&o.reply, op));

    let is_request = if is_request(&ops.r#do) || is_request(&ops.dump) {
        true
    } else if is_reply(&ops.r#do) || is_reply(&ops.dump) {
        false
    } else {
        unreachable!()
    };

    let check_do_op = |check_request: bool| {
        let Some(op) = ops.r#do.as_ref() else {
            return None;
        };
        if check_request {
            op.request.value.as_ref()
        } else {
            op.reply.value.as_ref()
        }
    };

    let value = None
        .or(op.value.as_ref())
        .or(ops.value.as_ref())
        .or_else(|| check_do_op(is_request))
        .or_else(|| check_do_op(!is_request))
        .unwrap_or_else(|| panic!("Operation {:?} doesn't specify a value", ops.name));

    parse_value(value)
}

pub fn gen_op(
    tokens: &mut TokenStream,
    spec: &Spec,
    ctx: &mut Context,
    ops: &OperationSpec,
    request_names: &mut Vec<OpInfo>,
) {
    let Some(attrset) = &ops.attribute_set else {
        return;
    };
    let attrs = spec.find_attr(attrset);

    let is_transparent = spec.operations.transparent || ops.transparent;
    let needs_value = ops.request_type_at_runtime;

    let fixed_header = |op: &Request| match (&ops.fixed_header, spec.protocol.as_deref()) {
        (Some(h), _) => Some(OpHeader {
            name: h.clone(),
            construct_header: None,
        }),
        (None, Some("genetlink" | "genetlink-legacy")) => {
            let construct_header = {
                let cmd = get_value(ops, op) as u8;
                let cmd = quote!(#cmd);
                // The expected use for genlmsghdr.version was to allow versioning of the
                // APIs provided by the subsystems. No subsystem to date made significant use
                // of this field, so setting it to 1 seems like a safe bet.
                // From: https://docs.kernel.org/userspace-api/netlink/intro.html#generic-netlink
                let version: u8 = 1;
                Box::new(move |header: &Ident, value_expr: Option<&TokenStream>| {
                    assert_eq!(needs_value, value_expr.is_some());
                    let value = value_expr.unwrap_or(&cmd);
                    quote! {
                        #header.set_cmd(#value);
                        #header.set_version(#version);
                    }
                }) as Box<dyn Fn(&Ident, Option<&TokenStream>) -> TokenStream>
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
        let request_name = request_kebab_name(&ops.name, op_name);
        let mut request_attrs = AttrSet {
            name: request_name.clone(),
            subset_of: Some(attrs.name.clone()),
            ..attrs.clone()
        };
        if !spec.operations.all_attrs && !ops.all_attrs {
            whitelist_op_attrs(&mut request_attrs.attributes, &op.request.attributes);
        }
        let request_header = || fixed_header(&op.request);
        request_names.push(OpInfo {
            name: request_name.clone(),
            header: request_header(),
            needs_value,
        });
        let request_header = request_header();

        let reply_header = fixed_header(&op.reply);
        let reply_name = reply_kebab_name(&ops.name, op_name);
        let mut reply_attrs = AttrSet {
            name: reply_name.clone(),
            subset_of: Some(attrs.name.clone()),
            ..attrs.clone()
        };
        if !spec.operations.all_attrs && !ops.all_attrs {
            whitelist_op_attrs(&mut reply_attrs.attributes, &op.reply.attributes);
        }

        if !is_transparent {
            tokens.extend(doc.clone());
            gen_writable_attrset(
                tokens,
                ctx,
                spec,
                &request_attrs,
                request_header.as_ref(),
                needs_value,
            );

            tokens.extend(doc.clone());
            gen_attrset(tokens, spec, ctx, &request_attrs, request_header.as_ref());

            tokens.extend(doc.clone());
            gen_writable_attrset(
                tokens,
                ctx,
                spec,
                &reply_attrs,
                reply_header.as_ref(),
                needs_value,
            );

            tokens.extend(doc.clone());
            gen_attrset(tokens, spec, ctx, &reply_attrs, reply_header.as_ref());
        }

        gen_request_impl::gen_request_wrapper(
            tokens,
            ctx,
            spec,
            op_name == "dump",
            get_value(ops, &op.request),
            &request_attrs,
            &reply_attrs,
            &request_name,
            &reply_name,
            request_header.as_ref(),
            reply_header.as_ref(),
            needs_value,
            ops.attribute_set
                .as_ref()
                .map(|s| s.as_str())
                .take_if(|_| is_transparent),
        );
    };

    if let Some(dump) = &ops.dump {
        generate("dump", dump);
    }

    if let Some(r#do) = &ops.r#do {
        generate("do", r#do);
    }
}
