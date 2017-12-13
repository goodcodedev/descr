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

#[derive(Debug, Clone)]
pub struct PartRegex {
    pub regex: String,
    pub not: bool,
    pub optional: bool,
    pub capture: bool,
    pub capture_name: Option<String>
}
impl PartRegex {
    pub fn add_to_string(&self, mut s: String, add_captures: bool) -> String {
        if !self.not {
            s.push_str("\\s*");
        }
        if add_captures && self.capture {
            s.push('(');
        }
        if self.optional {
            s.push_str("(?:");
        }
        if self.not {
            // Non capture "(?:", negative lookahead "(?!"
            s.push_str("(?:(?!");
        }
        s.push_str(&self.regex);
        if self.not {
            // Any char, zero or more times
            s.push_str(").)*");
        }
        if self.optional {
            s.push_str(")?");
        }
        if add_captures && self.capture {
            s.push(')');
        }
        s
    }
}

#[derive(Debug, Clone)]
pub struct CollectState {
    pub patterns: Vec<String>,
    pub captures: Vec<String>,
    pub regexes: Vec<PartRegex>,
    pub is_first: bool,
    pub is_end: bool,
    // Perhaps better described as
    // non determinant among
    // it's branches
    pub only_optional: bool,
    pub parent_refs: Vec<String>
}
impl CollectState {
    pub fn new(is_end: bool) -> CollectState {
        CollectState {
            patterns: Vec::new(),
            captures: Vec::new(),
            regexes: Vec::with_capacity(4),
            is_first: !is_end,
            is_end,
            only_optional: true,
            parent_refs: Vec::new()
        }
    }

    pub fn append(&mut self, state: &CollectState) {
        self.patterns.extend(state.patterns.iter().cloned());
        self.captures.extend(state.captures.iter().cloned());
        self.regexes.extend(state.regexes.iter().cloned());
        if self.is_first && !state.is_first {
            self.is_first = false;
        }
        if self.only_optional && !state.only_optional {
            self.only_optional = false;
        }
        // Not sure about relevance of parent_refs
    }

    // If it is end part and no regexes,
    // creates negative lookahead for patterns.
    // Patterns therefore needs to be expanded
    // for it to work correctly
    pub fn get_end_regex(&self, syntax_data: &SyntaxData, begin: &CollectState) -> String {
        if self.regexes.len() == 0 {
            // Collect until first non-optional token
            // of patterns
            let expanded_patterns = syntax_data.expand_pattern_list(&begin.patterns);
            let mut is_first = true;
            let mut s = String::with_capacity(10);
            s.push_str("(?!(?:");
            for p in &expanded_patterns {
                if let Some(pattern_entry) = syntax_data.entries.get(p) {
                    let collect = match pattern_entry {
                        &SyntaxEntry::Match{ref collect} => {
                            collect
                        },
                        &SyntaxEntry::BeginEnd{ref begin, ..} => {
                            begin
                        }
                    };
                    if !is_first {
                        s.push('|');
                    } else {
                        is_first = false;
                    }
                    for r in &collect.regexes {
                        s = r.add_to_string(s, false);
                        if !r.optional {
                            break;
                        }
                    }
                }
            }
            s.push_str("))");
            s
        } else {
            self.get_regex()
        }
    }

    // Collects regexes and creates a string
    pub fn get_regex(&self) -> String {
        let mut s = String::with_capacity(self.regexes.len() * 4);
        for regex in &self.regexes {
            s = regex.add_to_string(s, true);
        }
        s
    }

    pub fn add_regex(&mut self, not: bool, optional: bool,
                     regex: &str, default_name: Option<&str>)
    {
        let (capture, capture_name) = match default_name {
            Some(capture_name) => (true, Some(capture_name)),
            None => (false, None)
        };
        let p = PartRegex {
            regex: regex.to_string(),
            not,
            optional,
            capture,
            capture_name: capture_name.map(|n| { String::from(n) })
        };
        if let Some(capture_name) = capture_name {
            self.captures.push(capture_name.to_string());
        }
        if !p.optional && self.only_optional {
            self.only_optional = false;
        }
        self.regexes.push(p);
    }
}

pub enum CollectPartReturn {
    Continue,
    CollectEnd
}

