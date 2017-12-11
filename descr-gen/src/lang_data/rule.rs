use lang_data::data::*;
use lang_data::typed_part::*;
use lang_data::ast::RuleType;
use lang_data::ast::AstStruct;
use lang_data::annotations::*;
use descr_lang::gen::ast::*;
use process::codegen_syntax::{SyntaxData, SyntaxEntry};

/// Parser "rule"
/// List of tokens that makes up some ast type
/// The tokens themselves are stored in
/// lang_data, and referenced by key
#[derive(Debug)]
pub struct AstPartsRule<'a> {
    pub parts: Vec<AstRulePart<'a>>,
    pub ast_type: &'a str,
    pub annots: AnnotList<'a>
}
impl<'a: 's, 's> AstPartsRule<'a> {
    pub fn new(ast_type: &'a str, annots: AnnotList<'a>) -> AstPartsRule<'a> {
        AstPartsRule {
            parts: Vec::new(),
            ast_type,
            annots
        }
    }

    pub fn add_regex(&self, mut acc: String, part: &'a AstRulePart,
                     regex: &str, default_name: Option<&str>, captures: &mut Vec<String>)
                    -> String
    {
        let (capture, capture_name) = match default_name {
            Some(capture_name) => (true, Some(capture_name)),
            None => (false, None)
        };
        if !part.not {
            acc.push_str("\\s*");
        }
        if capture {
            acc.push('(');
        }
        if part.optional {
            acc.push_str("(?:");
        }
        if part.not {
            acc.push_str("^(?:(?!");
        }
        acc.push_str(regex);
        if part.not {
            acc.push_str(").)*");
        }
        if part.optional {
            acc.push_str(")?");
        }
        if capture {
            acc.push(')');
        }
        if let Some(capture_name) = capture_name {
            captures.push(capture_name.to_string());
        }
        acc
    }

    pub fn add_syntax_entries(&self, syntax_data: &mut SyntaxData<'s>, data: &LangData<'a>) {
        let mut regex = String::new();
        let mut regex_begin = String::new();
        let mut collecting_end = false;
        let mut patterns = Vec::new();
        let mut captures = Vec::new();
        let mut captures_begin = Vec::new();
        let mut is_first = true;
        for (i, part) in self.parts.iter().enumerate() {
            if i > 0 {
                is_first = false;
            }
            match &part.token {
                &AstRuleToken::Key(key) => {
                    let typed_part = data.typed_parts.get(key).expect("Could not find part");
                    match typed_part {
                        &TypedPart::AstPart{key} => {
                            // If we are at first position,
                            // add to parent level, else
                            // use current acc as begin,
                            // and collect end
                            // If already at end, transform
                            // to <Key>2 by setting end to [^\s]
                            // and using accum as begin
                            match data.resolve(key) {
                                ResolvedType::ResolvedEnum(key) => {
                                    let enum_data = data.ast_enums.get(key).unwrap();
                                    for item in &enum_data.items {
                                        if is_first {
                                            syntax_data.add_parent_entry(self.ast_type, item);
                                        } else {
                                            patterns.push(item.to_string());
                                        }
                                    }
                                },
                                ResolvedType::ResolvedStruct(key) => {
                                    if is_first {
                                        syntax_data.add_parent_entry(self.ast_type, key);
                                    } else {
                                        patterns.push(key.to_string());
                                    }
                                }
                            }
                            if !is_first {
                                if collecting_end {
                                    panic!("todo");
                                }
                                regex_begin = regex;
                                regex = String::new();
                                captures_begin = captures;
                                captures = Vec::new();
                                collecting_end = true;
                            }
                        },
                        &TypedPart::ListPart{key} => {
                            // Add references to list items
                            match data.resolve(key) {
                                ResolvedType::ResolvedEnum(key) => {
                                    let enum_data = data.ast_enums.get(key).unwrap();
                                    for item in &enum_data.items {
                                        if is_first {
                                            syntax_data.add_parent_entry(self.ast_type, item);
                                        } else {
                                            patterns.push(item.to_string());
                                        }
                                    }
                                },
                                ResolvedType::ResolvedStruct(key) => {
                                    if is_first {
                                        syntax_data.add_parent_entry(self.ast_type, key);
                                    } else {
                                        patterns.push(key.to_string());
                                    }
                                }
                            }
                            if !is_first {
                                if collecting_end {
                                    panic!("todo");
                                }
                                regex_begin = regex;
                                regex = String::new();
                                captures_begin = captures;
                                captures = Vec::new();
                                collecting_end = true;
                            }
                        },
                        &TypedPart::CharPart{chr, ..} => {
                            regex = self.add_regex(regex, part, &to_regex(&chr.to_string()), None, &mut captures);
                        },
                        &TypedPart::TagPart{tag, ..} => {
                            regex = self.add_regex(regex, part, &to_regex(tag), Some("keyword.other"), &mut captures);
                        },
                        &TypedPart::IntPart{key} => {
                            regex = self.add_regex(regex, part, "[-\\+]?[1-9]+", Some("constant.numeric"), &mut captures);
                        },
                        &TypedPart::IdentPart{key} => {
                            regex = self.add_regex(regex, part, "[_]*[a-zA-Z][a-zA-Z0-9_]*", Some("variable.other"), &mut captures);
                        },
                        &TypedPart::FnPart{key, ..} => {
                            panic!("Fn not implemented: {}", key);
                        },
                        &TypedPart::StringPart{key} => {
                            regex = self.add_regex(regex, part, "\"(?:[^\"\\\\]|\\.)*\"", Some("string.quoted"), &mut captures);
                        },
                        &TypedPart::StrPart{key} => {
                            regex = self.add_regex(regex, part, "\"(?:[^\"\\\\]|\\.)*\"", Some("string.quoted"), &mut captures);
                        },
                        &TypedPart::WSPart => {
                            regex.push_str("\\s+");
                        },
                    }
                },
                &AstRuleToken::Tag(tag) => {
                    regex = self.add_regex(regex, part, &to_regex(tag), Some("keyword.other"), &mut captures);
                },
                &AstRuleToken::Func(ident, ref args) => {
                    panic!("Fn not implemented: {}", ident);
                },
                &AstRuleToken::Group(ref parts) => {

                }
            }
        }
        if patterns.len() > 0 {
            if regex.len() == 0 {
                regex.push_str("[^\\s]");
            }
            let (begin, end, begin_captures, end_captures) = if collecting_end {
                (regex_begin, regex, captures_begin, captures)
            } else {
                (regex, "[^\\s]".to_string(), captures, Vec::new())
            };
            syntax_data.entries.insert(
                self.ast_type,
                SyntaxEntry::BeginEnd {
                    begin,
                    end,
                    begin_captures,
                    end_captures,
                    patterns
                }
            );
        } else {
            syntax_data.entries.insert(
                self.ast_type,
                SyntaxEntry::Match {
                    regex,
                    captures
                }
            );
        }
    }
}

