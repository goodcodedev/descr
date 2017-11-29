use lang_data::data::*;
use lang_data::typed_part::*;
use std::collections::HashMap;
use util::*;

/// Parser "rule"
/// List of tokens that makes up some ast type
/// The tokens themselves are stored in 
/// lang_data, and referenced by key
#[derive(Debug)]
pub struct AstPartsRule<'a> {
    pub part_keys: Vec<&'a str>,
    pub parts: Vec<AstRulePart<'a>>,
    pub ast_type: &'a str,
    pub member_idxs: HashMap<&'a str, usize>,
    pub idx_members: HashMap<usize, &'a str>
}
impl<'a> AstPartsRule<'a> {
    pub fn new(ast_type: &'a str) -> AstPartsRule<'a> {
        AstPartsRule {
            part_keys: Vec::new(),
            parts: Vec::new(),
            ast_type,
            member_idxs: HashMap::new(),
            idx_members: HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct AstRulePart<'a> {
    pub part_key: &'a str,
    pub member_key: Option<&'a str>,
    pub optional: bool
}
impl<'a> AstRulePart<'a> {
    pub fn get_typed_part(&self, data: &'a LangData<'a>) -> &'a TypedPart {
        data.typed_parts.get(self.part_key).unwrap()
    }
}

/// Ref rule
#[derive(Debug)]
pub enum AstRule<'a> {
    PartsRule(AstPartsRule<'a>),
    RefRule(&'a str)
}
impl<'a> AstRule<'a> {
    pub fn gen_rule(&self, mut s: String, base_type: &str, data: &LangData, is_enum: bool) -> String {
        match self {
            &AstRule::RefRule(rule_ref) => {
                if is_enum {
                    append!(s, "map!(" data.sc(rule_ref) ", |node| { ");
                    append!(s, base_type "::" rule_ref "Item(node) })");
                } else {
                    s += data.sc(rule_ref);
                }
            },
            &AstRule::PartsRule(ref parts_rule) => {
                s += "do_parse!(\n";
                for part in &parts_rule.parts {
                    append!(s 2, "sp >> ");
                    let typed_part = part.get_typed_part(data);
                    if let Some(member_name) = part.member_key {
                        append!(s, data.sc(member_name) "_k: ");
                    }
                    if part.optional {
                        s += "opt!(";
                        s = typed_part.gen_parser(s, data);
                        s += ")";
                    } else {
                        s = typed_part.gen_parser(s, data);
                    }
                    s += " >>\n";
                }
                s += "        (";
                if is_enum {
                    append!(s, base_type "(" parts_rule.ast_type "Item {\n");
                } else {
                    s += parts_rule.ast_type;
                    s += " {\n";
                }
                for part in &parts_rule.parts {
                    if let Some(member_key) = part.member_key {
                        let typed_part = part.get_typed_part(data);
                        append!(s 3, data.sc(member_key) ": ");
                        s = typed_part.gen_parser_val(s, part);
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