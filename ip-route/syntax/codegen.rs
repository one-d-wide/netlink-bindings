use ipnet::IpNet;
use quote::quote;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{Debug, Write},
    iter::Peekable,
    net::{IpAddr, Ipv4Addr},
    num::ParseIntError,
    rc::Rc,
    str::FromStr,
};

use crate::{COMMANDS, Command, Map, Val};

/// Trigram similarity
fn sort_trig_ascii<S: AsRef<[u8]>, P: AsRef<[u8]>>(vec: &mut [S], pat: P) {
    vec.sort_by_cached_key(|m| {
        let m = m.as_ref();
        let set = m.windows(3).collect::<HashSet<_>>();
        let score = !pat.as_ref().windows(3).filter(|p| set.contains(p)).count();

        let mut pref = [0u8; 4];
        let pref_len = m.len().min(4);
        pref[..pref_len].clone_from_slice(&m[..pref_len]);

        ((score as u64) << 32) | u32::from_be_bytes(pref) as u64
    });
}

/// Find a matching [`Command`].
///
/// A prefix describing a command can contain possible parts separated by "/":
/// "ip addr add/del". These parts are checked as a prefix. Possible flags in
/// between are passed as arguments, e.g. for command `"ip -4 address add ..."`
/// arguments are `["-4", ...]` and descriptor is `"ip addr add"`.
//
/// Returned descriptor contains a parts of the prefix that actually matched.
///
/// Returns (arguments, descriptor, command)
pub fn get_command(command: &str) -> (Vec<&str>, String, &Command) {
    let mut matches = Vec::new();
    for maps in COMMANDS {
        'next_map: for m in maps.iter() {
            let mut args = Vec::new();
            let mut pref_it = command.split_ascii_whitespace().filter(|s| !s.is_empty());

            let mut desc = String::new();
            let mut desc_push = |str| {
                if !desc.is_empty() {
                    desc.push_str(" ");
                }
                desc.push_str(str);
            };
            let mut unparsed = 0;
            let mut pats_it = m.prefix.split_ascii_whitespace().enumerate();
            let mut pats_cur = pats_it.next();
            'next_pat: loop {
                let Some((i, pats)) = pats_cur else {
                    break;
                };

                let Some(pref) = pref_it.next() else {
                    for pat in pats.split("/") {
                        if pat.is_empty() && unparsed == 0 {
                            desc_push(pat);
                            pats_it.next();
                            break 'next_pat;
                        }
                    }
                    continue 'next_map;
                };

                for pat in pats.split("/") {
                    if !pat.is_empty()
                        && (pref == pat && (i != 0 && pref.starts_with(pat)) || pref == pat)
                    {
                        desc_push(pat);
                        pats_cur = pats_it.next();
                        continue 'next_pat;
                    }
                }

                if i == 0 {
                    continue 'next_map;
                }

                unparsed += 1;
                args.push(pref);
            }

            args.extend(pref_it);
            matches.push((unparsed, (args, desc, *m)));
        }
    }

    if let Some((_, res)) = matches.into_iter().min_by_key(|(unparsed, _)| *unparsed) {
        return res;
    }

    let mut list = Vec::new();
    for maps in COMMANDS {
        for m in maps.iter() {
            list.push(m.prefix);
        }
    }

    panic!("{command:?} command is unimplemented:\nExpected one of: {list:#?}")
}

#[test]
fn select() {
    let (vec, desc, comm) = get_command("iptables -a -I -b");
    dbg!(&vec, &desc, comm.prefix);
    assert_eq!(&desc, "iptables -I");
    assert!(comm.prefix.starts_with("iptables"));
    assert_eq!(&vec, &["-a", "-b"]);
}

#[derive(Clone)]
pub struct State<'a> {
    pub shared: Rc<RefCell<SharedState>>,
    pub closing_len: usize,
    pub out_len: usize,
    pub argc: usize,
    pub iter: Iter<'a>,
}

type Iter<'a> = std::iter::Peekable<std::iter::Cloned<std::slice::Iter<'a, &'a str>>>;

#[derive(Clone, Default)]
pub struct SharedState {
    pub closing_level: Rc<()>,
    pub out: String,
    pub closing: Vec<usize>,
    pub tried_literals: HashMap<Pos, HashSet<String>>,
    pub max_ptr_pos: Pos,
    pub errs: Vec<Hint>,
    pub warns: Vec<Hint>,
}

#[derive(Clone)]
pub struct Hint {
    pub pos: Pos,
    pub text: String,
}

impl Hint {
    pub fn new(text: String, pos: Pos) -> Self {
        Self { text, pos }
    }
}

