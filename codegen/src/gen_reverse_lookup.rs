use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::{collections::BTreeMap, path::Path};

use crate::{
    gen_ops,
    gen_utils::{kebab_to_rust, kebab_to_type},
    parse_spec::Operation,
    parse_spec::Spec,
    CliArgs,
};

pub fn gen_reverse_lookup(args: &CliArgs, output: &Path) {
    let Some(dir) = &args.dir else {
        println!("Error: target --dir not specified");
        std::process::exit(1);
    };

    if dir.file_name().is_none_or(|s| s != "src") {
        println!("Error: --dir should point to 'src' directory");
        std::process::exit(1);
    };

    let mut genl = BTreeMap::new();
    let mut raw = BTreeMap::new();

    for dir in dir.read_dir().unwrap() {
        let dir = dir.unwrap();
        if !dir.file_type().unwrap().is_dir() {
            continue;
        }

        let spec = dir
            .path()
            .join(format!("{}.yaml", dir.file_name().to_str().unwrap()));

        let spec = std::fs::read_to_string(spec).unwrap();
        let spec = Spec::parse(&spec);

        println!("{:?}", spec.name);

        let ops = &spec.operations.list;
        if ops.is_empty() {
            continue;
        }

        let list = if spec.protocol.is_some_and(|p| p.starts_with("genetlink")) {
            genl.entry(spec.name.clone()).or_insert(Vec::new())
        } else {
            raw.entry(spec.protonum.unwrap()).or_insert(Vec::new())
        };

        for ops in ops {
            let mut generate = |op_name: &str, op: &Operation| {
                let request_value = gen_ops::get_value(
                    ops,
                    &op.request,
                    ops.r#do.as_ref().map(|op| &op.request),
                    None,
                );
                let request_name = gen_ops::request_kebab_name(&ops.name, op_name);

                let reply_value = gen_ops::get_value(
                    ops,
                    &op.reply,
                    ops.r#do.as_ref().map(|op| &op.reply),
                    ops.r#do.as_ref().map(|op| &op.request),
                );

                let reply_name = gen_ops::reply_kebab_name(&ops.name, op_name);
                list.push((
                    spec.name.clone(),
                    request_value,
                    reply_value,
                    request_name,
                    reply_name,
                    op_name == "dump",
                ));
            };

            if let Some(dump) = &ops.dump {
                generate("dump", dump);
            }

            if let Some(r#do) = &ops.r#do {
                generate("do", r#do);
            }
        }
    }

    let b = format_ident!("netlink_bindings");

    let mut netlink_raw = TokenStream::new();
    for (protonum, ops) in &raw {
        let mut variants = TokenStream::new();
        for (proto, request_value, reply_value, request, reply, is_dump) in ops {
            let prefix = format_ident!("{}", kebab_to_rust(proto));
            let request = format_ident!("{}", kebab_to_type(request));
            let reply = format_ident!("{}", kebab_to_type(reply));
            variants.extend(quote! {
                #[cfg(feature = #proto)]
                (#request_value, None, #is_dump) => Debug::fmt(&#b::#prefix::#request::new(buf), fmt),
                #[cfg(not(feature = #proto))]
                (#request_value, None, #is_dump) => consider(fmt, #proto),

                #[cfg(feature = #proto)]
                (#reply_value, Some(#request_value), #is_dump) => Debug::fmt(&#b::#prefix::#reply::new(buf), fmt),
                #[cfg(not(feature = #proto))]
                (#reply_value, Some(#request_value), #is_dump) => consider(fmt, #proto),
            })
        }

        netlink_raw.extend(quote! {
            #protonum => match (value, request_value, is_dump) {
                #variants
                _ => write!(fmt, "(Unknown operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"),
            }
        });
    }

    let mut generic = TokenStream::new();
    for (proto, ops) in &genl {
        let mut variants = TokenStream::new();
        for (_, request_value, reply_value, request, reply, is_dump) in ops {
            let prefix = format_ident!("{}", kebab_to_rust(proto));
            let request_value = *request_value as u8;
            let reply_value = *reply_value as u8;
            let request = format_ident!("{}", kebab_to_type(request));
            let reply = format_ident!("{}", kebab_to_type(reply));
            variants.extend(quote! {
                (#request_value, None, #is_dump) => Debug::fmt(&#b::#prefix::#request::new(buf), fmt),
                (#reply_value, Some(#request_value), #is_dump) => Debug::fmt(&#b::#prefix::#reply::new(buf), fmt),
            });
        }

        let proto_bytes = Literal::byte_string(proto.as_bytes());
        generic.extend(quote! {
            #[cfg(feature = #proto)]
            #proto_bytes => match (value, request_value, is_dump) {
                #variants
                _ => write!(fmt, "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}"),
            }
            #[cfg(not(feature = #proto))]
            #proto_bytes => consider(fmt, #proto),
        });
    }

    let mut tokens = TokenStream::new();
    tokens.extend(quote! {
        #![allow(clippy::all)]
        #![allow(unused_imports)]
        #![allow(unused_assignments)]
        #![allow(non_snake_case)]
        #![allow(unused_variables)]
        #![allow(irrefutable_let_patterns)]
        #![allow(unreachable_code)]
        #![allow(unreachable_patterns)]
        use std::fmt::Debug;
        use netlink_bindings::traits::Protocol;

        #[derive(Clone)]
        pub struct ReverseLookup<'a> {
            pub proto: Protocol,
            pub value: u16,
            pub request_value: Option<u16>,
            pub is_dump: bool,
            pub buf: &'a [u8],
        }

        fn consider(fmt: &mut std::fmt::Formatter<'_>, proto: &str) -> std::fmt::Result {
            write!(fmt, "Protocol {0:?} not enabled, consider --features={0}", proto)
        }

        impl Debug for ReverseLookup<'_> {
            fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let Self { proto, value, request_value, is_dump, buf } = self.clone();
                match proto {
                    Protocol::Raw { protonum, .. } => {
                        match protonum {
                            #netlink_raw
                            _ => write!(fmt, "(Protocol {protonum:?} not recognized)"),
                        }
                    }
                    Protocol::Generic(name) => {
                        let value = value as u8;
                        let request_value = request_value.map(|val| val as u8);
                        match name {
                            #generic
                            _ => write!(fmt, "(Protocol {name:?} not recognized)"),
                        }
                    }
                }
            }
        }
    });

    let out = crate::fmt(args, &tokens);

    println!("Writing {output:?}");
    std::fs::write(output, &out).unwrap();
}
