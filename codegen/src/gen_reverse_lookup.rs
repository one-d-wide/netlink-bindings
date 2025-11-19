use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::{collections::BTreeMap, path::Path, rc::Rc};

use crate::{
    gen_ops::{self, request_kebab_name},
    gen_utils::{kebab_to_rust, kebab_to_type},
    parse_spec::{Operation, Spec},
    CliArgs, WARNING,
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
        let spec = Rc::new(Spec::parse(&spec));

        println!("{:?}", spec.name);

        let ops = &spec.operations.list;
        if ops.is_empty() {
            continue;
        }

        let list = if spec
            .protocol
            .as_ref()
            .is_some_and(|p| p.starts_with("genetlink"))
        {
            genl.entry(spec.name.clone())
                .or_insert((Some(spec.clone()), Vec::new()))
        } else {
            raw.entry(spec.protonum.unwrap())
                .or_insert((Some(spec.clone()), Vec::new()))
        };

        if list.0.as_ref().is_some_and(|other| other.name != spec.name) {
            list.0 = None;
        }

        if list.0.is_none() && spec.operations.fallback_attrs.is_some() {
            println!("{WARNING} Fallback-attrs option is ignored: Multiple netlink families share the same protonum");
        }

        for ops in ops {
            let mut generate = |op_name: &str, op: &Operation| {
                let request_value = gen_ops::get_value(ops, &op.request);
                let request_name = gen_ops::request_kebab_name(&ops.name, op_name);

                let reply_value = gen_ops::get_value(ops, &op.reply);

                let reply_name = gen_ops::reply_kebab_name(&ops.name, op_name);
                list.1.push((
                    spec.name.clone(),
                    request_value,
                    reply_value,
                    request_name,
                    reply_name,
                    op_name == "dump",
                    spec.operations.transparent || ops.transparent,
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

    let gen_fallback = |spec: Option<&Rc<Spec>>| -> Option<_> {
        let spec = spec?;
        let fallback = spec.operations.fallback_attrs.as_ref()?;
        let prefix = format_ident!("{}", kebab_to_rust(&spec.name));
        let attrs_name = kebab_to_type(fallback);
        let request_name = request_kebab_name("", "do");
        let request = format_ident!("Request{}", kebab_to_type(&request_name));
        let fallback = quote! {
            Debug::fmt(&#b::#prefix::#request::decode_reply(buf), fmt)?;
        };

        Some((attrs_name, fallback))
    };

    let mut netlink_raw = TokenStream::new();
    for (protonum, (spec, ops)) in &raw {
        let mut variants = TokenStream::new();
        for (proto, request_value, reply_value, request, reply, is_dump, is_transparent) in ops {
            let prefix = format_ident!("{}", kebab_to_rust(proto));
            let mut request = format_ident!("{}", kebab_to_type(request));
            let mut reply = format_ident!("{}", kebab_to_type(reply));
            let mut decoder = quote!(new);

            if *is_transparent {
                request = format_ident!("Request{}", request);
                reply = request.clone();
                decoder = quote!(decode_reply);
            }

            variants.extend(quote! {
                #[cfg(feature = #proto)]
                if let (#request_value, None, #is_dump) = pat {
                    return Debug::fmt(&#b::#prefix::#request::#decoder(buf), fmt);
                }
                #[cfg(not(feature = #proto))]
                if let (#request_value, None, #is_dump) = pat {
                    return consider(fmt, #proto);
                }

                #[cfg(feature = #proto)]
                if let (#reply_value, Some(#request_value), #is_dump) = pat{
                    return Debug::fmt(&#b::#prefix::#reply::#decoder(buf), fmt);
                }
                #[cfg(not(feature = #proto))]
                if let (#reply_value, Some(#request_value), #is_dump) = pat {
                    return consider(fmt, #proto);
                }
            })
        }

        let fallback = if let Some((attrs, fallback)) = gen_fallback(spec.as_ref()) {
            quote!({
                writeln!(fmt, "Unknown operation, falling back to {:?}: value={value}, request_value={request_value:?}, is_dump={is_dump}", #attrs)?;
                #fallback
            })
        } else {
            quote! {
                write!(fmt, "(Unknown operation) value={value}, request_value={request_value:?}, is_dump={is_dump}")?;
            }
        };

        netlink_raw.extend(quote! {
            if protonum == #protonum {
                let pat = (value, request_value, is_dump);
                #variants
                #fallback
                return Ok(());
            }
        });
    }

    let mut generic = TokenStream::new();
    for (proto, (spec, ops)) in &genl {
        let mut variants = TokenStream::new();
        for (_, request_value, reply_value, request, reply, is_dump, is_transparent) in ops {
            let prefix = format_ident!("{}", kebab_to_rust(proto));
            let request_value = *request_value as u8;
            let reply_value = *reply_value as u8;
            let mut request = format_ident!("{}", kebab_to_type(request));
            let mut reply = format_ident!("{}", kebab_to_type(reply));
            let mut decoder = quote!(new);

            if *is_transparent {
                request = format_ident!("Request{}", request);
                reply = request.clone();
                decoder = quote!(decode_reply);
            }

            variants.extend(quote! {
                if let (#request_value, None, #is_dump) = pat {
                    return Debug::fmt(&#b::#prefix::#request::#decoder(buf), fmt);
                }
                if let (#reply_value, Some(#request_value), #is_dump) = pat {
                    return Debug::fmt(&#b::#prefix::#reply::#decoder(buf), fmt);
                }
            });
        }

        let fallback = if let Some((attrs, fallback)) = gen_fallback(spec.as_ref()) {
            quote!(
                writeln!(fmt, "Unknown genl operation, falling back to {:?} value={value}, request_value={request_value:?}, is_dump={is_dump}", #attrs)?;
                #fallback
            )
        } else {
            quote! {
                write!(fmt, "(Unknown genl operation) value={value}, request_value={request_value:?}, is_dump={is_dump}")?;
            }
        };

        let proto_bytes = Literal::byte_string(proto.as_bytes());
        generic.extend(quote! {
            if name == #proto_bytes {
                let pat = (value, request_value, is_dump);
                #[cfg(feature = #proto)]
                {
                    #variants
                    #fallback
                    return Ok(());
                }
                #[cfg(not(feature = #proto))]
                return consider(fmt, #proto);
            }
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
        use netlink_bindings::{
            builtin::PushBuiltinNfgenmsg,
            traits::{NetlinkRequest, Protocol},
        };

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
                        #netlink_raw
                        write!(fmt, "(Protocol {protonum:?} not recognized)")
                    }
                    Protocol::Generic(name) => {
                        let value = value as u8;
                        let request_value = request_value.map(|val| val as u8);
                        #generic
                        write!(fmt, "(Protocol {name:?} not recognized)")
                    }
                }
            }
        }
    });

    let out = crate::fmt(args, &tokens);

    println!("Writing {output:?}");
    std::fs::write(output, &out).unwrap();
}