impl<'a> std::fmt::Write for State<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        let out = &mut self.shared.borrow_mut().out;
        out.truncate(self.out_len);
        out.push_str(s);
        self.out_len = out.len();
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos(pub usize);

impl Pos {
    fn from_str(value: impl AsRef<str>) -> Self {
        Self(value.as_ref().as_ptr() as usize)
    }
}

impl<'a> State<'a> {
    fn closing_push(&mut self) {
        let mut shared = self.shared.borrow_mut();
        shared.closing.truncate(self.closing_len);
        let cur = Rc::strong_count(&shared.closing_level);
        shared.closing.push(cur);
        self.closing_len = shared.closing.len();
    }

    fn closing_flush(&mut self) {
        let mut shared = self.shared.borrow_mut();
        shared.closing.truncate(self.closing_len);
        let cur = Rc::strong_count(&shared.closing_level);

        while shared.closing.pop_if(|s| *s >= cur).is_some() {
            shared.out.truncate(self.out_len);
            shared.out.push_str("}");
            self.out_len = shared.out.len()
        }

        self.closing_len = shared.closing.len();
    }

    fn pos(&mut self) -> Pos {
        self.iter
            .peek()
            .map(Pos::from_str)
            .unwrap_or(Pos(usize::MAX))
    }

    fn update_pos(&mut self) {
        let pos = self.pos();
        let shared = &mut self.shared.borrow_mut();
        shared.max_ptr_pos = shared.max_ptr_pos.max(pos);
    }

    fn tokens(&mut self) -> String {
        let out = &mut self.shared.borrow_mut().out;
        out.truncate(self.out_len);
        out.clone()
    }
}

macro_rules! bail {
    ($($args:expr),*) => {
        { return Err(Error::NoMatch(format!($($args),*))) }
    };
}

macro_rules! bail_static {
    ($($args:tt),*) => {
        { return Err(Error::Value(format!($($args),*))) }
    }
}

#[derive(Clone, Debug)]
pub enum Error {
    NoMatch(String),
    Value(String),
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::Value(value.to_string())
    }
}

impl Error {
    fn as_str(&self) -> &str {
        match self {
            Self::NoMatch(s) => &s,
            Self::Value(s) => &s,
        }
    }
}

pub struct Parsed {
    pub stream: String,
    pub argc: usize,
}

fn parse_expr(s: &mut State<'_>, map: &Map, dont_error: bool) -> bool {
    let iter = s.iter.clone();
    let expr = iter.clone().collect::<Vec<_>>().join(" ");
    writeln!(s, "let _ = {expr:?};").unwrap();

    let res = parse_map(s, map) && s.iter.peek().is_none();

    if !res {
        let pos = s.pos();
        let mut shared = s.shared.borrow_mut();
        let last_pos = shared.max_ptr_pos;
        let mut options = String::new();

        if let Some(set) = shared.tried_literals.get(&pos.max(last_pos))
            && !set.is_empty()
            && let Some(last_tok) = iter.clone().find(|s| Pos::from_str(s) == last_pos)
        {
            let mut hints: Vec<_> = set.iter().cloned().collect();
            sort_trig_ascii(&mut hints, last_tok);
            let hint = hints.join(", ");
            options = format!(". Did you mean any of: {hint}?");
        }

        if pos < last_pos
            && !shared.errs.iter().any(|e| e.pos == last_pos)
            && !shared.warns.iter().any(|e| e.pos == last_pos)
            && let Some(last_tok) = iter.clone().find(|s| Pos::from_str(s) == last_pos)
        {
            shared.warns.push(Hint::new(
                format!("Didn't match {last_tok:?}{options}"),
                last_pos,
            ));
            options = String::new();
        }

        if let Some(tok) = s.iter.peek() {
            let out = if dont_error {
                &mut shared.warns
            } else {
                &mut shared.errs
            };

            out.push(Hint::new(
                format!("Didn't match token {tok:?}{options}"),
                pos,
            ));
        }
    }

    res
}

