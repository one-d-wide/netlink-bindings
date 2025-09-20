use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    gen_attrs::gen_attrset,
    gen_utils::kebab_to_type,
    gen_writable::gen_writable_attrset,
    parse_spec::{AttrProp, AttrType, Operation, OperationSpec, Spec},
    Context,
};

pub fn request_type_name(type_name: &str, op_name: &str) -> String {
    let type_name = kebab_to_type(type_name);
    format!("Op{type_name}{op_name}Request")
}

pub fn reply_type_name(type_name: &str, op_name: &str) -> String {
    let type_name = kebab_to_type(type_name);
    format!("Op{type_name}{op_name}Reply")
}

pub fn gen_ops(spec: &Spec, ctx: &mut Context) -> TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    for op in &spec.operations.list {
        gen_op(&mut tokens, spec, ctx, op);
    }

    tokens
}

pub struct OpHeader {
    pub name: String,
    #[allow(clippy::type_complexity)]
    pub fill: Option<Box<dyn Fn(&Ident) -> TokenStream>>,
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

pub fn gen_op(tokens: &mut TokenStream, spec: &Spec, ctx: &mut Context, ops: &OperationSpec) {
    let Some(attrset) = &ops.attribute_set else {
        return;
    };
    let attrs = spec.find_attr(attrset);

    let type_name = kebab_to_type(&ops.name);
    let fixed_header = match (&ops.fixed_header, spec.protocol.as_deref()) {
        (Some(h), _) => Some(OpHeader {
            name: h.clone(),
            fill: None,
        }),
        (None, Some("genetlink" | "genetlink-legacy")) => {
            let fill = ops.value.clone().map(|value| {
                let value: u8 = value.parse().unwrap();
                let version: u8 = 1;
                Box::new(move |header: &Ident| {
                    quote! {
                        #header.set_cmd(#value);
                        #header.set_version(#version);
                    }
                }) as Box<dyn Fn(&Ident) -> TokenStream>
            });

            Some(OpHeader {
                name: "builtin-nfgenmsg".into(),
                fill,
            })
        }
        _ => None,
    };
    let fixed_header = fixed_header.as_ref();

    let doc = ops
        .doc
        .as_ref()
        .map(|doc| quote!(#[doc = #doc]))
        .unwrap_or_default();

    let mut generate = |op_name: &str, op: &Operation| {
        let mut request_attrs = attrs.clone();
        whitelist_op_attrs(&mut request_attrs.attributes, &op.request.attributes);
        request_attrs.name = request_type_name(&type_name, op_name);

        tokens.extend(doc.clone());
        gen_writable_attrset(tokens, ctx, spec, &request_attrs, fixed_header);

        tokens.extend(doc.clone());
        gen_attrset(tokens, spec, ctx, &request_attrs, fixed_header);

        let mut reply_attrs = attrs.clone();
        whitelist_op_attrs(&mut reply_attrs.attributes, &op.reply.attributes);
        reply_attrs.name = reply_type_name(&type_name, op_name);

        tokens.extend(doc.clone());
        gen_writable_attrset(tokens, ctx, spec, &reply_attrs, fixed_header);

        tokens.extend(doc.clone());
        gen_attrset(tokens, spec, ctx, &reply_attrs, fixed_header);
    };

    if let Some(dump) = &ops.dump {
        generate("Dump", dump);
    }

    if let Some(r#do) = &ops.r#do {
        generate("Do", r#do);
    }
}
