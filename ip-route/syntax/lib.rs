use proc_macro2::TokenStream;
use quote::quote;

pub mod ip_addr;
pub mod ip_link;
pub mod ip_route;
pub mod ip_rule;
pub mod iptables;

pub mod codegen;
mod macros;

pub const COMMANDS: &[&[&Command]] = &[
    ip_addr::COMMANDS,
    ip_link::COMMANDS,
    ip_route::COMMANDS,
    ip_rule::COMMANDS,
    iptables::COMMANDS,
    // ...
];

pub const EPSILON: Map = Map::Last;
pub const ASSERT_LAST: Map = Map::AssertLast;
pub const ANY: Map = Map::Val(Val {
    name: "any",
    ty: "&str",
    flags: &[],
});

#[derive(Debug)]
pub enum Map {
    /// All inner maps must match in order.
    All(&'static [Self]),
    /// Any inner map must match.
    /// The code it generates is scoped, think of it like a `match` expression.
    Any(&'static [Self]),
    /// May or may not repeat.
    May(&'static Self),
    /// Repeats zero or more times.
    Star(&'static Self),
    /// Repeats one or more times.
    Plus(&'static Self),

    /// A literal string: `"foo"`, `"--foo"`, `"-f"`, etc.
    Lit(&'static str),
    /// A value: `"1"` (static), `"{}"` (argument), `"{var}"` (variable), `"{}[-{}][/{}]"` (pattern)
    Val(Val),

    /// A code snippet
    Code(&'static str),

    /// Matches if there are no more tokens
    Last,
    /// Assert that there are no more tokens
    AssertLast,
}

/// A value for a format!()-like substitutions.
///
/// This matches:
/// - Static values: `"42"`
/// - Local variables: `"{local}"`
/// - Arguments: `"{}", local`
/// - Formatted values: `"{}[-{}][/{}]"` (a tuple with possible None fields)
/// - Conditional: `"{:option}"` the following code is only ran on Some(_)
#[derive(Debug, Clone)]
pub struct Val {
    /// Variable name
    pub name: &'static str,
    /// Variable type
    pub ty: &'static str,
    /// Flags:
    /// - `"static"`, `"static-str"` - only match static values
    /// - `"time-ms"` - interpret suffixes like "s" or "ms", output milliseconds
    /// - `"mac"` - value is a 6-byte MAC address
    /// - `"hex"` - always interpret value as hexadecimal, even without "0x" prefix
    /// - `"format:{}[/{}]"` - a tuple with possible optional values in `"[]"`
    pub flags: &'static [&'static str],
}

pub const fn lit(lit: &'static str) -> Map {
    Map::Lit(lit)
}

pub const fn code(code: &'static str) -> Map {
    Map::Code(code)
}

impl Map {
    pub const fn star(&'static self) -> Self {
        Map::Star(self)
    }
    pub const fn plus(&'static self) -> Self {
        Map::Plus(self)
    }
    pub const fn may(&'static self) -> Self {
        Map::May(self)
    }

    pub fn visit(&self, f: &mut impl FnMut(&Map)) {
        f(self);
        match self {
            Map::All(maps) => maps.iter().for_each(|m| m.visit(f)),
            Map::Any(maps) => maps.iter().for_each(|m| m.visit(f)),
            Map::May(map) => map.visit(f),
            Map::Star(map) => map.visit(f),
            Map::Plus(map) => map.visit(f),
            _ => {}
        }
    }
}

pub type CommandGen = fn(desc: &str, stream: TokenStream) -> TokenStream;
pub type CommandGenChain = fn(desc: &str, stream: TokenStream, chain: TokenStream) -> TokenStream;

#[derive(Debug, Clone)]
pub struct Command {
    pub prefix: &'static str,
    pub prelude: &'static str,
    pub syntax: &'static Map,
    pub generate: CommandGen,
    pub generate_chain: Option<CommandGenChain>,
}

impl Command {
    pub fn is_dump(&self) -> bool {
        let list = [
            ("ip", &["get", "show", "list"][..]),
            ("iptables", &["-L"][..]),
            ("ip6tables", &["-L"][..]),
        ];
        for (prefix, verbs) in list {
            let comm = self.prefix.split(" ").next().unwrap();
            if comm.split("/").any(|s| s.contains(prefix)) {
                for verb in verbs {
                    if self.prefix.contains(verb) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn runner(
        &self,
        sock: Option<TokenStream>,
        is_dump: bool,
        toks: TokenStream,
    ) -> TokenStream {
        let comm = self.prefix.split(" ").next().unwrap();

        let Some(sock) = sock else {
            return quote! {{ #toks }};
        };

        if comm.starts_with("iptables") || comm.starts_with("ip6tables") {
            if is_dump {
                quote! {{
                    let req = { #toks };
                    (#sock).request(&req)
                }}
            } else {
                quote! {{
                    let req = { #toks };
                    use ip_route::utils::IptablesDo;
                    (#sock).iptables_request(&req)
                }}
            }
        } else {
            if is_dump {
                quote! {{
                    let req = { #toks };
                    (#sock).request(&req)
                }}
            } else {
                quote! {{
                    let req = { #toks };
                    (#sock).request(&req).map_err(|e| e.into()).and_then(|mut r| r.recv_ack())
                }}
            }
        }
    }
}
