use lang_data::data::*;
use lang_data::typed_part::*;
use lang_data::ast::AstType;
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
    pub idx_members: HashMap<usize, &'a str>
}
impl<'a> AstPartsRule<'a> {
    pub fn new(ast_type: &'a str) -> AstPartsRule<'a> {
        AstPartsRule {
            parts: Vec::new(),
            ast_type,
            member_idxs: HashMap::new(),
            idx_members: HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct AstRulePart<'a> {
    pub token: AstRuleToken<'a>,
    pub member_key: Option<&'a str>,
    pub optional: bool,
    pub not: bool
}

#[derive(Debug)]
pub enum AstRuleToken<'a> {
    Key(&'a str),
    Tag(&'a str)
}

pub enum TypedRulePart<'a> {
    Keyed(&'a TypedPart<'a>),
    Quoted(&'a str)
}
impl<'a, 'b> TypedRulePart<'a> {
    pub fn gen_parser(&self, mut s: String, data: &'b LangData<'a>) -> String {
        match self {
            &TypedRulePart::Keyed(part) => part.gen_parser(s, data),
            &TypedRulePart::Quoted(string) => {
                append!(s, "tag!(\"" string "\")");
                s
            }
        }
    }

    pub fn gen_parser_val(&self, mut s: String, part: &'b AstRulePart<'a>, data: &'b LangData<'a>) -> String {
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
                    s += "true";
                    s
                }
            }
        }
    }
}

impl<'a> AstRulePart<'a> {
    pub fn get_typed_part(&self, data: &'a LangData<'a>) -> TypedRulePart<'a> {
        match &self.token {
            &AstRuleToken::Key(key) => TypedRulePart::Keyed(data.typed_parts.get(key).unwrap()),
            &AstRuleToken::Tag(string) => TypedRulePart::Quoted(string)
        }
    }
}

/// Ref rule
#[derive(Debug)]
pub enum AstRule<'a> {
    PartsRule(AstPartsRule<'a>),
    RefRule(&'a str)
}
impl<'a> AstRule<'a> {
    pub fn gen_rule(&self, mut s: String, data: &LangData, type_ref: &AstType<'a>) -> String {
        let (is_enum, type_name) = match type_ref {
            &AstType::AstStruct(tn) => (false, tn),
            &AstType::AstEnum(tn) => (true, tn)
        };
        match self {
            &AstRule::RefRule(rule_ref) => {
                if is_enum {
                    append!(s, "map!(" data.sc(rule_ref) ", |node| { ");
                    append!(s, type_name "::" rule_ref "Item(node) })");
                } else {
                    s += data.sc(rule_ref);
                }
            },
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
                if is_enum {
                    append!(s, type_name "::" parts_rule.ast_type "Item(" parts_rule.ast_type " {\n");
                } else {
                    s += parts_rule.ast_type;
                    s += " {\n";
                }
                for part in &parts_rule.parts {
                    if let Some(member_key) = part.member_key {
                        let typed_part = part.get_typed_part(data);
                        append!(s 3, data.sc(member_key) ": ");
                        s = typed_part.gen_parser_val(s, part, data);
                        s += ",\n";
                    }
                }
                if is_enum {
                    s += "        })))";
                } else {
                    s += "        }))";
                }
            }
        }
        s
    }
}

#[derive(Debug)]
pub struct ListRule<'a> {
    pub ast_rule: AstRule<'a>,
    pub sep: Option<&'a str>
}
impl<'a> ListRule<'a> {
    pub fn new(sep: Option<&'a str>, ast_rule: AstRule<'a>) -> ListRule<'a> {
        ListRule {
            sep,
            ast_rule
        }
    }
}