pub fn parse(val: &str) -> Parsed {
    let (tokens, _, command) = get_command(val);

    #[cfg(feature = "coverage")]
    {
        let prefix = command.prefix.replace("/", "_").replace(" ", "_");
        let file = format!("/tmp/code_all_{prefix}");
        let mut f = std::fs::File::options()
            .write(true)
            .read(true)
            // .truncate(false)
            .truncate(true)
            .append(true)
            .create(true)
            .open(file)
            .unwrap();
        f.lock().unwrap();
        use std::io::Write;
        command.syntax.visit(&mut |p| match p {
            Map::Code(code) => writeln!(f, "{code:?}").unwrap(),
            _ => {}
        });
    }

    // eprintln!("{tokens:?}");

    let iter = tokens.iter().cloned().peekable();
    let mut s = State {
        shared: Default::default(),
        closing_len: 0,
        out_len: 0,
        argc: 0,
        iter: iter.clone(),
    };

    let shared = s.shared.clone();
    scopeguard::defer_on_unwind! {
        let shared = shared.borrow().clone();
        for err in shared.errs.iter().rev() {
            eprintln!("Parsing error: {}", err.text);
        }
        for warn in shared.warns.iter().rev() {
            eprintln!("Parsing note: {}", warn.text);
        }
    }

    parse_expr(&mut s, command.syntax, false);

    if !shared.borrow().errs.is_empty() {
        let shared = shared.borrow().clone();
        for Hint { text, .. } in shared.errs.iter().rev() {
            writeln!(s, "compile_error!({text:?});").unwrap();
        }

        let matched_pos = s.pos();
        for Hint { text, pos, .. } in shared.warns.iter().rev() {
            if *pos < matched_pos {
                continue;
            }
            let err = format!("Note: {text}");
            writeln!(s, "compile_error!({err:?});").unwrap();
        }
    }

    let stream = s.tokens();

    // println!();
    // println!("OUT:");
    // println!("{stream}");
    // println!("END");

    Parsed {
        stream,
        argc: s.argc,
    }
}

/// Parse "[?<cond>: ... :]"
pub fn parse_cond(s: &mut State, map: &Map) -> bool {
    let tok_cond = [
        ("[?", ":][:", ":]"),
        ("{?", ":}{:", ":}"),
        ("(?", ":)(:", ":)"),
    ];
    if !matches!(map, Map::Val(_))
        && let Some(tok) = s.iter.peek().cloned()
        && let Some((tok_if, tok_else, tok_end)) = tok_cond.iter().find(|s| tok.starts_with(s.0))
        && let Some(cond) = tok.strip_prefix(tok_if)
        && let Some(cond) = cond.strip_suffix(":")
    {
        let old = s.clone();

        s.iter.next();

        let mut subexpr = Vec::new();
        let mut subexpr_else = Vec::new();
        let mut seen_else = false;
        loop {
            let Some(tok) = s.iter.next() else {
                panic!("Did't see closing {:?}", tok_if);
            };
            if tok == *tok_end {
                break;
            } else if tok == *tok_else && !seen_else {
                seen_else = true;
            } else if seen_else {
                subexpr_else.push(tok);
            } else {
                subexpr.push(tok);
            }
        }

        let mut new_s = State {
            iter: subexpr.iter().cloned().peekable(),
            ..s.clone()
        };

        let _level = s.shared.borrow().closing_level.clone();

        writeln!(new_s, "if {cond} {{").unwrap();
        let mut is_ok = parse_expr(&mut new_s, map, true);
        new_s.closing_flush();

        if seen_else {
            writeln!(new_s, "}} else {{").unwrap();
            new_s.iter = subexpr_else.iter().cloned().peekable();
            is_ok &= parse_expr(&mut new_s, map, true);
            new_s.closing_flush();
        }

        writeln!(new_s, "}}").unwrap();

        if is_ok {
            *s = State {
                iter: s.iter.clone(),
                ..new_s
            };
            return true;
        }

        *s = old;
    }

    false
}

