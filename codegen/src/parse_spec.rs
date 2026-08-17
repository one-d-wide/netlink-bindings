// vim:textwidth=80

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_default_from_empty_object;
use serde_yaml::Value;
use std::{
    os::unix::ffi::{OsStrExt, OsStringExt},
    path::Path,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum ByteOrder {
    #[serde(rename = "big-endian")]
    Big,
    #[serde(rename = "little-endian")]
    Little,
    #[default]
    #[serde(rename = "native-endian")]
    Host,
}

impl ByteOrder {
    pub fn host() -> Self {
        Self::Host
    }
    pub fn is_host(&self) -> bool {
        matches!(self, Self::Host)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
#[serde(rename_all = "kebab-case")]
pub enum EnumEntry {
    NameOnly(String),
    Extended {
        name: String,
        value: Option<u64>,
        doc: Option<String>,
    },
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
#[serde(rename_all = "kebab-case")]
pub enum ConstValue {
    U64(u64),
    String(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum DefType {
    /// a single, standalone constant
    Const {
        /// The value for the const.
        value: ConstValue,
    },
    /// defines an integer enumeration, with values for each entry incrementing
    /// by 1, (e.g. 0, 1, 2, 3)
    #[serde(rename_all = "kebab-case")]
    Enum {
        /// The first value for enum and flags, allows overriding the default
        /// start value of 0 (for enum) and starting bit (for flags). For flags
        /// value-start selects the starting bit, not the shifted value.
        ///
        /// Sparse enumerations are not supported.
        value_start: Option<u64>,
        /// Array of names of the entries for enum and flags.
        entries: Vec<EnumEntry>,
    },
    /// <https://docs.kernel.org/6.16/userspace-api/netlink/genetlink-legacy.html#structures>
    Struct {
        #[allow(unused)]
        header: Option<String>,
        members: Vec<AttrProp>,

        /// Controls whether decoder should pad missing bytes with zeros or
        /// truncate when needed.
        #[serde(default = "default_true")]
        shrinkable: bool,
    },
    /// defines an integer enumeration, with values for each entry occupying a
    /// bit, starting from bit 0, (e.g. 1, 2, 4, 8)
    #[serde(rename_all = "kebab-case")]
    Flags {
        /// The first value for enum and flags, allows overriding the default
        /// start value of 0 (for enum) and starting bit (for flags). For flags
        /// value-start selects the starting bit, not the shifted value.
        ///
        /// Sparse enumerations are not supported.
        value_start: Option<u64>,
        /// Array of names of the entries for enum and flags.
        entries: Vec<EnumEntry>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum AttrType {
    Unused,
    U8,
    U16,
    #[serde(alias = "uint")]
    U32,
    /// 64 bit values are usually aligned by the kernel but it is recommended
    /// that the user space is able to deal with unaligned values.
    U64,
    S8,
    S16,
    #[serde(alias = "sint")]
    S32,
    /// 64 bit values are usually aligned by the kernel but it is recommended
    /// that the user space is able to deal with unaligned values.
    S64,
    /// Special attribute type used for padding attributes which require
    /// alignment bigger than standard 4B alignment required by netlink (e.g. 64
    /// bit integers). There can only be a single attribute of the pad type in
    /// any attribute set and it should be automatically used for padding when
    /// needed.
    Pad {
        len: Option<usize>,
    },
    /// Attribute with no payload, its presence is the entire information.
    Flag,
    /// Raw binary data attribute, the contents are opaque to generic code.
    #[serde(rename_all = "kebab-case")]
    Binary {
        r#struct: Option<String>,
        /// Only appears in struct member definitions
        len: Option<usize>,
    },
    /// Character string. Unless checks has unterminated-ok set to true the
    /// string is required to be null terminated. max-len in checks indicates
    /// the longest possible string, if not present the length of the string is
    /// unbounded.
    ///
    /// Note that max-len does not count the terminating character.
    String,
    /// Attribute containing other (nested) attributes. nested-attributes
    /// specifies which attribute set is used inside.
    #[serde(rename_all = "kebab-case")]
    Nest {
        /// Identifies the attribute space for attributes nested within given
        /// attribute. Only valid for complex attributes which may have
        /// sub-attributes.
        nested_attributes: String,
    },
    IndexedArray {
        #[serde(flatten)]
        sub_type: IndexedArrayType,
    },

    // Experimental
    #[serde(rename = "cbitfield")]
    #[serde(rename_all = "kebab-case")]
    CBitField {
        bits: usize,
        sub_type: CBitFieldType,
    },

    // And some weird ones
    /// Translated to a corresponding binary struct type in [`Spec::fixup()`]
    Bitfield32,
    #[serde(rename_all = "kebab-case")]
    NestTypeValue {
        nested_attributes: String,
        /// Name of the value extracted from the type of a nest-type-value attribute.
        #[allow(unused)]
        type_value: Vec<String>,
    },
    /// Carry another type, specified by value of preceding "selector" attribute
    #[serde(rename_all = "kebab-case")]
    SubMessage {
        /// Name of the sub-message definition to use for the attribute.
        sub_message: String,
        /// Name of the attribute to use for dynamic selection of sub-message format specifier.
        selector: String,
    },
    Undefined,
}

impl Default for AttrType {
    fn default() -> Self {
        Self::Undefined
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum CBitFieldType {
    U8,
    U16,
    #[serde(alias = "uint")]
    U32,
}

/// Same as [`AttrType`], but can be used inside [`AttrType::IndexedArray`]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "sub-type")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum IndexedArrayType {
    #[serde(rename_all = "kebab-case")]
    Nest { nested_attributes: String },

    #[serde(rename_all = "kebab-case")]
    U32,
    #[serde(rename_all = "kebab-case")]
    Binary { r#struct: Option<String> },

    #[serde(skip)]
    Plain {
        // Same attribute, but array type replaced with sub-type
        attr: Box<AttrProp>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum NumOrName {
    Num(usize),
    Name(String),
}

impl NumOrName {
    pub fn get_value(&self) -> usize {
        match self {
            Self::Num(val) => *val,
            _ => unreachable!("{:?}", self),
        }
    }
}

impl From<usize> for NumOrName {
    fn from(value: usize) -> Self {
        Self::Num(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[allow(unused)]
pub struct AttrCheck {
    pub min_len: Option<NumOrName>,
    pub max_len: Option<NumOrName>,
    pub exact_len: Option<NumOrName>,
    pub min: Option<NumOrName>,
    pub max: Option<NumOrName>,
    pub flags_mask: Option<NumOrName>,
    pub unterminated_ok: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct AttrProp {
    /// Identifies the attribute, unique within the set.
    pub name: String,
    /// Netlink attribute type, see Attribute types.
    #[serde(flatten, deserialize_with = "deserialize_default_from_empty_object")]
    pub r#type: AttrType,
    /// Numerical attribute ID, used in serialized Netlink messages. The value
    /// property can be skipped, in which case the attribute ID will be the
    /// value of the previous attribute plus one (recursively) and 1 for the
    /// first attribute in the attribute set.
    ///
    /// Attributes (and operations) use 1 as the default value for the first
    /// entry (unlike enums in definitions which start from 0) because entry 0
    /// is almost always reserved as undefined. Spec can explicitly set value to
    /// 0 if needed.
    ///
    /// Note that the value of an attribute is defined only in its main set (not
    /// in subsets).
    pub value: Option<u16>,
    /// Boolean property signifying that the attribute may be present multiple
    /// times. Allowing an attribute to repeat is the recommended way of
    /// implementing arrays (no extra nesting).
    pub multi_attr: Option<bool>,
    /// For integer types specifies attribute byte order - little-endian or
    /// big-endian.
    #[serde(default = "ByteOrder::host")]
    pub byte_order: ByteOrder,
    /// For integer types specifies that values in the attribute belong to an
    /// enum or flags from the definitions section.
    pub r#enum: Option<String>,
    /// Treat enum as flags regardless of its type in definitions. When both
    /// enum and flags forms are needed definitions should contain an enum and
    /// attributes which need the flags form should use this attribute.
    pub enum_as_flags: Option<bool>,
    /// Input validation constraints used by the kernel. User space should query
    /// the policy of the running kernel using Generic Netlink introspection,
    /// rather than depend on what is specified in the spec file.
    ///
    /// The validation policy in the kernel is formed by combining the type
    /// definition (type and nested-attributes) and the checks.
    pub checks: Option<AttrCheck>,
    /// Optional format indicator that is intended only for choosing the right
    /// formatting mechanism when displaying values of this type. Currently
    /// supported hints are hex, mac, fddi, ipv4, ipv6 and uuid.
    pub display_hint: Option<String>,
    pub doc: Option<String>,
}

impl AttrProp {
    pub fn is_num(&self) -> bool {
        matches!(
            self.r#type,
            AttrType::U8
                | AttrType::U16
                | AttrType::U32
                | AttrType::U64
                | AttrType::S8
                | AttrType::S16
                | AttrType::S32
                | AttrType::S64
        )
    }

    pub fn is_ipv4(&self) -> bool {
        matches!(self.r#type, AttrType::U32)
            && self.display_hint.as_ref().is_some_and(|h| h == "ipv4")
    }

    pub fn is_ipv6(&self) -> bool {
        matches!(self.r#type, AttrType::Binary { r#struct: None, .. })
            && self.display_hint.as_ref().is_some_and(|h| h == "ipv6")
            && self
                .checks
                .as_ref()
                .is_some_and(|c| c.min_len.as_ref().is_some_and(|l| l.get_value() == 16))
    }

    pub fn is_ip(&self) -> bool {
        matches!(self.r#type, AttrType::Binary { r#struct: None, .. })
            && self
                .display_hint
                .as_ref()
                .is_some_and(|h| matches!(h.as_ref(), "ipv4" | "ipv4-or-v6"))
    }

    pub fn is_sockaddr(&self) -> bool {
        matches!(self.r#type, AttrType::Binary { r#struct: None, .. })
            && self
                .display_hint
                .as_ref()
                .is_some_and(|h| h == "sockaddr_in-or-sockaddr_in6")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Definition {
    /// Name for the enum type with commands, if empty no name will be used.
    #[allow(unused)]
    pub enum_name: Option<String>,
    /// Prefix for the C enum name of the command. The name is formed by
    /// concatenating the prefix with the upper case name of the command, with
    /// dashes replaced by underscores.
    #[allow(unused)]
    pub name_prefix: Option<String>,
    /// Name of the type / constant.
    pub name: String,
    #[serde(flatten)]
    pub def: DefType,
    pub doc: Option<String>,

    /// Render the max members for this enum.
    #[allow(unused)]
    pub render_max: Option<bool>,
    /// Name of the render-max counter enum entry.
    #[allow(unused)]
    pub enum_cnt_name: Option<String>,
    /// For C-compatible languages, header which already defines this attribute set.
    #[allow(unused)]
    pub header: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AttrSet {
    /// Prefix for the C enum name of the command. The name is formed by
    /// concatenating the prefix with the upper case name of the command, with
    /// dashes replaced by underscores.
    #[allow(unused)]
    pub name_prefix: Option<String>,
    /// Name for the enum type with commands.
    #[allow(unused)]
    pub enum_name: Option<String>,
    pub name: String,
    pub attributes: Vec<AttrProp>,
    /// Name of another space which this is a logical part of. Sub-spaces can be
    /// used to define a limited group of attributes which are used in a nest.
    pub subset_of: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Request {
    /// Numerical message ID, used in serialized Netlink messages. The same
    /// enumeration rules are applied as to attribute values.
    pub value: Option<u16>,
    #[serde(default)]
    pub attributes: Vec<String>,
    pub fixed_header: Option<String>,
    pub attribute_set: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Operation {
    #[allow(unused)]
    pub pre: Option<String>,
    #[allow(unused)]
    pub post: Option<String>,
    #[serde(default)]
    pub request: Request,
    #[serde(default)]
    pub reply: Request,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum KernelValidationFlag {
    Strict,
    Dump,
    DumpStrict,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct NotifyEvent {
    /// Explicit list of the attributes for the notification.
    #[allow(unused)]
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct OperationSpec {
    /// Identifies the operation.
    pub name: String,
    /// Specifies the attribute set contained within the message.
    pub attribute_set: Option<String>,
    /// Assume operation uses all known attributes (debug only)
    #[serde(default)]
    pub all_attrs: bool,

    /// ID of this message if value for request and response differ, i.e.
    /// requests and responses have different message enums.
    pub value: Option<u16>,
    /// Name of the structure defining the optional fixed-length protocol
    /// header. This header is placed in a message after the netlink and
    /// genetlink headers and before any attributes.
    pub fixed_header: Option<String>,

    /// Specification for the doit request. Should contain request, reply or
    /// both of these properties, each holding a Message attribute list.
    pub r#do: Option<Operation>,
    /// Specification for the dumpit request. Should contain request, reply or
    /// both of these properties, each holding a Message attribute list.
    pub r#dump: Option<Operation>,
    pub doc: Option<String>,

    #[serde(default)]
    pub rust_filter: Option<String>,
    #[serde(default)]
    pub rust_filter_request: Option<String>,
    #[serde(default)]
    pub rust_filter_reply: Option<String>,
    /// Make operation reuse common encoding/decoding types
    #[serde(default)]
    pub transparent: bool,
    /// Allow request type to be provided at runtime
    #[serde(default)]
    pub request_type_at_runtime: bool,

    #[serde(default)]
    pub no_ack: bool,

    /// Name of the command sharing the reply type with this notification.
    pub notify: Option<String>,
    /// Explicit list of the attributes for the notification.
    pub event: Option<NotifyEvent>,
    /// Name of the multicast group generating given notification.
    pub mcgrp: Option<String>,

    #[serde(default)]
    pub flags: Vec<String>,

    /// Kernel attribute validation flags.
    #[allow(unused)]
    pub dont_validate: Option<Vec<KernelValidationFlag>>,
    /// Name of the kernel config option gating the presence of the operation,
    /// without the 'CONFIG_' prefix.
    #[allow(unused)]
    pub config_cond: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Operations {
    /// Name for the enum type with commands, if empty no name will be used.
    #[allow(unused)]
    pub enum_name: Option<String>,
    /// Prefix for the C enum name of the command. The name is formed by
    /// concatenating the prefix with the upper case name of the command, with
    /// dashes replaced by underscores.
    #[allow(unused)]
    pub name_prefix: Option<String>,
    /// The model of assigning values to the operations. "unified" is the recommended model where
    /// all message types belong to a single enum. "directional" has the messages sent to the
    /// kernel and from the kernel enumerated separately.
    pub enum_model: Option<String>,
    /// The only property of operations for genetlink, holds the list of
    /// operations, notifications etc.
    pub list: Vec<OperationSpec>,
    pub fixed_header: Option<String>,

    /// Fallback to this attribute set in reverse-lookup
    pub fallback_attrs: Option<String>,
    /// Make operation reuse common encoding/decoding types
    #[serde(default = "default_true")]
    pub transparent: bool,
    /// Don't narrow down operation type attributes
    #[serde(default)]
    pub all_attrs: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct MulticastGroup {
    /// The name for the group, used to form the define and the value of the
    /// define.
    pub name: String,
    #[serde(default)]
    pub flags: Vec<String>,
    pub value: Option<u64>,
    /// Override for the name of the define in C uAPI.
    #[allow(unused)]
    pub c_define_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct MulticastGroups {
    pub list: Vec<MulticastGroup>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
/// Value to match for dynamic selection of sub-message format specifier.
pub struct SubMessageFormat {
    pub value: String,
    /// Name of the struct definition to use as the fixed header for the sub message.
    pub fixed_header: Option<String>,
    /// Name of the attribute space from which to resolve attributes in the sub message.
    pub attribute_set: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct SubMessage {
    /// Name of the sub-message definition
    pub name: String,
    /// Dynamically selected format specifiers
    pub formats: Vec<SubMessageFormat>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct KernelFamily {
    // We don't use this
}

fn default_protocol() -> Option<String> {
    Some("genetlink".to_string())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Spec {
    pub name: String,
    pub version: Option<u8>,
    #[serde(default = "default_protocol")]
    pub protocol: Option<String>,
    pub protonum: Option<u16>,
    pub doc: String,

    /// Path to the uAPI header, default is linux/${family-name}.h
    #[allow(unused)]
    pub uapi_header: Option<String>,
    /// Name of the define for the family name.
    #[allow(unused)]
    pub c_family_name: Option<String>,
    /// Additional global attributes used for kernel C code generation.
    #[allow(unused)]
    pub kernel_family: Option<KernelFamily>,
    /// Defines if the input policy in the kernel is global, per-operation, or
    /// split per operation type. Default is split.
    #[allow(unused)]
    pub kernel_policy: Option<String>,
    /// Name of the define for the version of the family.
    #[allow(unused)]
    pub c_version_name: Option<String>,
    /// Makes the number of attributes and commands be specified by a define,
    /// not an enum value.
    #[allow(unused)]
    pub max_by_define: Option<bool>,
    /// The explicit name for constant holding the count of operations (last operation + 1).
    #[allow(unused)]
    pub cmd_cnt_name: Option<String>,
    /// Name of the define for the last operation in the list.
    #[allow(unused)]
    pub cmd_max_name: Option<String>,

    #[serde(default)]
    pub experimental: Experimental,

    #[serde(default)]
    pub definitions: Vec<Definition>,
    #[serde(default)]
    pub attribute_sets: Vec<AttrSet>,
    #[serde(default)]
    pub operations: Operations,
    #[allow(unused)]
    pub mcast_groups: Option<MulticastGroups>,
    pub sub_messages: Option<Vec<SubMessage>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Experimental {
    #[serde(default)]
    pub struct_prefix: Option<bool>,
    #[serde(default)]
    pub struct_explicit_padding: bool,
    /// TODO: deprecate. `Pusher` trait can be used instead.
    #[serde(default)]
    pub attr_binary_write: bool,
}

impl Default for Experimental {
    fn default() -> Self {
        Self {
            struct_prefix: Some(false),
            struct_explicit_padding: true,
            attr_binary_write: false,
        }
    }
}

fn merge_yaml(dst: &mut Value, src: &Value) {
    match (dst, src) {
        (Value::Mapping(dst), Value::Mapping(src)) => {
            for (key, src_val) in src {
                if let Some(key) = key.as_str().filter(|k| k.starts_with("_override")) {
                    let Some(src_val) = src_val.as_bool() else {
                        panic!("Value of {key:?} is not a bool: {src_val:?}");
                    };

                    match key {
                        "_override_entirely" => {
                            if src_val {
                                dst.clear();
                            }
                        }
                        _ => panic!("Unknown {:?} key: {key:?}", "_override"),
                    }
                    continue;
                }

                match dst.get_mut(key) {
                    Some(dst_val) => merge_yaml(dst_val, src_val),
                    None => _ = dst.insert(key.clone(), src_val.clone()),
                };
            }
        }
        (Value::Sequence(dst), Value::Sequence(src)) => {
            'next: for src_val in src {
                if let Value::Mapping(map) = src_val {
                    if let Some(name) = map.get("name") {
                        for dst_val in dst.iter_mut() {
                            if let Value::Mapping(dst_map) = dst_val {
                                if dst_map.get("name") == Some(name) {
                                    merge_yaml(dst_val, src_val);
                                    continue 'next;
                                }
                            }
                        }
                    }
                }

                dst.push(src_val.clone());
            }
        }
        (dst, src) => *dst = src.clone(),
    }
}

impl Spec {
    pub fn parse_with_overrides(path: &Path) -> Self {
        let over_prefix = path.with_extension("override");
        let over_prefix = over_prefix.file_name().unwrap().as_bytes();

        let mut overs = Vec::new();
        if let Some(dir) = path.parent() {
            for file in std::fs::read_dir(dir).unwrap() {
                let file = file.unwrap();
                let file_name = file.file_name().into_vec();
                if file_name.starts_with(over_prefix) && file_name.ends_with(b".yaml") {
                    overs.push(file.path());
                }
            }
        }
        overs.sort();
        overs.reverse();

        if path.exists() {
            overs.push(path.into());
        }

        let Some(path) = overs.pop() else {
            println!("Spec file doesn't exist: {path:?}");
            std::process::exit(1);
        };

        println!("Parsing spec: {path:?}");
        let spec = std::fs::read_to_string(path).unwrap();
        let mut spec = Self::parse_to_value(&spec);

        for over in overs {
            println!("Parsing spec override: {over:?}");
            let over = std::fs::read_to_string(&over).unwrap();
            let over: Value = serde_yaml::from_str(&over).unwrap();
            merge_yaml(&mut spec, &over);
        }

        let mut spec: Spec = serde_yaml::from_value(spec).unwrap();
        spec.check();
        spec.fixup();
        spec
    }

    fn parse_to_value(buf: &str) -> serde_yaml::Value {
        // Replace metadata with empty lines
        let sep = "\n---\n";
        let buf: Vec<u8> = if let Some(rem) = buf.find(sep) {
            buf.as_bytes()
                .iter()
                .enumerate()
                .filter(|(i, e)| *i >= rem + sep.len() || **e == b'\n')
                .map(|(_, &e)| e)
                .collect()
        } else {
            buf.into()
        };

        let mut raw_spec: Value = serde_yaml::from_slice(&buf).unwrap();

        // In some cases, we have to first populate attributes of subset-of
        // attrset, before it can be correctly parsed
        let attrsets = raw_spec["attribute-sets"].as_sequence_mut().unwrap();
        for i in 0..attrsets.len() {
            let Some(subset_of) = attrsets[i].get("subset-of") else {
                continue;
            };

            let mut supers = attrsets
                .iter()
                .find(|s| s["name"].as_str().unwrap() == subset_of.as_str().unwrap())
                .unwrap()
                .clone();
            merge_yaml(&mut supers, &attrsets[i]);

            for sup in supers["attributes"].as_sequence_mut().unwrap() {
                let sub_attrs = attrsets[i]["attributes"].as_sequence().unwrap();
                if sub_attrs
                    .iter()
                    .all(|sub| sub["name"].as_str().unwrap() != sup["name"].as_str().unwrap())
                {
                    sup["type"] = Value::String("unused".into());
                }
            }

            attrsets[i] = supers;
        }

        // Verify the spec here, as later parsing from Value won't preserve the source lines
        if serde_yaml::from_value::<Spec>(raw_spec.clone()).is_err() {
            serde_yaml::from_slice::<Spec>(&buf).unwrap();
            unreachable!();
        }

        raw_spec
    }

    /// Matches genetlink and genetlink-legacy specifications
    pub fn is_genetlink(&self) -> bool {
        self.protocol
            .as_ref()
            .is_some_and(|p| p.starts_with("genetlink"))
    }

    fn check(&self) {
        for (i, def) in self.definitions.iter().enumerate() {
            if self
                .definitions
                .iter()
                .enumerate()
                .any(|(j, d)| i != j && d.name == def.name)
            {
                panic!("Found multiple definitions for {:?}", def.name);
            }
        }

        for (i, attrs) in self.attribute_sets.iter().enumerate() {
            if self
                .attribute_sets
                .iter()
                .enumerate()
                .any(|(j, d)| i != j && d.name == attrs.name)
            {
                panic!("Found multiple definitions for {:?}", attrs.name);
            }
        }

        if let Some(m) = &self.operations.enum_model {
            if m != "directional" {
                todo!("Handle {m:?} enum model");
            }
        }
    }

    fn fixup(&mut self) {
        // Remove non-uapi definitions
        self.definitions
            .retain(|d| d.scope.as_ref().is_none_or(|s| s == "uapi"));

        // Add explicit values in attribute sets
        for attrs in &mut self.attribute_sets {
            let mut id: u16 = 0;
            for attr in &mut attrs.attributes {
                id += 1;
                if let Some(val) = &mut attr.value {
                    id = *val;
                }
                attr.value = Some(id);
            }
        }

        // Replace bitfield32 type with binary struct
        for attrs in &mut self.attribute_sets {
            for attr in &mut attrs.attributes {
                if let AttrType::Bitfield32 = &attr.r#type {
                    attr.r#type = AttrType::Binary {
                        r#struct: Some("builtin-bitfield32".into()),
                        len: Some(8),
                    };
                }
            }
        }

        // Replace NestTypeValue with just Nest
        for attrs in &mut self.attribute_sets {
            for attr in &mut attrs.attributes {
                if let AttrType::NestTypeValue {
                    nested_attributes, ..
                } = &attr.r#type
                {
                    attr.r#type = AttrType::Nest {
                        nested_attributes: nested_attributes.clone(),
                    };
                }
            }
        }

        // Remove forward declarations (enums with no entries)
        self.definitions.retain(|d| match &d.def {
            DefType::Flags { entries, .. } | DefType::Enum { entries, .. } => !entries.is_empty(),
            _ => true,
        });

        // Move copy fixed-header definitions to each operation
        if let Some(fixed_header) = &self.operations.fixed_header.clone() {
            for ops in &mut self.operations.list {
                if ops.fixed_header.is_some() {
                    continue;
                }
                // if let Some(other) = &ops.fixed_header {
                //     if fixed_header == other {
                //         continue;
                //     }
                //     panic!("Operation {:?} already defines fixed header", ops.name);
                // }
                ops.fixed_header = Some(fixed_header.clone());
            }
        }

        // Substitute array type
        for attrs in &mut self.attribute_sets {
            for attr in &mut attrs.attributes {
                if let AttrType::IndexedArray { .. } = &attr.r#type {
                    let mut copy = attr.clone();
                    if let AttrType::IndexedArray { sub_type } = &mut attr.r#type {
                        copy.r#type = match sub_type.clone() {
                            IndexedArrayType::U32 => AttrType::U32,
                            IndexedArrayType::Binary { r#struct } => AttrType::Binary {
                                r#struct,
                                len: None,
                            },
                            IndexedArrayType::Nest { .. } => continue,
                            IndexedArrayType::Plain { .. } => unreachable!(),
                        };
                        *sub_type = IndexedArrayType::Plain {
                            attr: Box::new(copy),
                        };
                    }
                }
            }
        }

        // Assign operation type values
        if self.is_genetlink() {
            let mut request_type = 1;
            for ops in &mut self.operations.list {
                let value = None
                    .or(ops.value.as_ref())
                    .or(ops.r#do.as_ref().and_then(|op| op.request.value.as_ref()))
                    .or(ops.dump.as_ref().and_then(|op| op.request.value.as_ref()));

                if let Some(value) = value {
                    request_type = *value;
                } else {
                    ops.value = Some(request_type);
                }
                request_type += 1;
            }
        }

        // Substitute placeholder operations
        for ops in &mut self.operations.list {
            if !ops.request_type_at_runtime {
                continue;
            }
            if ops.r#do.is_some() || ops.dump.is_some() {
                continue;
            }
            let req = Request {
                value: Some(0xfefe),
                ..Default::default()
            };
            let op = Operation {
                request: req.clone(),
                reply: req.clone(),
                ..Default::default()
            };
            ops.transparent = true;
            ops.r#do = Some(op.clone());
            ops.dump = Some(op.clone());
        }
    }

    pub fn try_find_def(&self, name: &str) -> Option<&Definition> {
        self.definitions.iter().find(|op| op.name == name)
    }

    #[track_caller]
    pub fn find_def(&self, name: &str) -> &Definition {
        self.try_find_def(name)
            .unwrap_or_else(|| panic!("Definition {name:?} not found"))
    }

    #[track_caller]
    pub fn find_attr(&self, name: &str) -> &AttrSet {
        self.attribute_sets
            .iter()
            .find(|op| op.name == name)
            .unwrap_or_else(|| panic!("Attribute set {name:?} not found"))
    }

    #[track_caller]
    pub fn find_sub_message(&self, name: &str) -> &SubMessage {
        self.sub_messages
            .as_ref()
            .unwrap()
            .iter()
            .find(|op| op.name == name)
            .unwrap_or_else(|| panic!("Definition {name:?} not found"))
    }

    #[track_caller]
    pub fn find_op(&self, name: &str) -> &OperationSpec {
        self.operations
            .list
            .iter()
            .find(|op| op.name == name)
            .unwrap_or_else(|| panic!("Operation {name:?} not found"))
    }
}