pub fn to_regex(string: &str) -> String {
    let mut s = String::with_capacity(string.len() + 1);
    for chr in string.chars() {
        match chr {
            '(' => s.push_str("\\("),
            ')' => s.push_str("\\)"),
            '{' => s.push_str("\\{"),
            '}' => s.push_str("\\}"),
            '[' => s.push_str("\\["),
            ']' => s.push_str("\\]"),
            '?' => s.push_str("\\?"),
            '.' => s.push_str("\\."),
            '+' => s.push_str("\\+"),
            '\\' => s.push_str("\\\\"),
            other => s.push(other)
        }
    }
    s
}

#[derive(Debug)]
pub struct AstRulePart<'a> {
    pub token: AstRuleToken<'a>,
    pub member_key: Option<&'a str>,
    pub optional: bool,
    pub not: bool,
    pub annots: AnnotList<'a>
}

#[derive(Debug)]
pub enum AstRuleToken<'a> {
    Key(&'a str),
    Tag(&'a str),
    Func(&'a str, Vec<RuleFuncArg<'a>>),
    Group(Vec<AstRulePart<'a>>)
}
impl<'a> AstRuleToken<'a> {
    pub fn parse_func_token(token: &FuncToken<'a>) -> AstRuleToken<'a> {
        AstRuleToken::Func(
            token.ident,
            token.fn_args
                .iter()
                .map(|arg| match arg {
                    &FuncArg::QuotedItem(Quoted { string }) => {
                        RuleFuncArg::Quoted(string)
                    }
                }).collect::<Vec<_>>()
        )
    }
}

#[derive(Debug)]
pub enum RuleFuncArg<'a> {
    Quoted(&'a str),
}

impl<'a, 'b> AstRulePart<'a> {
    pub fn gen_part_parser(&self, mut s: String, data: &'b LangData<'a>) -> String {
        if let &AstRuleToken::Group(ref parts) = &self.token {
            // Just forward to parts for now
            for part in parts {
                s = part.gen_part_parser(s, data);
            }
            s
        } else {
            indent!(s 2);
            if !self.optional && !self.not {
                append!(s, "sp >> ");
            }
            if let Some(member_name) = self.member_key {
                append!(s, data.sc(member_name) "_k: ");
            }
            if self.not {
                append!(s, "until_done_result!(");
            }
            if self.optional {
                append!(s, "opt!(do_parse!(sp >> res: ");
                s = self.gen_parser(s, data);
                s += " >> (res)))";
            } else {
                s = self.gen_parser(s, data);
            }
            if self.not {
                s += ")";
            }
            s += " >>\n";
            s
        }
    }
    pub fn gen_parser(&self, mut s: String, data: &'b LangData<'a>) -> String {
        match &self.token {
            &AstRuleToken::Key(key) => data.typed_parts.get(key).unwrap().gen_parser(s, data),
            &AstRuleToken::Tag(string) => {
                if data.debug {
                    s += "debug_wrap!(";
                }
                append!(s, "tag!(\"" string "\")");
                if data.debug {
                    s += ")";
                }
                s
            }
            &AstRuleToken::Func(ident, ref args) => {
                if data.debug {
                    s += "debug_wrap!(";
                }
                append!(s, ident "!(");
                let num_args = args.len();
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        &RuleFuncArg::Quoted(string) => {
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
            },
            &AstRuleToken::Group(ref parts) => {
                // Just forward to parts for now
                for part in parts {
                    s = part.gen_part_parser(s, data);
                }
                s
            }
        }
    }

    pub fn gen_parser_struct_assign(&self, mut s: String, struct_data: Option<&AstStruct<'a>>, data: &'b LangData<'a>) -> String {
        if let &AstRuleToken::Group(ref parts) = &self.token {
            // Just forward to parts for now
            for part in parts {
                s = part.gen_parser_struct_assign(s, struct_data, data);
            }
        } else if let Some(member_key) = self.member_key {
            let is_boxed = match struct_data {
                Some(struct_data) => {
                    struct_data.members.get(member_key).unwrap().boxed
                }
                _ => false,
            };
            append!(s 3, data.sc(member_key) ": ");
            if is_boxed {
                s += "Box::new(";
            }
            if self.not {
                // Not is collected as str
                s += "std::str::from_utf8(";
                s += self.member_key.unwrap();
                s += "_k).unwrap()";
            } else {
                match &self.token {
                    &AstRuleToken::Key(key) => {
                        s = data.typed_parts.get(key).unwrap().gen_parser_val(s, self, data)
                    },
                    &AstRuleToken::Tag(..) => {
                        if self.optional {
                            append!(s, self.member_key.unwrap() "_k.is_some()");
                        } else {
                            s += "true";
                        }
                    }
                    &AstRuleToken::Func(..) => {
                        s += self.member_key.unwrap();
                        s += "_k";
                    },
                    &AstRuleToken::Group(ref parts) => {}
                }
            }
            if is_boxed {
                s += ")";
            }
            s += ",\n";
        }
        s
    }

    pub fn add_to_source(&self, mut s: String, data: &LangData<'a>) -> String {
        if self.not {
            if let Some(member_key) = self.member_key {
                if self.optional {
                    append!(s 2, "if let Some(not_part) = node." member_key "{ s += not_part }\n");
                } else {
                    append!(s 2, "s += node." member_key ";\n");
                }
            }
        } else {
            match &self.token {
                &AstRuleToken::Key(key) => s = data.typed_parts
                                                    .get(key)
                                                    .unwrap()
                                                    .add_to_source(
                                                        s, 
                                                        self.member_key, 
                                                        self.optional,
                                                        data
                                                    ),
                &AstRuleToken::Tag(quoted) => {
                    if self.optional {
                        if let Some(member_key) = self.member_key {
                            append!(s 2, "if node." data.sc(member_key) " { s += \"" quoted "\"; }\n");
                        }
                    } else {
                        append!(s 2, "s += \"" quoted "\";\n");
                    }
                },
                &AstRuleToken::Func(..) => {
                    // Todo, make map of functions and let it handle
                    //panic!("func to source todo");
                },
                &AstRuleToken::Group(ref parts) => {
                    // Just forward to parts for now
                    for part in parts {
                        s = part.add_to_source(s, data);
                    }
                }
            }
        }
        s
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
                    append!(s, type_name "::" rule_ref "Item(" node_expr ") })");
                } else {
                    s += data.sc(rule_ref);
                }
            }
            &AstRule::PartsRule(ref parts_rule) => {
                s += "do_parse!(\n";
                for part in &parts_rule.parts {
                    s = part.gen_part_parser(s, data);
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
                        s = part.gen_parser_struct_assign(s, struct_data, data);
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
