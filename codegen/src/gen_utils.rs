use quote::format_ident;
use syn::Ident;

use crate::parse_spec::{AttrProp, AttrSet, AttrType};

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

pub fn doc_attr(attr: &AttrProp, mut write: impl FnMut(&str)) {
    let mut docs = Vec::new();
    if let Some(doc) = &attr.doc {
        docs.push(doc.clone());
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

        docs.push(format!(
            "Associated type: {:?} {comment}",
            kebab_to_type(r#enum)
        ));
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
