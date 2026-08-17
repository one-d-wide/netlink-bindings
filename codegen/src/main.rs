// vim:textwidth=80

use argh::FromArgs;
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    collections::HashSet,
    ffi::CString,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

mod dump;
mod gen_attrs;
mod gen_cstruct;
mod gen_debug_impl;
mod gen_defs;
mod gen_iterable;
mod gen_lookup;
mod gen_notif;
mod gen_ops;
mod gen_request_impl;
mod gen_reverse_lookup;
mod gen_sub_message;
mod gen_utils;
mod gen_writable;
mod parse_spec;

use crate::{
    gen_attrs::gen_attrsets,
    gen_cstruct::gen_cstruct,
    gen_defs::gen_defs,
    gen_notif::gen_notifs,
    gen_ops::gen_ops,
    gen_utils::{escape_md, Pandoc},
    gen_writable::gen_writable,
    parse_spec::{DefType, Spec},
};

/// ANSI escapes to show bold yellow "warning:"
const WARNING: &str = "\x1b[1m\x1b[33mwarning\x1b(B\x1b[m:";

#[derive(Debug, Default)]
struct Context {
    args: CliArgs,
    pandoc: Option<Pandoc>,
    test_exprs: HashSet<String>,
    generated_array_introspect: HashSet<String>,
    generated_array_iterable: HashSet<String>,
    generated_arrays: HashSet<String>,
    generated_sub_messages: HashSet<String>,
}

#[derive(FromArgs, Clone, Debug, Default)]
/// Generate Rust bindings for Netlink from YAML specification
#[argh(help_triggers("-h", "--help"))]
struct CliArgs {
    /// target dir (overrides outputs)
    #[argh(option, short = 'd', arg_name = "path")]
    #[argh(usage)]
    dir: Option<PathBuf>,

    /// output for Rust bindings
    #[argh(option, short = 'o', arg_name = "path")]
    #[argh(usage)]
    output: Option<PathBuf>,

    /// dump generated interface
    #[argh(option, arg_name = "path")]
    #[argh(usage)]
    dump: Option<PathBuf>,

    /// dump generated interface
    #[argh(option, arg_name = "path")]
    #[argh(usage)]
    dump_all: Option<PathBuf>,

    /// formatter command
    #[argh(option, arg_name = "command", default = "\"rustfmt\".into()")]
    fmt: PathBuf,

    /// don't use formatter
    #[argh(switch)]
    no_fmt: bool,

    /// don't use pandoc
    #[argh(switch)]
    no_pandoc: bool,

    /// don't generate dump
    #[argh(switch)]
    no_dump: bool,

    /// don't generate writables
    #[argh(switch)]
    no_writable: bool,

    /// don't generate operation
    #[argh(switch)]
    no_operations: bool,

    /// aggregate reverse lookup
    #[argh(option, arg_name = "path")]
    #[argh(usage)]
    reverse_lookup: Option<PathBuf>,

    #[argh(positional, arg_name = "path_to_spec")]
    #[argh(usage)]
    spec: Option<PathBuf>,
}

