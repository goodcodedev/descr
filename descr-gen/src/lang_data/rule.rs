use lang_data::data::*;
use lang_data::typed_part::*;
use lang_data::ast::RuleType;
use std::collections::HashMap;

/// Parser "rule"
/// List of tokens that makes up some ast type
/// The tokens themselves are stored in
/// lang_data, and referenced by key
#[derive(Debug)]
pub struct AstPartsRule<'a> {
    pub parts: Vec<AstRulePart<'a>>,
    pub ast_type: &'a str,
    pub member_idxs: HashMap<&'a str, usize>,
    pub idx_members: HashMap<usize, &'a str>,
}
impl<'a> AstPartsRule<'a> {
    pub fn new(ast_type: &'a str) -> AstPartsRule<'a> {
        AstPartsRule {
            parts: Vec::new(),
            ast_type,
            member_idxs: HashMap::new(),
            idx_members: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct AstRulePart<'a> {
    pub token: AstRuleToken<'a>,
    pub member_key: Option<&'a str>,
    pub optional: bool,
    pub not: bool,
}

#[derive(Debug)]
pub enum AstRuleToken<'a> {
    Key(&'a str),
    Tag(&'a str),
    Func(&'a str, Vec<RuleFuncArg<'a>>),
}

#[derive(Debug)]
pub enum RuleFuncArg<'a> {
    Quoted(&'a str),
}

pub enum TypedRulePart<'a> {
    Keyed(&'a TypedPart<'a>),
    Quoted(&'a str),
    Func(&'a str, Vec<TypedRuleFuncArg<'a>>),
}
pub enum TypedRuleFuncArg<'a> {
    Quoted(&'a str),
}
impl<'a, 'b> TypedRulePart<'a> {
    pub fn gen_parser(&self, mut s: String, data: &'b LangData<'a>) -> String {
        match self {
            &TypedRulePart::Keyed(part) => part.gen_parser(s, data),
            &TypedRulePart::Quoted(string) => {
                if data.debug {
                    s += "debug_wrap!(";
                }
                append!(s, "tag!(\"" string "\")");
                if data.debug {
                    s += ")";
                }
                s
            }
            &TypedRulePart::Func(ident, ref args) => {
                if data.debug {
                    s += "debug_wrap!(";
                }
                append!(s, ident "!(");
                let num_args = args.len();
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        &TypedRuleFuncArg::Quoted(string) => {
                            append!(s, "\"" string "\"");
                        }
                    }
                    if i < num_args - 1 {
                        s += ", ";
                    }
                }
                s += ")";
                if data.debug {
                    s += ")";
                }
                s
            }
        }
    }

    pub fn gen_parser_val(
        &self,
        mut s: String,
        part: &'b AstRulePart<'a>,
        data: &'b LangData<'a>,
    ) -> String {
        if part.not {
            // Not is collected as str
            s += "std::str::from_utf8(";
            s += part.member_key.unwrap();
            s += "_k).unwrap()";
            s
        } else {
            match self {
                &TypedRulePart::Keyed(typed_part) => typed_part.gen_parser_val(s, part, data),
                &TypedRulePart::Quoted(..) => {
                    if part.optional {
                        append!(s, part.member_key.unwrap() "_k.is_some()");
                    } else {
                        s += "true";
                    }
                    s
                }
                &TypedRulePart::Func(..) => {
                    s += part.member_key.unwrap();
                    s += "_k";
                    s
                }
            }
        }
    }

    pub fn add_to_source(&self, mut s: String, part: &AstRulePart<'a>, data: &LangData<'a>) -> String {
        if part.not {
            if let Some(member_key) = part.member_key {
                if part.optional {
                    append!(s 2, "if let Some(not_part) = node." member_key "{ s += not_part }\n");
                } else {
                    append!(s 2, "s += node" member_key ";\n");
                }
            }
        } else {
            match self {
                &TypedRulePart::Keyed(typed_part) => s = typed_part.add_to_source(s, part, data),
                &TypedRulePart::Quoted(quoted) => {
                    if part.optional {
                        if let Some(member_key) = part.member_key {
                            append!(s 2, "if node." data.sc(member_key) " { s += \"" quoted "\"; }\n");
                        }
                    } else {
                        append!(s 2, "s += \"" quoted "\";\n");
                    }
                },
                &TypedRulePart::Func(..) => {
                    // Todo, make map of functions and let it handle
                    //panic!("func to source todo");
                }
            }
        }
        s
    }
}

impl<'a> AstRulePart<'a> {
    pub fn get_typed_part(&self, data: &'a LangData<'a>) -> TypedRulePart<'a> {
        match &self.token {
            &AstRuleToken::Key(key) => TypedRulePart::Keyed(data.typed_parts.get(key).unwrap()),
            &AstRuleToken::Tag(string) => TypedRulePart::Quoted(string),
            &AstRuleToken::Func(ident, ref args) => TypedRulePart::Func(
                ident,
                args.iter()
                    .map(|arg| match arg {
                        &RuleFuncArg::Quoted(string) => TypedRuleFuncArg::Quoted(string),
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

/// Ref rule
#[derive(Debug)]
pub enum AstRule<'a> {
    PartsRule(AstPartsRule<'a>),
    RefRule(&'a str),
}
impl<'a> AstRule<'a> {
    pub fn gen_rule(
        &self,
        mut s: String,
        data: &LangData,
        rule_type: &RuleType<'a>,
        resolved: &ResolvedType<'a>,
    ) -> String {
        let (is_many, type_name) = match rule_type {
            &RuleType::SingleType(tn) => (false, tn),
            &RuleType::ManyType(tn) => (true, tn),
        };
        let (struct_data, enum_data) = match resolved {
            &ResolvedType::ResolvedEnum(key) => (None, data.ast_enums.get(key)),
            &ResolvedType::ResolvedStruct(key) => (data.ast_structs.get(key), None),
        };
        match self {
            &AstRule::RefRule(rule_ref) => {
                // When is_many, an enum is assumed generated for
                // the rule
                if is_many {
                    let node_expr = match enum_data {
                        Some(enum_data) => {
                            if enum_data.boxed_items.contains(rule_ref) {
                                "Box::new(node)"
                            } else {
                                "node"
                            }
                        }
                        _ => "node",
                    };
                    append!(s, "map!(" data.sc(rule_ref) ", |node| { ");
                    append!(s, type_name "::" rule_ref "Item(node) })");
                } else {
                    s += data.sc(rule_ref);
                }
            }
            &AstRule::PartsRule(ref parts_rule) => {
                s += "do_parse!(\n";
                for part in &parts_rule.parts {
                    indent!(s 2);
                    if !part.optional && !part.not {
                        append!(s, "sp >> ");
                    }
                    let typed_part = part.get_typed_part(data);
                    if let Some(member_name) = part.member_key {
                        append!(s, data.sc(member_name) "_k: ");
                    }
                    if part.not {
                        append!(s, "until_done_result!(");
                    }
                    if part.optional {
                        append!(s, "opt!(do_parse!(sp >> res: ");
                        s = typed_part.gen_parser(s, data);
                        s += " >> (res)))";
                    } else {
                        s = typed_part.gen_parser(s, data);
                    }
                    if part.not {
                        s += ")";
                    }
                    s += " >>\n";
                }
                s += "        (";
                // There could also be "simple enum" here
                // which is enums without data
                let is_simple = is_many && resolved.is_simple(data);
                let is_boxed_item = match enum_data {
                    Some(enum_data) => {
                        if enum_data.boxed_items.contains(parts_rule.ast_type) {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                };
                if is_many {
                    // Could "resolved" be used instead?
                    if is_simple {
                        append!(s, type_name "::" parts_rule.ast_type);
                    } else {
                        append!(s, type_name "::" parts_rule.ast_type "Item(");
                        if is_boxed_item {
                            append!(s, "Box::new(" parts_rule.ast_type "");
                        } else {
                            s += parts_rule.ast_type;
                        }
                        s += " {\n";
                    }
                } else {
                    s += parts_rule.ast_type;
                    s += " {\n";
                }
                if is_simple {
                    s += "        ))";
                } else {
                    for part in &parts_rule.parts {
                        if let Some(member_key) = part.member_key {
                            let is_boxed = match struct_data {
                                Some(struct_data) => {
                                    struct_data.members.get(member_key).unwrap().boxed
                                }
                                _ => false,
                            };
                            let typed_part = part.get_typed_part(data);
                            append!(s 3, data.sc(member_key) ": ");
                            if is_boxed {
                                s += "Box::new(";
                            }
                            s = typed_part.gen_parser_val(s, part, data);
                            if is_boxed {
                                s += ")";
                            }
                            s += ",\n";
                        }
                    }
                    if is_many {
                        s += "        })))";
                        if is_boxed_item {
                            s += ")";
                        }
                    } else {
                        s += "        }))";
                    }
                }
            }
        }
        s
    }
}

#[derive(Debug)]
pub struct ListRule<'a> {
    pub ast_rule: AstRule<'a>,
    pub sep: Option<&'a str>,
}
impl<'a> ListRule<'a> {
    pub fn new(sep: Option<&'a str>, ast_rule: AstRule<'a>) -> ListRule<'a> {
        ListRule { sep, ast_rule }
    }
}
