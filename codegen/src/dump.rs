use std::io::Write;

use crate::{
    gen_attrs::shorthand_name,
    gen_ops, gen_sub_message,
    gen_utils::{doc_attr, kebab_to_rust, kebab_to_type},
    gen_writable::writable_type,
    parse_spec::{AttrProp, AttrSet, AttrType, IndexedArrayType, Operation, Request, Spec},
    WARNING,
};

pub fn find_attr<'a>(attrset: &'a AttrSet, attr: &str) -> Option<&'a AttrProp> {
    if let Some(attr) = attrset.attributes.iter().find(|a| *a.name == *attr) {
        Some(attr)
    } else {
        println!(
            "{WARNING} Can't find attribute {attr:?} in {:?}",
            attrset.name
        );
        None
    }
}

#[derive(Debug)]
pub enum NestIdent {
    None,
    Bracket,
    Comma,
}

#[derive(Debug, Default)]
pub struct Nest {
    pub level: Vec<(NestIdent, Option<String>)>,
}

impl Nest {
    pub fn indent(&self, out: &mut Vec<u8>) {
        for _ in 0..self.level.len() {
            write!(out, "  ").unwrap();
        }
    }

    pub fn indent_none(&mut self, out: &mut Vec<u8>) {
        self.indent(out);
        self.level.push((NestIdent::None, None));
    }

    pub fn indent_none_attrs(&mut self, out: &mut Vec<u8>, attrs: &str) {
        self.indent(out);
        self.level.push((NestIdent::None, Some(attrs.into())));
    }

    pub fn indent_advance(&mut self, out: &mut Vec<u8>) {
        self.indent(out);
        self.level.push((NestIdent::Bracket, None));
    }

    pub fn indent_comma(&mut self, out: &mut Vec<u8>) {
        self.indent(out);
        self.level.push((NestIdent::Comma, None));
    }

    pub fn visited(&self, attrs: &str) -> bool {
        self.level
            .iter()
            .take(self.level.len() - 1)
            .any(|(_, a)| a.as_ref().is_some_and(|a| a == attrs))
    }

    pub fn end(&mut self, out: &mut Vec<u8>) {
        let (is_comma, _) = self.level.pop().unwrap();
        self.indent(out);
        match is_comma {
            NestIdent::None => {}
            NestIdent::Comma => writeln!(out, "}},").unwrap(),
            NestIdent::Bracket => writeln!(out, "}}").unwrap(),
        }
    }
}

pub fn dump_string_type(attr: &AttrProp) -> String {
    match &attr.r#type {
        AttrType::Flag => "()".into(),
        AttrType::U8 => "u8".into(),
        AttrType::U16 => "u16".into(),
        AttrType::U32 if attr.is_ipv4() => "Ipv4Addr".into(),
        AttrType::U32 => "u32".into(),
        AttrType::U64 => "u64".into(),
        AttrType::S8 => "i8".into(),
        AttrType::S16 => "i16".into(),
        AttrType::S32 => "i32".into(),
        AttrType::S64 => "i64".into(),
        AttrType::Binary { .. } if attr.is_ipv6() => "Ipv6Addr".into(),
        AttrType::Binary { .. } if attr.is_ip() => "IpAddr".into(),
        AttrType::Binary { .. } if attr.is_sockaddr() => "SocketAddr".into(),
        AttrType::Binary {
            r#struct: Some(struct_type),
            ..
        } => {
            format!("{}", writable_type(struct_type))
        }
        AttrType::String => "&CStr".into(),
        AttrType::Pad { .. } | AttrType::Binary { .. } => "&[u8]".into(),
        AttrType::IndexedArray { sub_type } => match sub_type {
            IndexedArrayType::Plain { attr } => dump_string_type(attr),
            IndexedArrayType::Nest { nested_attributes } => {
                format!("{}", writable_type(nested_attributes))
            }
            other => unreachable!("{other:?}"),
        },
        AttrType::Nest { .. } => "".into(),
        AttrType::SubMessage { .. } => "submessage".into(),
        r#type => unreachable!("{:?}", r#type),
    }
}