fn main() {
    let mut args: CliArgs = argh::from_env();

    if let Some(output) = &args.reverse_lookup {
        gen_reverse_lookup::gen_reverse_lookup(&args, output);
        return;
    }

    let mut has_tests = false;
    if let Some(mut dir) = args.dir.clone() {
        let mut prot = dir.file_name().unwrap().to_str().unwrap();

        if let Some(spec) = &args.spec {
            if prot == "src" {
                if spec.extension().is_none_or(|ext| ext != "yaml") {
                    println!("Provided spec doesn't look .yaml file. Refusing to copy");
                    std::process::exit(1);
                }
                prot = spec.file_stem().unwrap().to_str().unwrap();
                dir = dir.join(prot);
            }
        }

        args.output = Some(dir.join("mod.rs"));
        // args.dump = Some(dir.join(format!("{prot}.md")));
        // args.dump_all = Some(dir.join(format!("{prot}-all.md")));
        let new_spec = dir.join(format!("{prot}.yaml"));
        if let Some(spec) = &args.spec {
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::copy(spec, &new_spec).unwrap();
        }
        args.spec = Some(new_spec);

        if dir.join("tests.rs").exists() {
            has_tests = true;
        }
    }

    let Some(spec) = &args.spec else {
        println!("Path to yaml spec not provided");
        std::process::exit(1);
    };

    let spec = Spec::parse_with_overrides(spec);

    let mut ctx = Context {
        args: args.clone(),
        ..Default::default()
    };
    let mut tokens = TokenStream::new();

    let mod_doc = escape_md(&mut ctx, &spec.doc);
    tokens.extend(quote! {
        #![doc = #mod_doc]
        #![allow(clippy::all)]
        #![allow(unused_imports)]
        #![allow(unused_assignments)]
        #![allow(non_snake_case)]
        #![allow(unused_variables)]
        #![allow(irrefutable_let_patterns)]
        #![allow(unreachable_code)]
        #![allow(unreachable_patterns)]
    });
    if has_tests {
        tokens.extend(quote! {
            #[cfg(test)]
            mod tests;
        });
    }
    tokens.extend(quote! {
        use crate::{
            consts,
            traits::{NetlinkRequest, Protocol},
            utils::*,
        };
    });
    if spec.name != "builtin" {
        tokens.extend(quote! {
            use crate::builtin::{BuiltinBitfield32, BuiltinNfgenmsg, Nlmsghdr, PushDummy};
        });
    }
    let protoname = &spec.name;
    let cprotoname = CString::new(spec.name.clone()).unwrap();
    tokens.extend(quote! {
        pub const PROTONAME: &str = #protoname;
        pub const PROTONAME_CSTR: &CStr = #cprotoname;
    });
    if let Some(protonum) = &spec.protonum {
        tokens.extend(quote! {
            pub const PROTONUM: u16 = #protonum;
        });
    } else if spec.name == "nlctrl" {
        tokens.extend(quote! {
            pub const PROTONUM: u16 = 0x10;
        });
    }
    tokens.extend(gen_defs(&spec, &mut ctx));
    for def in &spec.definitions {
        let DefType::Struct { members, .. } = &def.def else {
            continue;
        };

        gen_cstruct(&mut tokens, &spec, &mut ctx, def, members);
    }
    tokens.extend(gen_attrsets(&spec, &mut ctx));
    if !args.no_writable {
        tokens.extend(gen_writable(&spec, &mut ctx));
    }
    gen_notifs(&mut tokens, &spec, &mut ctx);
    if !args.no_operations {
        gen_ops(&mut tokens, &spec, &mut ctx);
    }

    let mut tokens_str = tokens.to_string();

    if !ctx.test_exprs.is_empty() {
        let mut tests: Vec<_> = ctx.test_exprs.iter().map(|s| s.as_str()).collect();
        tests.sort();
        let tests = tests.join("\n");
        tokens_str.push_str(&format!(
            "
            #[cfg(test)]
            mod generated_tests {{
                use super::*;
                #[test]
                fn tests() {{
                    {tests}
                }}
            }}
        "
        ));
    }

    let out = fmt(&args, &tokens_str);

    if let Some(output) = &args.output {
        println!("Writing {output:?}");
        std::fs::write(output, &out).unwrap();
    }

    if !args.no_dump {
        if let Some(dump) = &args.dump {
            let mut buf = Vec::new();
            dump::dump_ops(&mut buf, &mut ctx, &spec, false);
            println!("Dumping {dump:?}");
            std::fs::write(dump, &buf).unwrap();
        }

        if let Some(dump_all) = &args.dump_all {
            let mut buf = Vec::new();
            dump::dump_ops(&mut buf, &mut ctx, &spec, true);
            println!("Dumping all {dump_all:?}");
            std::fs::write(dump_all, &buf).unwrap();
        }
    }
}

fn fmt(args: &CliArgs, tokens: &str) -> Vec<u8> {
    if !args.no_fmt {
        let mut proc = Command::new(&args.fmt)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .arg("--edition=2024")
            .spawn()
            .unwrap();

        proc.stdin
            .as_mut()
            .unwrap()
            .write_all(tokens.as_bytes())
            .unwrap();

        let res = proc.wait_with_output().unwrap();

        if !res.status.success() {
            println!("Error formatting with {:?}", args.fmt);
            std::process::exit(1);
        }

        res.stdout
    } else {
        tokens.as_bytes().to_vec()
    }
}