pub fn parse_map(s: &mut State, map: &Map) -> bool {
    while parse_cond(s, map) {}

    s.update_pos();
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
            let _level = s.shared.borrow().closing_level.clone();
            let old = s.clone();
            for m in any.iter() {
                let last_pos = s.pos();
                if parse_map(s, m) && s.pos() != last_pos {
                    s.closing_flush();
                    return true;
                }
                *s = old.clone();
            }
            false
        }
        Map::May(m) => {
            let _level = s.shared.borrow().closing_level.clone();
            let old = s.clone();
            if !parse_map(s, m) {
                *s = old.clone();
            }
            s.closing_flush();
            true
        }
        Map::Star(m) => {
            let _level = s.shared.borrow().closing_level.clone();
            while s.iter.peek().is_some() {
                while parse_cond(s, map) {}
                let last_pos = s.pos();
                let old = s.clone();
                if !parse_map(s, m) {
                    *s = old.clone();
                    break;
                }
                s.closing_flush();
                if s.pos() == last_pos {
                    panic!("Stuck on token {:?}", s.iter.peek().unwrap());
                }
            }
            s.closing_flush();
            true
        }
        Map::Plus(m) => {
            let _level = s.shared.borrow().closing_level.clone();
            let old = s.clone();
            if !parse_map(s, m) {
                *s = old;
                return false;
            }
            s.closing_flush();
            while s.iter.peek().is_some() {
                while parse_cond(s, m) {}

                let last_pos = s.pos();
                let old = s.clone();
                if !parse_map(s, m) {
                    *s = old.clone();
                    break;
                }
                s.closing_flush();
                if s.pos() == last_pos {
                    panic!("Stuck on token {:?}", s.iter.peek().unwrap());
                }
            }
            s.closing_flush();
            true
        }
        Map::Val(val) => {
            let mut old = s.clone();
            match parse_val(s, val) {
                Ok(()) => return true,
                Err(Error::Value(err)) => {
                    s.shared.borrow_mut().warns.push(Hint::new(err, old.pos()))
                }
                Err(_) => {}
            }
            *s = old;
            false
        }
        Map::Code(code) => {
            #[cfg(feature = "coverage")]
            {
                let file = format!("/tmp/code_used");
                let mut f = std::fs::File::options()
                    .write(true)
                    .read(true)
                    .truncate(false)
                    .append(true)
                    .create(true)
                    .open(file)
                    .unwrap();
                f.lock().unwrap();
                use std::io::Write;
                writeln!(f, "{:?}", code).unwrap();
            }

            writeln!(s, "{code}").unwrap();
            true
        }
        Map::Last => s.iter.peek().is_none(),
        Map::AssertLast => {
            if let Some(tok) = s.iter.peek() {
                s.shared.borrow_mut().errs.push(Hint::new(
                    format!("Unexpected attribute: {tok:?}"),
                    Pos::from_str(tok),
                ));
                return false;
            }
            true
        }
        Map::Lit(lit) => {
            let pos = s.pos();
            s.shared
                .borrow_mut()
                .tried_literals
                .entry(pos)
                .or_default()
                .insert(lit.to_string());

            if s.iter.peek().is_none_or(|tok| tok != lit) {
                return false;
            }
            s.iter.next();
            true
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ValSubt<'a> {
    is_static: bool,
    name: String,
    tok: String,
    ty: &'a str,
}

fn is_num(ty: &str) -> bool {
    matches!(ty, "u8" | "u16" | "u32" | "u64")
}

fn is_true(x: &str) -> bool {
    x == "on" || bool::from_str(x).is_ok_and(|x| x == true)
}

fn is_false(x: &str) -> bool {
    x == "off" || bool::from_str(x).is_ok_and(|x| x == false)
}

pub fn parse_val(s: &mut State, val: &Val) -> Result<(), Error> {
    let mut tok = match s.iter.peek() {
        Some(tok) => *tok,
        None => bail!("No tokens left"),
    };

    let Val {
        name, ty, flags, ..
    } = *val;

    #[cfg(test)]
    eprintln!("tok={tok:?} name={name:?} ty={ty:?}");

    let mut is_tuple = false;

    let mut vals: Vec<ValSubt<'_>> = Vec::new();

    if ty == "IpNet" && flags.is_empty() && (tok.contains("/{") || tok.contains("}/")) {
        let val = Val {
            ty: "(IpAddr, u8)",
            flags: &["format:{}/{}"],
            ..val.clone()
        };
        parse_val(s, &val)?;
        let val = format!("IpNet::new_assert({name}.0, {name}.1)");
        writeln!(s, "let {name}: {ty} = {val};").unwrap();
        return Ok(());
    } else if ty.starts_with("(") && ty.ends_with(")") {
        is_tuple = true;

        let fmt = flags
            .iter()
            .find_map(|f| f.strip_prefix("format:"))
            .expect("Tuple type must have flags \"format:...\"");

        #[cfg(test)]
        eprintln!("Matching fmt={:?} against tok={:?}", fmt, tok);

        let mut tok_iter = tok.chars().peekable();
        let mut fmt_iter = fmt.chars().peekable();
        let mut ty_iter = ty
            .trim_matches(['(', ')'])
            .split(",")
            .map(str::trim_ascii)
            .peekable();

        if let Err(err) = parse_opt(&mut fmt_iter, &mut tok_iter, &mut ty_iter, &mut vals, 0) {
            let err = err.as_str();
            bail_static!("Can't parse {tok:?} as format {fmt:?} {err}");
        }

        if tok_iter.peek().is_some() {
            bail!("Unmatched characters {:?}", tok_iter.collect::<String>());
        }

        #[cfg(test)]
        eprintln!("Matched");

        assert_ne!(vals.len(), 0);
    } else {
        let mut is_static = true;
        if let Some(n) = tok.strip_prefix("{")
            && let Some(n) = n.strip_suffix("}")
        {
            is_static = false;
            tok = n;
        }

        vals.push(ValSubt {
            is_static,
            name: name.into(),
            tok: tok.into(),
            ty,
        });
    }

    if is_tuple {
        write!(s, "let {name}: {ty} = (").unwrap();
        for val in vals.iter() {
            let val = parse_subt(val, flags, s)?;
            write!(s, "{val}, ").unwrap();
        }
        writeln!(s, ");").unwrap();
    } else {
        assert_eq!(vals.len(), 1);
        let val = &vals[0];
        let val = parse_subt(val, flags, s)?;
        writeln!(s, "let {name}: {ty} = {val};").unwrap();
    }

    s.iter.next();

    Ok(())
}