pub fn dump_writable_attr(out: &mut Vec<u8>, n: &mut Nest, spec: &Spec, attr: &AttrProp) {
    doc_attr(attr, |doc| {
        if out.get(out.len() - 2) != Some(&b'\n') {
            writeln!(out).unwrap();
        }
        for line in doc.lines() {
            n.indent(out);
            writeln!(out, "// {line}").unwrap();
        }
    });

    match &attr.r#type {
        AttrType::Unused => {}
        AttrType::IndexedArray { sub_type } => {
            n.indent_none(out);
            writeln!(out, ".array_{}()", kebab_to_rust(&attr.name)).unwrap();
            match sub_type {
                IndexedArrayType::Plain { attr } => {
                    let rust_type = dump_string_type(attr);
                    n.indent(out);
                    writeln!(out, ".entry(val) // {rust_type}",).unwrap();
                }
                IndexedArrayType::Nest { nested_attributes } => {
                    n.indent_none_attrs(out, nested_attributes);
                    writeln!(out, ".entry_nested()").unwrap();
                    let attrset = spec.find_attr(nested_attributes);
                    for attr in &attrset.attributes {
                        dump_writable_attr(out, n, spec, attr);
                    }
                    n.end(out);
                    writeln!(out, ".end_nested()").unwrap();
                }
                other => unreachable!("{other:?}"),
            }
            n.end(out);
            writeln!(out, ".end_array()").unwrap();
        }
        AttrType::Nest { nested_attributes } => {
            n.indent_none_attrs(out, nested_attributes);
            writeln!(out, ".nested_{}()", kebab_to_rust(&attr.name)).unwrap();
            if !n.visited(nested_attributes) {
                let attrset = spec.find_attr(nested_attributes);
                for attr in &attrset.attributes {
                    dump_writable_attr(out, n, spec, attr);
                }
            } else {
                n.indent(out);
                writeln!(out, "// ...").unwrap();
            }
            n.end(out);
            writeln!(out, ".end_nested()").unwrap();
        }
        AttrType::SubMessage { sub_message, .. } => {
            let sub_message = spec.find_sub_message(sub_message);

            for sub_attr in &sub_message.formats {
                let push = gen_sub_message::sub_message_push_name(&attr.name, &sub_attr.value);

                let mut args = String::new();
                let mut comm = String::new();
                if let Some(fixed_header) = &sub_attr.fixed_header {
                    args = "fixed_header".to_string();
                    comm = format!(" // {}", writable_type(fixed_header));
                }

                if sub_attr.attribute_set.is_some() {
                    n.indent_none(out);
                } else {
                    n.indent(out);
                }
                writeln!(out, ".{}({args}){comm}", push).unwrap();
                if let Some(attrs) = &sub_attr.attribute_set {
                    let attrset = spec.find_attr(attrs);
                    for attr in &attrset.attributes {
                        dump_writable_attr(out, n, spec, attr);
                    }
                    n.end(out);
                    writeln!(out, ".end_nested()").unwrap();
                }
            }
        }
        _ => {
            let rust_type = dump_string_type(attr);

            n.indent(out);
            writeln!(
                out,
                ".push_{}(val) // {rust_type}",
                kebab_to_rust(&attr.name)
            )
            .unwrap();
        }
    }
}

pub fn dump_writable(
    out: &mut Vec<u8>,
    spec: &Spec,
    request: &Request,
    attrset: &AttrSet,
    type_name: &str,
) {
    let mut n = Nest::default();
    n.indent_none(out);
    writeln!(out, "{}::new(&mut vec)", writable_type(type_name)).unwrap();
    for attr in &request.attributes {
        let Some(attr) = find_attr(attrset, attr) else {
            continue;
        };
        dump_writable_attr(out, &mut n, spec, attr);
    }
    n.indent(out);
    writeln!(out, ";").unwrap();
}

