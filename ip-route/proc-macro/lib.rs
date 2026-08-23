extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use std::fmt::Write;
use syn::{
    Expr, Ident, LitStr, Path, PathArguments, PathSegment, Token, UsePath, UseTree,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    visit_mut::VisitMut,
};

use ip_route_syntax::*;

struct FmtArgs {
    sock: Option<Expr>,
    fmt: LitStr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for FmtArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut sock = None;
        if input.fork().parse::<LitStr>().is_err() {
            sock = Some(input.parse::<Expr>()?);
            input.parse::<Token![,]>()?;
        }
        let fmt = input.parse::<LitStr>()?;
        input.parse::<Token![,]>().ok();
        let args = Punctuated::<Expr, Token![,]>::parse_terminated(input)?;

        Ok(FmtArgs { sock, fmt, args })
    }
}
#[proc_macro]
pub fn ip(input: TokenStream) -> TokenStream {
    ip_inner(input, false)
}

#[proc_macro]
pub fn ip_dump(input: TokenStream) -> TokenStream {
    ip_inner(input, true)
}

fn ip_inner(input: TokenStream, is_dump: bool) -> TokenStream {
    let input = parse_macro_input!(input as FmtArgs);

    let cmd = input.fmt.value();
    let res = codegen::parse(&cmd);

    let (_, desc, command) = codegen::get_command(&cmd);

    let mut toks: proc_macro2::TokenStream = match syn::parse_str(&res.stream) {
        Ok(x) => x,
        Err(err) => return err_codegen(&res.stream, err).into(),
    };

    let mut prelude: proc_macro2::TokenStream = match syn::parse_str(command.prelude) {
        Ok(x) => x,
        Err(err) => return err_codegen(&res.stream, err).into(),
    };

    toks = (command.generate)(&desc, toks);

    prelude.extend(toks);
    toks = prelude;

    let mut args = proc_macro2::TokenStream::new();
    for (i, e) in input.args.iter().enumerate() {
        let span = e.span();
        let arg: Ident = syn::parse_str(&format!("arg{i}")).unwrap();
        args.extend(quote::quote_spanned! {span=>
            let #arg = #e;
        });
    }

    let mut errs = proc_macro2::TokenStream::new();
    if res.argc < input.args.len() {
        errs.extend(err_unused_args(&input, res.argc));
    }

    if input.sock.is_some() && !is_dump && command.is_dump() {
        errs.extend(err_dump(&input));
    }

    toks = quote! {
        #errs
        #args
        #toks
    };

    let sock = input.sock.as_ref().map(ToTokens::to_token_stream);
    let res = command.runner(sock, is_dump, toks);

    let mut res: syn::Block = syn::parse2(quote! {{ #res }}).unwrap();

    SubtPaths.visit_block_mut(&mut res);

    // eprintln!("{}", res.to_token_stream().to_string());

    res.into_token_stream().into()
}

fn is_external(ident: &Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "std" | "libc" | "netlink_bindings" | "netlink_socket2" | "ipnet",
    )
}

struct SubtPaths;
impl VisitMut for SubtPaths {
    fn visit_use_path_mut(&mut self, use_path: &mut UsePath) {
        if is_external(&use_path.ident) {
            for seg in ["ip_route", "reexports"].iter().rev() {
                *use_path = UsePath {
                    ident: syn::parse_str(seg).unwrap(),
                    tree: Box::new(UseTree::Path(use_path.clone())),
                    colon2_token: Default::default(),
                };
            }
        }
    }
    fn visit_path_mut(&mut self, path: &mut Path) {
        if path.segments.first().is_some_and(|s| is_external(&s.ident)) {
            let seg = |name| PathSegment {
                ident: syn::parse_str(name).unwrap(),
                arguments: PathArguments::None,
            };

            path.leading_colon.get_or_insert_default();
            path.segments.insert(0, seg("ip_route"));
            path.segments.insert(1, seg("reexports"));
        }
        syn::visit_mut::visit_path_mut(self, path)
    }
}

fn err_codegen(res: &str, err: syn::Error) -> proc_macro2::TokenStream {
    let mut err = format!("Error parsing codegen output: {err}");
    for line in res.lines() {
        writeln!(err, "> {line}").unwrap();
    }

    syn::Error::new(Span::call_site(), err).into_compile_error()
}

fn err_unused_args(input: &FmtArgs, expected: usize) -> proc_macro2::TokenStream {
    let mut err = syn::Error::new(
        input.args[expected].span(),
        &format!("unused argument in format!()-like macro"),
    );

    err.combine(syn::Error::new(
        input.fmt.span(),
        &format!(
            "this command contains only {} unnamed substitutions \"{{}}\", while {} arguments are provided",
            expected,
            input.args.len(),
        ),
    ));

    err.into_compile_error()
}

fn err_dump(input: &FmtArgs) -> proc_macro2::TokenStream {
    let sock = input.sock.to_token_stream();
    let fmt = input.fmt.to_token_stream();
    let args = if input.args.len() > 0 {
        format!("{sock}, {fmt}, {}", input.args.to_token_stream())
    } else {
        format!("{sock}, {fmt}")
    };

    let mut err = syn::Error::new(
        Span::call_site(),
        format!("this command may have a response, capture it using ip_dump!() macro"),
    );

    err.combine(syn::Error::new(
        Span::call_site(),
        format!(
            "consider changing this to

    let mut iter = ip_dump!({args})?;
    while let Some((header, attrs)) = iter.recv().transpose()? {{
        // ...
    }}

or simply

    let (header, attrs) = ip_dump!({args})?.recv_one()?;
"
        ),
    ));

    err.into_compile_error()
}

#[doc(hidden)]
#[proc_macro]
pub fn _write_integration_tests(_: TokenStream) -> TokenStream {
    let path = format!("{}/tests.rs", std::env::var("CARGO_TARGET_TMPDIR").unwrap());
    std::fs::write(&path, collect_blobs()).unwrap();
    quote! { #path }.into()
}

fn collect_blobs() -> String {
    let mut res = String::new();
    for commands in COMMANDS.iter() {
        for command in commands.iter() {
            let mut out = String::new();
            collect_blobs_map(&mut out, command.syntax);

            let mut toks: proc_macro2::TokenStream = syn::parse_str(&out).unwrap();
            let desc = command
                .prefix
                .split(" ")
                .map(|s| s.split("/").next().unwrap())
                .collect::<Vec<_>>()
                .join(" ");

            toks = (command.generate)(&desc, toks);

            let ignore = stringify! {
                macro_rules! ignore_tokens {
                    ($($arg:tt)*) => {};
                }
            };

            out = toks.to_string().replace("compile_error", "ignore_tokens");

            writeln!(res, "let _ = {{ {ignore} {} {} }};", command.prelude, out).unwrap();
        }
    }
    format!("{{ {res} }}")
}

fn collect_blobs_map(out: &mut String, m: &Map) {
    match m {
        Map::All(maps) => {
            for m in maps.iter() {
                collect_blobs_map(out, m);
            }
        }
        Map::Any(maps) => {
            for m in maps.iter() {
                out.push_str("{");
                collect_blobs_map(out, m);
                out.push_str("}");
            }
        }
        Map::Plus(m) => {
            collect_blobs_map(out, m);
        }
        Map::Star(m) => {
            out.push_str("{");
            collect_blobs_map(out, m);
            out.push_str("}");
        }
        Map::May(m) => {
            out.push_str("{");
            collect_blobs_map(out, m);
            out.push_str("}");
        }
        Map::Code(code) => {
            out.push_str(code);
            out.push_str("\n");
        }
        Map::Val(Val { name, ty, .. }) => {
            out.push_str(&format!("let {name}: {ty} = unreachable!();\n"));
        }
        Map::Lit(_) | Map::AssertLast | Map::Last => {}
    }
}