fn parse_opt<'a, CI, SI>(
    fmt_iter: &mut Peekable<CI>,
    tok_iter: &mut Peekable<CI>,
    ty_iter: &mut Peekable<SI>,
    vals: &mut Vec<ValSubt<'a>>,
    _depth: usize,
) -> Result<(), Error>
where
    CI: Iterator<Item = char> + Clone + Debug,
    SI: Iterator<Item = &'a str> + Clone + Debug,
{
    let is_pat = |c: char| c.is_ascii_digit() || c == '{';
    let get_pat = |c: char, fmt_iter: &mut Peekable<CI>, vals: &mut Vec<ValSubt<'a>>| {
        if c == '{' {
            while let Some(c) = fmt_iter.next() {
                if c == '}' {
                    break;
                }
            }
            Ok(vals.len() as u8)
        } else {
            c.to_string().parse::<u8>()
        }
    };

    loop {
        #[cfg(test)]
        eprintln!("{_depth}: fmt: {:?}", fmt_iter.clone().collect::<String>());
        #[cfg(test)]
        eprintln!("{_depth}: tok: {:?}", tok_iter.clone().collect::<String>());

        if fmt_iter.peek() == Some(&']') {
            break;
        }

        let Some(c) = fmt_iter.next() else {
            break;
        };

        if c == '[' {
            let old_tok = tok_iter.clone();
            let old_fmt = fmt_iter.clone();
            let old_ty = ty_iter.clone();

            #[cfg(test)]
            dbg!(&tok_iter);
            let res = parse_opt(fmt_iter, tok_iter, ty_iter, vals, _depth + 1);
            if res.is_ok() {
                if fmt_iter.next() != Some(']') {
                    bail!("Didn't see terminating ']'");
                };
                continue;
            }

            *fmt_iter = old_fmt;
            *tok_iter = old_tok;
            *ty_iter = old_ty;
            let mut s = 1;
            while let Some(c) = fmt_iter.next() {
                #[cfg(test)]
                dbg!(s, c);
                if is_pat(c) {
                    let c = get_pat(c, fmt_iter, vals)?;
                    let name = format!("_tuple_val{c}");
                    let tok = format!("None");
                    vals.push(ValSubt {
                        is_static: true,
                        name,
                        tok,
                        ty: ty_iter.next().unwrap(),
                    });
                } else if c == '[' {
                    s += 1;
                } else if c == ']' {
                    s -= 1;
                }
                if s == 0 {
                    break;
                }
            }
            continue;
        } else if is_pat(c) {
            let c = get_pat(c, fmt_iter, vals)?;

            let mut id = String::new();
            let is_static;
            if tok_iter.peek() == Some(&'{') {
                tok_iter.next();
                is_static = false;
                loop {
                    let Some(n) = tok_iter.next() else {
                        bail!("Didn't see terminating '}}'");
                    };
                    if n == '}' {
                        break;
                    }
                    id.push(n);
                }
            } else {
                is_static = true;
                let mut opt = false;
                let sep: Vec<_> = fmt_iter
                    .clone()
                    .filter(|sep| {
                        if ['[', ']'].contains(sep) {
                            opt = true;
                            false
                        } else {
                            !sep.is_ascii_digit() && !['{', '}'].contains(sep)
                        }
                    })
                    .collect();
                if !sep.is_empty() {
                    loop {
                        let Some(&n) = tok_iter.peek() else {
                            if opt {
                                break;
                            }
                            bail!("Didn't see separating {sep:?}");
                        };
                        if sep.contains(&n) {
                            break;
                        }
                        tok_iter.next();
                        id.push(n);
                    }
                } else {
                    while let Some(n) = tok_iter.next() {
                        id.push(n);
                    }
                }
            }

            let name = format!("_tuple_val{c}");
            vals.push(ValSubt {
                is_static,
                name,
                tok: id,
                ty: ty_iter.next().unwrap(),
            });
        } else if tok_iter.peek() == Some(&c) {
            tok_iter.next();
        } else {
            bail!("Unmatched character {c:?}");
        }
    }

    Ok(())
}