fn dump_shorthand_attr(
    out: &mut Vec<u8>,
    n: &mut Nest,
    spec: &Spec,
    attr: &AttrProp,
    var: Option<&str>,
) {
    let iter_var = "attrs";
    let entry_var = "entry";

    let var = var.unwrap_or(iter_var);

    if let AttrType::Unused = &attr.r#type {
        return;
    }

    let name = shorthand_name(&attr.name);
    let rust_type = dump_string_type(attr);

    if let AttrType::Nest { .. } = &attr.r#type {
        n.indent_advance(out);
        writeln!(out, "{{ // Nested {}", kebab_to_type(&attr.name)).unwrap();
    }

    let mut got_docs = false;
    doc_attr(attr, |doc| {
        got_docs = true;
        if out.get(out.len() - 2) != Some(&b'\n') {
            writeln!(out).unwrap();
        }
        for line in doc.lines() {
            n.indent(out);
            writeln!(out, "// {line}").unwrap();
        }
    });

    match &attr.r#type {
        _ if attr.multi_attr.unwrap_or(false) => {
            if !got_docs {
                writeln!(out).unwrap();
            }
            n.indent_advance(out);
            writeln!(out, "for {entry_var} in {var}.{name}() {{").unwrap();

            match &attr.r#type {
                AttrType::Nest { nested_attributes } => {
                    let attrs = spec.find_attr(nested_attributes);
                    for attr in &attrs.attributes {
                        dump_shorthand_attr(out, n, spec, attr, Some(entry_var));
                    }
                }
                _ => {
                    n.indent(out);
                    writeln!(out, "{entry_var}; // {rust_type}").unwrap();
                }
            };

            n.end(out);
            if matches!(attr.r#type, AttrType::Nest { .. }) {
                n.end(out);
            }
        }
        AttrType::Nest { nested_attributes } => {
            n.indent(out);
            writeln!(out, "let {iter_var} = {var}.{name}();").unwrap();

            let attrset = spec.find_attr(nested_attributes);
            for attr in &attrset.attributes {
                dump_shorthand_attr(out, n, spec, attr, Some(iter_var));
            }

            n.end(out);
        }
        AttrType::IndexedArray { sub_type } => {
            if !got_docs {
                writeln!(out).unwrap();
            }
            n.indent_advance(out);
            writeln!(out, "for {entry_var} in {var}.{name}() {{").unwrap();

            match sub_type {
                IndexedArrayType::U32 => {
                    dump_shorthand_attr(out, n, spec, attr, Some(entry_var));
                }
                IndexedArrayType::Nest { nested_attributes } => {
                    let attrset = spec.find_attr(nested_attributes);
                    for attr in &attrset.attributes {
                        dump_shorthand_attr(out, n, spec, attr, Some(entry_var));
                    }
                }
                other => unreachable!("{other:?}"),
            }

            n.end(out);
        }
        _ => {
            n.indent(out);
            writeln!(out, "{var}.{name}(); // {rust_type}").unwrap();
        }
    }
}

pub fn dump_shorthand(
    out: &mut Vec<u8>,
    spec: &Spec,
    request: &Request,
    attrset: &AttrSet,
    op_type: &str,
) {
    let mut n = Nest { level: Vec::new() };

    writeln!(out, "let attrs = {op_type}::new(buf);").unwrap();
    writeln!(out).unwrap();

    if request.attributes.is_empty() {
        writeln!(out, "// No attributes").unwrap();
        return;
    }

    for attr in &request.attributes {
        let Some(attr) = find_attr(attrset, attr) else {
            continue;
        };
        dump_shorthand_attr(out, &mut n, spec, attr, None);
    }

    assert!(n.level.is_empty());
}

fn dump_readable_attr(out: &mut Vec<u8>, n: &mut Nest, spec: &Spec, attr: &AttrProp) {
    let iter_var = "iter";
    let attr_var = "attr";
    let entry_var = "entry";

    let name = kebab_to_type(&attr.name);

    doc_attr(attr, |doc| {
        if out.get(out.len() - 2) != Some(&b'\n') {
            writeln!(out).unwrap();
        }
        for line in doc.lines() {
            n.indent(out);
            writeln!(out, "// {line}").unwrap();
        }
    });

    match &attr.r#type {
        AttrType::Unused => {}
        AttrType::Nest { nested_attributes } => {
            n.indent_comma(out);
            writeln!(out, "{name}({iter_var}) => {{").unwrap();

            n.indent_advance(out);
            writeln!(out, "for {attr_var} in {iter_var} {{").unwrap();

            n.indent_advance(out);
            writeln!(out, "match {attr_var} {{").unwrap();

            let attrset = spec.find_attr(nested_attributes);
            for attr in &attrset.attributes {
                dump_readable_attr(out, n, spec, attr);
            }

            n.end(out);
            n.end(out);
            n.end(out);
        }
        AttrType::IndexedArray { sub_type } => {
            n.indent_comma(out);
            writeln!(out, "{name}({iter_var}) => {{").unwrap();

            n.indent_advance(out);
            writeln!(out, "for {entry_var} in {iter_var} {{").unwrap();
            match sub_type {
                IndexedArrayType::Plain { attr } => {
                    dump_readable_attr(out, n, spec, attr);
                }
                IndexedArrayType::Nest { nested_attributes } => {
                    n.indent_advance(out);
                    writeln!(out, "for {attr_var} in {entry_var} {{").unwrap();

                    n.indent_advance(out);
                    writeln!(out, "match {attr_var} {{").unwrap();

                    let attrset = spec.find_attr(nested_attributes);
                    for attr in &attrset.attributes {
                        dump_readable_attr(out, n, spec, attr);
                    }

                    n.end(out);
                    n.end(out);
                }
                other => unreachable!("{other:?}"),
            }

            n.end(out);
            n.end(out);
        }
        _ => {
            let rust_type = dump_string_type(attr);

            n.indent(out);
            writeln!(out, "{name}(val) => {{}}, // {rust_type}",).unwrap();
        }
    }
}