impl<'a: 's, 's> AstPartsRule<'a> {
    pub fn new(ast_type: &'a str, annots: AnnotList<'a>) -> AstPartsRule<'a> {
        AstPartsRule {
            parts: Vec::new(),
            ast_type,
            annots
        }
    }

    pub fn collect_part_syntax(&self, part: &AstRulePart,
                               state: &mut CollectState, 
                               syntax_data: &mut SyntaxData,
                               data: &LangData<'a>) -> CollectPartReturn
    {
        let annot_name = match part.annots.items.get("syntax") {
            Some(ref annot) => {
                match annot.args.get("name") {
                    Some(name) => match name {
                        &AnArgVal::Quoted(name) => Some(name),
                        _ => None
                    }
                    None => None
                }
            },
            None => None
        };
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
                                    if state.is_first {
                                        syntax_data.add_parent_entry(self.ast_type, item);
                                        state.parent_refs.push(String::from(*item));
                                    } else {
                                        state.patterns.push(String::from(*item));
                                    }
                                }
                            },
                            ResolvedType::ResolvedStruct(key) => {
                                if state.is_first {
                                    syntax_data.add_parent_entry(self.ast_type, key);
                                    state.parent_refs.push(String::from(key.to_string()));
                                } else {
                                    state.patterns.push(String::from(key));
                                }
                            }
                        }
                        if !state.is_first && state.is_end {
                            // Todo, branch to "next level" entry,
                            // name_2.., which continues collecting
                            // regexes, and is included at this level
                            panic!("second level regex todo");
                        } else if !state.is_first {
                            // Switch to end "mode"
                            return CollectPartReturn::CollectEnd;
                        }
                    },
                    &TypedPart::ListPart{key} => {
                        // Add references to list items
                        match data.resolve(key) {
                            ResolvedType::ResolvedEnum(key) => {
                                let enum_data = data.ast_enums.get(key).unwrap();
                                for item in &enum_data.items {
                                    if state.is_first {
                                        syntax_data.add_parent_entry(self.ast_type, item);
                                        state.parent_refs.push(String::from(*item));
                                    } else {
                                        state.patterns.push(String::from(*item));
                                    }
                                }
                            },
                            ResolvedType::ResolvedStruct(key) => {
                                if state.is_first {
                                    syntax_data.add_parent_entry(self.ast_type, key);
                                    state.parent_refs.push(String::from(key.to_string()));
                                } else {
                                    state.patterns.push(String::from(key));
                                }
                            }
                        }
                        if !state.is_first && state.is_end {
                            // Todo, branch to "next level" entry,
                            // name_2.., which continues collecting
                            // regexes, and is included at this level
                            panic!("second level regex todo");
                        } else if !state.is_first {
                            // Switch to end "mode"
                            return CollectPartReturn::CollectEnd;
                        }
                    },
                    &TypedPart::CharPart{chr, ..} => {
                        state.add_regex(part.not, part.optional, &to_regex(&chr.to_string()), annot_name.or(None));
                    },
                    &TypedPart::TagPart{tag, ..} => {
                        state.add_regex(part.not, part.optional, &to_regex(tag), annot_name.or(Some("keyword.other")));
                    },
                    &TypedPart::IntPart{..} => {
                        state.add_regex(part.not, part.optional, "[-\\+]?[1-9]+", annot_name.or(Some("constant.numeric")));
                    },
                    &TypedPart::IdentPart{..} => {
                        state.add_regex(part.not, part.optional, "[_]*[a-zA-Z][a-zA-Z0-9_]*", annot_name.or(Some("variable.other")));
                    },
                    &TypedPart::FnPart{key, ..} => {
                        panic!("Fn not implemented: {}", key);
                    },
                    &TypedPart::StringPart{..} => {
                        state.add_regex(part.not, part.optional, "\"(?:[^\"\\\\]|\\.)*\"", annot_name.or(Some("string.quoted")));
                    },
                    &TypedPart::StrPart{..} => {
                        state.add_regex(part.not, part.optional, "\"(?:[^\"\\\\]|\\.)*\"", annot_name.or(Some("string.quoted")));
                    },
                    &TypedPart::WSPart => {
                        state.add_regex(false, false, "\\s+", None);
                    },
                }
            },
            &AstRuleToken::Tag(tag) => {
                state.add_regex(part.not, part.optional, &to_regex(tag), annot_name.or(Some("keyword.other")));
            },
            &AstRuleToken::Func(ident, ..) => {
                panic!("Fn not implemented: {}", ident);
            },
            &AstRuleToken::Group(..) => {

            }
        } 
        CollectPartReturn::Continue
    }

    pub fn add_syntax_entries(&self, mut syntax_data: &mut SyntaxData, data: &LangData<'a>) {
        let mut collect_state = CollectState::new(false);
        let mut collect_state_begin = None;
        for part in &self.parts {
            match self.collect_part_syntax(part, &mut collect_state, &mut syntax_data, data) {
                CollectPartReturn::CollectEnd => {
                    if collect_state.is_first {
                        collect_state.is_first = false;
                    }
                    // Put collect_state in begin,
                    // and create new collect_state for end
                    collect_state_begin = Some(collect_state);
                    collect_state = CollectState::new(true);
                },
                CollectPartReturn::Continue => {
                    if collect_state.is_first {
                        collect_state.is_first = false;
                    }
                }
            }
        }
        if let Some(collect_begin) = collect_state_begin {
            // Begin end entry
            syntax_data.entries.insert(
                self.ast_type.to_string(),
                SyntaxEntry::BeginEnd {
                    begin: collect_begin,
                    end: collect_state
                }
            );
        } else {
            // Match entry
            syntax_data.entries.insert(
                self.ast_type.to_string(),
                SyntaxEntry::Match {
                    collect: collect_state
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
            '*' => s.push_str("\\*"),
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
                    &AstRuleToken::Group(..) => {}
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