fn parse_subt(v: &ValSubt, flags: &[&str], s: &mut State<'_>) -> Result<String, Error> {
    let ValSubt {
        is_static, tok, ty, ..
    } = v;
    let ty = *ty;

    if flags.contains(&"static-str") && ty != "&str" {
        unimplemented!("Value has static-str flag, but its type is not &str: {ty:?}");
    }

    if *is_static && flags.contains(&"static-str") && !ty.as_bytes()[0].is_ascii_alphabetic() {
        bail!("Expected static &str");
    }

    if !*is_static && (flags.contains(&"static") || flags.contains(&"static-str")) {
        bail!("Expected static");
    }

    if let Some(ty) = ty.strip_prefix("Option<")
        && let Some(ty) = ty.strip_suffix(">")
    {
        if *is_static && tok == "None" {
            return Ok(format!("None"));
        } else {
            let mut v = v.clone();
            v.ty = ty;
            return parse_subt(&v, flags, s).map(|t| format!("Some({t})"));
        }
    }

    let res = if *is_static {
        let val = match ty {
            "IpAddr" => match tok.parse::<IpAddr>() {
                Ok(n) => addr_to_tok(n),
                Err(err) => bail_static!("Can't parse {tok:?} as {ty}: {err}"),
            },
            "Ipv4Addr" => match tok.parse::<Ipv4Addr>() {
                Ok(ipv4) => format!("Ipv4Addr::from_octets({:?})", ipv4.octets()),
                Err(err) => bail_static!("Can't parse {tok:?} as {ty}: {err}"),
            },
            "IpNet" if !tok.contains('/') => match tok.parse::<IpAddr>() {
                Ok(n) => addr_as_ipnet_to_tok(n),
                Err(err) => bail_static!("Can't parse {tok:?} as {ty}: {err}"),
            },
            "IpNet" => match tok.parse::<IpNet>() {
                Ok(n) => ipnet_to_tok(n),
                Err(err) => bail_static!("Can't parse {tok:?} as {ty}: {err}"),
            },
            _ if is_num(ty) && flags.contains(&"hex") => {
                let Some(tok) = tok.strip_prefix("0x") else {
                    let err = format!("A hexadecimal {tok:?} is missing {:?} prefix", "0x");
                    s.shared
                        .borrow_mut()
                        .errs
                        .push(Hint::new(err.clone(), Pos::from_str(tok)));
                    bail_static!("{err}");
                };
                match u64::from_str_radix(tok, 16) {
                    Ok(n) => format!("{n}"),
                    Err(err) => bail_static!("Can't parse {tok:?} as {ty}: {err}"),
                }
            }
            _ if is_num(ty) && flags.contains(&"time-ms") => {
                let mut raw = false;
                let mut res = if let Some(tok) = tok.strip_suffix("ms") {
                    tok.parse::<u32>()
                } else if let Some(tok) = tok.strip_suffix("s") {
                    tok.parse::<u32>().map(|s| s * 1000)
                } else {
                    raw = true;
                    tok.parse::<u32>()
                };

                if !raw && flags.contains(&"mul8-suffix") {
                    res = res.map(|r| r * 8);
                }

                if !raw && flags.contains(&"mul4-suffix") {
                    res = res.map(|r| r * 4);
                }

                match res {
                    Ok(n) => format!("{n}"),
                    Err(err) => bail_static!("Can't parse {tok:?} as {ty}: {err}"),
                }
            }
            _ if is_num(ty)
                && let Some(tok) = tok.strip_prefix("0x") =>
            {
                match u64::from_str_radix(tok, 16) {
                    Ok(n) => format!("{n}"),
                    Err(err) => bail_static!("Can't parse {tok:?} as {ty}: {err}"),
                }
            }
            _ if is_num(ty) && is_true(tok) => format!("1"),
            _ if is_num(ty) && is_false(tok) => format!("0"),
            _ if is_num(ty) => match tok.parse::<u64>() {
                Ok(n) => format!("{n}"),
                Err(err) => bail_static!("Can't parse {tok:?} as {ty}: {err}"),
            },
            "bool" => if let Ok(v) = tok.parse::<u8>() {
                v > 0
            } else if let Ok(v) = tok.parse::<bool>() {
                v
            } else {
                bail_static!("Can't parse {tok:?} as {ty}");
            }
            .to_string(),
            "&[u8]" if flags.contains(&"mac") => {
                let mut str = format!("&[");
                let mut n = 0;
                for byte in tok.split(":") {
                    match u64::from_str_radix(byte, 16) {
                        Ok(n) => write!(str, "0x{n:x}, ").unwrap(),
                        Err(err) => bail_static!("Can't parse {tok:?} as mac address: {err}"),
                    }
                    n += 1;
                }
                if n != 6 {
                    bail_static!("Can't parse {tok:?} as mac address");
                }
                write!(str, "]").unwrap();
                str
            }
            "&str" => quote! { #tok }.to_string(),
            _ => unimplemented!("Unknown type {ty:?}"),
        };

        format!("{val}")
    } else {
        let (tok, flag) = tok.split_once(":").unwrap_or((tok, ""));

        let mut tok = format!("{tok}");
        if tok.is_empty() {
            let i = s.argc;
            s.argc += 1;
            tok = format!("arg{i}");
        }

        match flag {
            "" => {}
            "opt" | "option" | "optional" => {
                writeln!(s, "if let Some({tok}) = {tok} {{").unwrap();
                s.closing_push();
            }
            _ => panic!("Unknown flag {flag:?}"),
        }

        tok
    };

    Ok(res)
}

fn addr_to_tok(addr: IpAddr) -> String {
    match addr {
        IpAddr::V4(ipv4) => format!("IpAddr::V4(Ipv4Addr::from_octets({:?}))", ipv4.octets()),
        IpAddr::V6(ipv6) => format!("IpAddr::V6(Ipv6Addr::from_octets({:?}))", ipv6.octets()),
    }
}

fn addr_as_ipnet_to_tok(addr: IpAddr) -> String {
    format!(
        "IpNet::new_assert({}, {})",
        addr_to_tok(addr),
        match addr {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        },
    )
}

fn ipnet_to_tok(net: IpNet) -> String {
    format!(
        "IpNet::new_assert({}, {})",
        addr_to_tok(net.addr()),
        net.prefix_len()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_val() {
        let ty = "(u8, u8, IpAddr, IpAddr, Option<IpAddr>, Option<IpAddr>, Option<IpAddr>)";
        let fmt = "0-1[-2][-3][-4][-5][-6]";
        let tok = "10-{a}-1.2.0.0-{b}-{}-4.0.0.0";
        let flags = &[][..];

        let mut tok_iter = tok.chars().peekable();
        let mut fmt_iter = fmt.chars().peekable();
        let mut ty_iter = ty
            .trim_matches(['(', ')'])
            .split(",")
            .map(str::trim_ascii)
            .peekable();

        let mut vals = Vec::new();
        parse_opt(&mut fmt_iter, &mut tok_iter, &mut ty_iter, &mut vals, 0).unwrap();

        dbg!(&vals);
        assert_eq!(
            &vals[..],
            &[
                ValSubt {
                    is_static: true,
                    name: "_tuple_val0".into(),
                    tok: "10".into(),
                    ty: "u8",
                },
                ValSubt {
                    is_static: false,
                    name: "_tuple_val1".into(),
                    tok: "a".into(),
                    ty: "u8",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val2".into(),
                    tok: "1.2.0.0".into(),
                    ty: "IpAddr",
                },
                ValSubt {
                    is_static: false,
                    name: "_tuple_val3".into(),
                    tok: "b".into(),
                    ty: "IpAddr",
                },
                ValSubt {
                    is_static: false,
                    name: "_tuple_val4".into(),
                    tok: "".into(),
                    ty: "Option<IpAddr>",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val5".into(),
                    tok: "4.0.0.0".into(),
                    ty: "Option<IpAddr>",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val6".into(),
                    tok: "None".into(),
                    ty: "Option<IpAddr>",
                },
            ][..],
        );

        let toks = [];
        let s = &mut State {
            shared: Default::default(),
            closing_len: Default::default(),
            out_len: Default::default(),
            argc: Default::default(),
            iter: toks.iter().cloned().peekable(),
        };

        let mut iter = vals.iter();
        let mut n = || dbg!(parse_subt(iter.next().unwrap(), flags, s).unwrap());

        assert_eq!(n(), "10");
        assert_eq!(n(), "a");
        assert_eq!(n(), "IpAddr::V4(Ipv4Addr::from_octets([1, 2, 0, 0]))");
        assert_eq!(n(), "b");
        assert_eq!(n(), "Some(arg0)");
        assert_eq!(n(), "Some(IpAddr::V4(Ipv4Addr::from_octets([4, 0, 0, 0])))");
        assert_eq!(n(), "None");

        for val in &vals {
            let val = parse_subt(val, flags, s).unwrap();
            dbg!(val);
        }
    }

    #[test]
    fn test_val_recursing() {
        let ty = "(Option<u8>, Option<u8>, u8)";
        let fmt = "[{}[-{}]]-{}";
        let tok = "10-11-12";

        let mut tok_iter = tok.chars().peekable();
        let mut fmt_iter = fmt.chars().peekable();
        let mut ty_iter = ty
            .trim_matches(['(', ')'])
            .split(",")
            .map(str::trim_ascii)
            .peekable();

        let mut vals = Vec::new();
        parse_opt(&mut fmt_iter, &mut tok_iter, &mut ty_iter, &mut vals, 0).unwrap();

        dbg!(&vals);
        assert_eq!(
            &vals[..],
            &[
                ValSubt {
                    is_static: true,
                    name: "_tuple_val0".into(),
                    tok: "10".into(),
                    ty: "Option<u8>",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val1".into(),
                    tok: "11".into(),
                    ty: "Option<u8>",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val2".into(),
                    tok: "12".into(),
                    ty: "u8",
                },
            ][..],
        );
    }

    #[test]
    fn test_val2() {
        let ty = "(u32, u32, Option<u32>, Option<u32>, u32)";
        let fmt = "0/1[-2[-3]]/4";
        let tok = "10/20/40";

        let mut tok_iter = tok.chars().peekable();
        let mut fmt_iter = fmt.chars().peekable();
        let mut ty_iter = ty
            .trim_matches(['(', ')'])
            .split(",")
            .map(str::trim_ascii)
            .peekable();

        let mut vals = Vec::new();
        parse_opt(&mut fmt_iter, &mut tok_iter, &mut ty_iter, &mut vals, 0).unwrap();

        dbg!(&vals);
        assert_eq!(
            &vals[..],
            &[
                ValSubt {
                    is_static: true,
                    name: "_tuple_val0".into(),
                    tok: "10".into(),
                    ty: "u32",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val1".into(),
                    tok: "20".into(),
                    ty: "u32",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val2".into(),
                    tok: "None".into(),
                    ty: "Option<u32>",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val3".into(),
                    tok: "None".into(),
                    ty: "Option<u32>",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val4".into(),
                    tok: "40".into(),
                    ty: "u32",
                },
            ][..],
        );

        let toks = [];
        let flags = &[][..];
        let s = &mut State {
            shared: Default::default(),
            closing_len: Default::default(),
            out_len: Default::default(),
            argc: Default::default(),
            iter: toks.iter().cloned().peekable(),
        };

        let mut iter = vals.iter();
        let mut n = || dbg!(parse_subt(iter.next().unwrap(), flags, s).unwrap());

        assert_eq!(n(), "10");
        assert_eq!(n(), "20");

        for val in &vals {
            let val = parse_subt(val, flags, s).unwrap();
            dbg!(val);
        }
    }

    #[test]
    fn test_val3() {
        let ty = "(u32, Option<u32>)";
        let fmt = "{}[/{}]";
        let tok = "0xff";

        let mut tok_iter = tok.chars().peekable();
        let mut fmt_iter = fmt.chars().peekable();
        let mut ty_iter = ty
            .trim_matches(['(', ')'])
            .split(",")
            .map(str::trim_ascii)
            .peekable();

        let mut vals = Vec::new();
        parse_opt(&mut fmt_iter, &mut tok_iter, &mut ty_iter, &mut vals, 0).unwrap();

        dbg!(&vals);
        assert_eq!(
            &vals[..],
            &[
                ValSubt {
                    is_static: true,
                    name: "_tuple_val0".into(),
                    tok: "0xff".into(),
                    ty: "u32",
                },
                ValSubt {
                    is_static: true,
                    name: "_tuple_val1".into(),
                    tok: "None".into(),
                    ty: "Option<u32>",
                },
            ][..],
        );

        let toks = [];
        let flags = &[][..];
        let s = &mut State {
            shared: Default::default(),
            closing_len: Default::default(),
            out_len: Default::default(),
            argc: Default::default(),
            iter: toks.iter().cloned().peekable(),
        };

        let mut iter = vals.iter();
        let mut n = || dbg!(parse_subt(iter.next().unwrap(), flags, s).unwrap());

        assert_eq!(n(), "255");
        assert_eq!(n(), "None");

        for val in &vals {
            let val = parse_subt(val, flags, s).unwrap();
            dbg!(val);
        }
    }
}
