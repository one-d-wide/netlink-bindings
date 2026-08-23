// Proof of concept for the interpreter:
//
//     cargo run --bin interp-poc
//     cargo run --bin interp-poc-generated

use ip_route_syntax::*;

use proc_macro2::TokenStream;
use quote::ToTokens;

use std::{cell::RefCell, fmt::Write, rc::Rc, str::FromStr};

#[derive(Clone)]
pub struct State<'a> {
    pub out: Rc<RefCell<Vec<usize>>>,
    pub out_len: usize,
    pub argc: usize,
    pub iter: std::iter::Peekable<std::iter::Cloned<std::slice::Iter<'a, &'a str>>>,
    pub errs: Rc<RefCell<Vec<String>>>,
    pub warns: Rc<RefCell<Vec<String>>>,
}

impl<'a> State<'a> {
    fn map_id(&mut self, map: &'static Map) -> usize {
        map as *const _ as usize
    }

    fn push(&mut self, map: &'static Map) {
        let x = self.map_id(map);
        let mut out = self.out.borrow_mut();
        out.truncate(self.out_len);
        out.push(x);
        self.out_len = out.len();
    }

    fn tokens(&mut self) -> Vec<usize> {
        let mut out = self.out.borrow_mut();
        out.truncate(self.out_len);
        out.clone()
    }
}

pub fn parse_map(s: &mut State, map: &'static Map) -> bool {
    match map {
        Map::All(all) => {
            let old = s.clone();
            for m in all.iter() {
                if !parse_map(s, m) {
                    *s = old;
                    return false;
                }
            }
            true
        }
        Map::Any(any) => {
            let old = s.clone();
            for m in any.iter() {
                s.push(m);
                if parse_map(s, m) {
                    return true;
                }
                *s = old.clone();
            }
            false
        }
        Map::May(map) => {
            let old = s.clone();
            s.push(map);
            if !parse_map(s, map) {
                *s = old.clone();
            }
            true
        }
        Map::Star(map) => {
            while s.iter.peek().is_some() {
                let old = s.clone();
                s.push(map);
                if !parse_map(s, map) {
                    *s = old.clone();
                    break;
                }
            }
            true
        }
        Map::Plus(map) => {
            let old = s.clone();
            s.push(map);
            if !parse_map(s, map) {
                *s = old;
                return false;
            }
            while s.iter.peek().is_some() {
                let old = s.clone();
                s.push(map);
                if !parse_map(s, map) {
                    *s = old.clone();
                    break;
                }
            }
            true
        }
        Map::Val(val) => {
            if !val.flags.contains(&"static") && !val.flags.contains(&"static-str") {
                s.iter.next();
                return true;
            }
            false
        }
        Map::Code(_) => true,
        Map::AssertLast => {
            if let Some(tok) = s.iter.peek() {
                s.errs
                    .borrow_mut()
                    .push(format!("Unexpected attribute: {tok:?}"));
                return false;
            }
            true
        }
        Map::Last => s.iter.peek().is_none(),
        Map::Lit(lit) => {
            if s.iter.peek().is_none_or(|tok| tok != lit) {
                return false;
            }
            s.iter.next();
            true
        }
    }
}

pub fn code_map(s: &mut State, map: &Map, c: &mut String) {
    match map {
        Map::All(all) => {
            for m in all.iter() {
                code_map(s, m, c);
            }
        }
        Map::Any(any) => {
            if any.is_empty() {
                return;
            }

            writeln!(c, "match i.pop() {{").unwrap();
            for m in any.iter() {
                let x = s.map_id(m);

                writeln!(c, "{x} => {{").unwrap();
                code_map(s, m, c);
                writeln!(c, "}}").unwrap();
            }
            writeln!(c, "_ => unreachable!(),").unwrap();
            writeln!(c, "}}").unwrap();
        }
        Map::May(m) => {
            let x = s.map_id(m);
            writeln!(c, "if i.next_if({x}) {{").unwrap();
            code_map(s, m, c);
            writeln!(c, "}}").unwrap();
        }
        Map::Star(m) | Map::Plus(m) => {
            let x = s.map_id(m);

            writeln!(c, "loop {{").unwrap();
            writeln!(c, "if !i.next_if({x}) {{ break }}").unwrap();
            code_map(s, m, c);
            writeln!(c, "}}").unwrap();
        }
        Map::Val(val) => {
            writeln!(c, "let {}: {} = next_arg();", val.name, val.ty).unwrap();
        }
        Map::Code(code) => {
            writeln!(c, "{code}").unwrap();
        }
        Map::Lit(_) | Map::AssertLast | Map::Last => {}
    }
}

fn main() {
    let line = "ip link set lo up";

    let (args, _, comm) = codegen::get_command(line);

    let mut s = State {
        out: Default::default(),
        errs: Default::default(),
        warns: Default::default(),
        out_len: 0,
        argc: 0,
        iter: args.iter().cloned().peekable(),
    };

    let mut blob = String::new();
    code_map(&mut s, comm.syntax, &mut blob);

    s.push(comm.syntax);
    if !parse_map(&mut s, comm.syntax) || s.iter.peek().is_some() {
        s.errs
            .borrow_mut()
            .push(format!("Didn't match token {:?}", s.iter.peek().unwrap()));
    }

    if !s.errs.borrow().is_empty() {
        dbg!(s.errs);
        std::process::exit(1);
    }

    // eprintln!("{:#x?}", s.out);
    eprintln!("{:#?}", s.out);

    let tok = TokenStream::from_str(&blob).unwrap();

    let ids = s.tokens();

    let x = s.map_id(comm.syntax);
    let lp = quote::quote! {
        const IDS: &[usize] = &[#(#ids),*];

        struct Seq(usize);
        impl Seq {
            fn next_if(&mut self, x: usize) -> bool {
                if self.0 < IDS.len() && IDS[self.0] == x {
                    self.0 += 1;
                    return true;
                } else {
                    return false;
                }
            }
            fn pop(&mut self) -> usize {
                let res = IDS[self.0];
                self.0 += 1;
                res
            }
            fn is_empty(&self) -> bool {
                self.0 >= IDS.len()
            }
        }

        let mut i = Seq(0);
        fn next_arg<T: Default>() -> T {
            Default::default()
        }

        while !i.is_empty() {
            if !i.next_if(#x) { panic!("leftover: {:#?}", &IDS[i.0..]) }
            #tok
        }
    };

    let tok = (comm.generate)("ip route add", lp);

    let c = tok.into_token_stream().to_string();
    let c = c.replace("compile_error !", "panic !");
    let prelude = comm.prelude;

    let res = format!(
        "pub fn main() {{
            #![allow(unused)]
            {prelude}
            let req = {{ {c} }};
            let mut sock = netlink_socket2::NetlinkSocket::new();
            sock.request(&req).unwrap().recv_ack().unwrap();
            dbg!(req);
        }}"
    );

    let bin = "./ip-route/src/bin";
    std::fs::create_dir_all(bin).unwrap();
    let file = format!("{bin}/interp-poc-generated.rs");
    println!("Writing {file:?}");
    std::fs::write(&file, res).unwrap();

    std::process::Command::new("rustfmt")
        .arg(&file)
        .spawn()
        .unwrap();
}