pub fn dump_readable(
    out: &mut Vec<u8>,
    spec: &Spec,
    request: &Request,
    attrset: &AttrSet,
    op_type: &str,
) {
    let iter_var = "iter";
    let attr_var = "attr";

    let mut n = Nest { level: Vec::new() };

    writeln!(out, "let {iter_var} = {op_type}::new(buf);").unwrap();
    n.indent_advance(out);

    if request.attributes.is_empty() {
        writeln!(out, "// No attributes").unwrap();
        return;
    }

    writeln!(out, "for {attr_var} in {iter_var} {{").unwrap();
    n.indent_advance(out);
    writeln!(out, "match {attr_var} {{").unwrap();

    for attr in &request.attributes {
        let Some(attr) = find_attr(attrset, attr) else {
            continue;
        };
        dump_readable_attr(out, &mut n, spec, attr);
    }

    n.end(out);
    n.end(out);

    assert!(n.level.is_empty());
}

pub fn dump_ops(out: &mut Vec<u8>, spec: &Spec, all: bool) {
    for ops in &spec.operations.list {
        let Some(attrset) = &ops.attribute_set else {
            continue;
        };
        let attrset = spec.find_attr(attrset);

        writeln!(out).unwrap();
        writeln!(out, "# Operation {:?}", ops.name).unwrap();

        let mut generate = |op_name, op: &Operation| {
            let reply_name = gen_ops::reply_type_name(&ops.name, op_name);
            let request_name = gen_ops::request_type_name(&ops.name, op_name);

            writeln!(out).unwrap();
            writeln!(out, "## {op_name} (request)").unwrap();
            writeln!(out).unwrap();

            writeln!(out, "```rust").unwrap();
            dump_writable(out, spec, &op.request, attrset, &request_name);
            writeln!(out, "```").unwrap();

            if all {
                writeln!(out).unwrap();
                writeln!(out, "```rust").unwrap();
                dump_shorthand(out, spec, &op.reply, attrset, &reply_name);
                writeln!(out, "```").unwrap();
            }

            writeln!(out).unwrap();
            writeln!(out, "### {op_name} (reply)").unwrap();
            writeln!(out).unwrap();

            if all {
                writeln!(out, "```rust").unwrap();
                dump_writable(out, spec, &op.reply, attrset, &reply_name);
                writeln!(out, "```").unwrap();
                writeln!(out).unwrap();
            }

            writeln!(out, "```rust").unwrap();
            dump_shorthand(out, spec, &op.reply, attrset, &reply_name);
            writeln!(out, "```").unwrap();

            if all {
                writeln!(out).unwrap();
                writeln!(out, "## Low-level decoding").unwrap();
                writeln!(out).unwrap();

                writeln!(out, "### {op_name} (request)").unwrap();
                writeln!(out).unwrap();

                writeln!(out, "```rust").unwrap();
                dump_readable(out, spec, &op.request, attrset, &request_name);
                writeln!(out, "```").unwrap();

                writeln!(out).unwrap();
                writeln!(out, "### {op_name} (reply)").unwrap();
                writeln!(out).unwrap();

                writeln!(out, "```rust").unwrap();
                dump_readable(out, spec, &op.reply, attrset, &reply_name);
                writeln!(out, "```").unwrap();
            }
        };

        if let Some(r#do) = &ops.r#do {
            generate("Do", r#do)
        }

        if let Some(dump) = &ops.dump {
            generate("Dump", dump);
        }
    }
}
