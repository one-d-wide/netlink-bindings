use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    process::{Child, Command},
    time::Duration,
};

use quote::format_ident;
use syn::Ident;

use crate::{
    parse_spec::{AttrProp, AttrSet, AttrType, Spec, SubMessage},
    Context, WARNING,
};

pub fn sanitize_ident(name: &str) -> Ident {
    let keywords = ["self"];
    match name {
        name if name.chars().next().unwrap().is_ascii_digit() => {
            format_ident!("_{name}")
        }
        name if keywords.iter().any(|k| k.eq_ignore_ascii_case(name)) => {
            format_ident!("_{name}")
        }
        _ => format_ident!("{name}"),
    }
}

#[derive(Debug)]
pub struct Pandoc {
    addr: SocketAddr,
    proc: Child,
}

impl Drop for Pandoc {
    fn drop(&mut self) {
        let _ = self.proc.kill();
    }
}

pub fn escape_md(ctx: &mut Context, text: &str) -> String {
    if ctx.args.no_pandoc {
        let mut res = String::new();
        for c in text.chars() {
            if "\\`~*_{}[]#+-.".contains(c) {
                res.push('\\');
            }
            res.push(c);
        }
        return res;
    }

    let req = serde_json::json!({
        "text": text,
        "from": "rst",
        "to": "markdown",
        // Pandoc server has a bug on platforms disabling baked-in data files
        "files": {"data/data/abbreviations": ""},
    });
    let body = serde_json::to_string(&req).unwrap();
    let body_len = body.len();

    let req = format!(
        "\
POST / HTTP/1.0\r
Content-Type: application/json\r
Accept: application/json\r
Content-Length: {body_len}\r
\r
{body}"
    );

    let mut resp = String::new();
    let addr = spawn_pandoc_server(ctx);
    let mut conn = TcpStream::connect(addr).expect("Can't connect to `pandoc` server");
    conn.write_all(req.as_bytes()).unwrap();
    conn.read_to_string(&mut resp).unwrap();

    if !resp.starts_with("HTTP/1.0 200") {
        let err = resp.lines().next().unwrap_or("").strip_prefix("HTTP/1.0 ");
        panic!("Pandoc server returned error: {err:?}");
    }

    let Some(delim) = resp.find("\r\n\r\n") else {
        panic!("Pandoc server returned error: {resp:?}");
    };

    #[derive(serde::Deserialize)]
    struct Message {
        verbosity: String,
        message: String,
    }
    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    enum Resp {
        Ok {
            output: String,
            messages: Vec<Message>,
        },
        Err {
            error: String,
        },
    }
    let resp = &resp[delim..];
    match serde_json::from_str::<Resp>(resp) {
        Err(err) => panic!("Can't parse pandoc response: {err:?}\nResponse: {resp:?}"),
        Ok(Resp::Err { error }) => panic!("Pandoc server returned error: {error:?}"),
        Ok(Resp::Ok { output, messages }) => {
            for Message { verbosity, message } in messages {
                // TODO: seems like a problem for the kernel
                // TODO: Do we really need this?
                println!("{WARNING} {verbosity} from pandoc server: {message}");
                println!("{WARNING} While converting ReST -> Markdown:");
                for line in text.lines() {
                    println!("{WARNING}   {line}");
                }
            }
            output
        }
    }
}

pub fn spawn_pandoc_server(ctx: &mut Context) -> SocketAddr {
    if let Some(Pandoc { addr, .. }) = &ctx.pandoc {
        return *addr;
    }

    let addr = {
        let local: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let sock = TcpListener::bind(local).unwrap();
        sock.local_addr().unwrap()
    };

    let res = Command::new("pandoc")
        .args(["server", "--port", &format!("{}", addr.port())])
        .spawn();
    if let Err(err) = res {
        let err = format!("Error starting `pandoc` server: {err}");
        println!("{WARNING} {err}");
        println!("{WARNING} We use pandoc to convert documentation from ReST to Markdown");
        println!("{WARNING} Pass --no-pandoc to disable special formatting in doc strings");
        panic!("{err}");
    };
    let mut proc = res.unwrap();

    while proc.try_wait().unwrap().is_none() && TcpStream::connect(addr).is_err() {
        std::thread::sleep(Duration::from_millis(10));
    }

    ctx.pandoc = Some(Pandoc { addr, proc });
    addr
}

pub fn kebab_to_rust(name: &str) -> String {
    let res = name
        .chars()
        .map(|c| match c {
            '-' | ' ' => '_',
            c => c,
        })
        .collect();

    res
}

pub fn kebab_to_type(name: &str) -> String {
    let mut res = String::new();
    let mut capitalize = true;
    for c in name.chars() {
        match c {
            '-' | '_' | ' ' => {
                capitalize = true;
            }
            c if capitalize => {
                capitalize = false;
                res.extend(c.to_uppercase())
            }
            c => res.push(c),
        }
    }

    res
}

pub fn kebab_to_upper(name: &str) -> String {
    let res = name
        .chars()
        .map(|c| match c {
            '-' | ' ' => '_',
            c => c.to_ascii_uppercase(),
        })
        .collect();

    res
}

pub fn doc_attr(ctx: &mut Context, attr: &AttrProp, mut write: impl FnMut(&str)) {
    let mut docs = Vec::new();
    if let Some(doc) = &attr.doc {
        docs.push(escape_md(ctx, doc));
    }

    // if let Some(checks) = &attr.checks {
    //     if let Some(mask) = &checks.flags_mask {
    //          TODO:
    //     }
    // }

    if let Some(r#enum) = &attr.r#enum {
        let comment = if let Some(true) = attr.enum_as_flags {
            "(1 bit per enumeration)"
        } else {
            "(enum)"
        };

        let enum_type = kebab_to_type(r#enum);
        docs.push(format!("Associated type: [`{enum_type}`] {comment}",));
    };

    if let Some(true) = &attr.multi_attr {
        docs.push("Attribute may repeat multiple times (treat it as array)".into());
    }

    if !docs.is_empty() {
        write(&docs.join("\n"));
    }
}

pub fn lifetime_needed_attr(attr: &AttrProp) -> bool {
    matches!(
        attr.r#type,
        AttrType::Pad { .. }
            | AttrType::String
            | AttrType::Binary { r#struct: None, .. }
            | AttrType::Nest { .. }
            | AttrType::IndexedArray { .. }
    ) && !attr.is_ipv4()
        && !attr.is_ipv6()
        && !attr.is_ip()
        && !attr.is_sockaddr()
}

pub fn lifetime_needed_attrs(attrs: &AttrSet) -> bool {
    for m in &attrs.attributes {
        if lifetime_needed_attr(m) {
            return true;
        }
    }
    false
}

pub fn lifetime_needed_sub_message(spec: &Spec, sub: &SubMessage) -> bool {
    for m in &sub.formats {
        if let Some(attrs) = &m.attribute_set {
            if lifetime_needed_attrs(spec.find_attr(attrs)) {
                return true;
            }
        }
    }
    false
}

pub const fn align_up(len: usize, alignment: usize) -> usize {
    ((len) + alignment - 1) & !(alignment - 1)
}
