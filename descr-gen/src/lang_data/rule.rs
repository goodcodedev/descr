use lang_data::data::*;
use lang_data::typed_part::*;
use lang_data::ast::RuleType;
use lang_data::ast::AstStruct;
use lang_data::annotations::*;
use descr_lang::gen::ast::*;
use std::collections::HashMap;
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
pub enum PartRegex {
    Regex {
        regex: String,
        not: bool,
        optional: bool,
        capture: bool,
        capture_name: Option<String>,
        in_group: bool
    },
    OptStart,
    OptEnd,
    NotStart,
    NotEnd
}
impl PartRegex {
    pub fn in_group(&self) -> bool {
        match self {
            &PartRegex::OptStart |
            &PartRegex::OptEnd |
            &PartRegex::NotStart |
            &PartRegex::NotEnd => {
                true
            },
            &PartRegex::Regex{
                in_group,
                ..
            } => {
                in_group
            }
        }
    }

    pub fn add_to_string(&self, mut s: String, add_captures: bool) -> String {
        // Group opt/not starts and ends
        match self {
            &PartRegex::OptStart => s.push_str("(?:"),
            &PartRegex::OptEnd => s.push_str(")?"),
            &PartRegex::NotStart => s.push_str("(?!"),
            &PartRegex::NotEnd => s.push_str(")"),
            &PartRegex::Regex{
                ref regex,
                not,
                optional,
                capture,
                ..
            } => {
                if !not {
                    s.push_str("\\s*");
                }
                if add_captures && capture {
                    s.push('(');
                }
                if optional {
                    s.push_str("(?:");
                }
                if not {
                    // Non capture "(?:", negative lookahead "(?!"
                    s.push_str("(?:(?!");
                }
                s.push_str(regex);
                if not {
                    // Any char, zero or more times
                    s.push_str(").)*");
                }
                if optional {
                    s.push_str(")?");
                }
                if add_captures && capture {
                    s.push(')');
                }
            }
        }
        s
    }
}

#[derive(Debug, Clone)]
pub enum CollectStateGroup {
    Opt,
    Not
}

#[derive(Debug, Clone)]
pub struct CollectState {
    // Sub patterns
    pub patterns: Vec<String>,
    // Regex captures with syntax type name
    pub captures: Vec<String>,
    pub regexes: Vec<PartRegex>,
    pub is_first: bool,
    pub is_last: bool,
    pub is_end: bool,
    pub group_stack: Vec<CollectStateGroup>,
    // If this is branched into a sub.
    // In this case, the match is continued,
    // and regex from here should be lookahead
    pub is_subbed: bool,
    // Perhaps better defined as
    // non determinant among
    // it's branches
    // In those cases, the match is expanded
    // to include child match match
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
            is_last: false,
            is_end,
            group_stack: Vec::new(),
            is_subbed: false,
            only_optional: true,
            parent_refs: Vec::new()
        }
    }

    pub fn close_regex_groups(&mut self) {
        self.regexes.extend(self.group_stack.iter()
                                            .rev()
                                            .map(|grp| match grp {
                                                &CollectStateGroup::Opt => PartRegex::OptEnd,
                                                &CollectStateGroup::Not => PartRegex::NotEnd,
                                            })
                                            .collect::<Vec<_>>());
    }

    // Can be used if a group_stack is
    // copied because of split in the
    // middle of group
    pub fn open_regex_groups(&mut self) {
        let mut opened = self.group_stack.iter()
                                         .map(|grp| match grp {
                                             &CollectStateGroup::Opt => PartRegex::OptStart,
                                             &CollectStateGroup::Not => PartRegex::NotStart,
                                         })
                                         .collect::<Vec<_>>();
        opened.extend(self.regexes.clone());
        self.regexes = opened;
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
        // In case this is subbed, the match is moved to the sub,
        // so need to build negative lookahead
        if self.regexes.len() == 0 || self.only_optional || self.is_subbed {
            // Collect until first non-optional token
            // of patterns
            // Todo: This is not perfect yet as these expanded patterns themselves
            // might be expanded (I think). Some recursion should be used.
            // let is_only_optional = self.regexes.len() > 0;
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
                        if !r.in_group() {
                            if let &PartRegex::Regex{optional, ..} = r {
                                if !optional {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            // There might be optional regexes with captures
            if !self.is_subbed {
                for regex in &self.regexes {
                    s = regex.add_to_string(s, true);
                }
            }
            s.push_str("))");
            s
        } else {
            let mut s = String::with_capacity(self.regexes.len() * 4);
            if self.is_subbed {
                // If this is branched into a sub, use negative lookahead
                s.push_str("(?!(?:");
            }
            for regex in &self.regexes {
                s = regex.add_to_string(s, true);
            }
            if self.is_subbed {
                s.push_str("))");
            }
            s
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
                     regex: &str, default_name: Option<&str>, in_group: bool)
    {
        let (capture, capture_name) = match default_name {
            Some(capture_name) => (true, Some(capture_name)),
            None => (false, None)
        };
        let p = PartRegex::Regex {
            regex: regex.to_string(),
            not,
            optional,
            capture,
            capture_name: capture_name.map(|n| { String::from(n) }),
            in_group
        };
        if let Some(capture_name) = capture_name {
            self.captures.push(capture_name.to_string());
        }
        if !optional && self.only_optional {
            self.only_optional = false;
        }
        self.regexes.push(p);
    }
}

pub enum CollectPartReturn {
    Continue,
    CollectEnd,
    CollectSub
}

pub enum CollectPart<'a : 'b, 'b> {
    Part {
        part: &'b AstRulePart<'a>,
        opt_group: bool,
        not_group: bool
    },
    OptGroupStart,
    OptGroupEnd,
    NotGroupStart,
    NotGroupEnd
}

impl<'a: 's, 's> AstPartsRule<'a> {
    pub fn new(ast_type: &'a str, annots: AnnotList<'a>) -> AstPartsRule<'a> {
        AstPartsRule {
            parts: Vec::new(),
            ast_type,
            annots
        }
    }

    fn add_parent_entry(&self, key: &str,
                        state: &mut CollectState, 
                        syntax_data: &mut SyntaxData,
                        data: &LangData<'a>)
    {
        match data.resolve(key) {
            ResolvedType::ResolvedEnum(key) => {
                let enum_data = data.ast_enums.get(key).unwrap();
                for item in &enum_data.items {
                    syntax_data.add_parent_entry(self.ast_type, item);
                    state.parent_refs.push(String::from(*item));
                }
            },
            ResolvedType::ResolvedStruct(key) => {
                syntax_data.add_parent_entry(self.ast_type, key);
                state.parent_refs.push(String::from(key.to_string()));
            }
        }
    }

    fn add_patterns(&self, key: &str,
                    state: &mut CollectState, 
                    data: &LangData<'a>)
    {
        match data.resolve(key) {
            ResolvedType::ResolvedEnum(key) => {
                let enum_data = data.ast_enums.get(key).unwrap();
                for item in &enum_data.items {
                    state.patterns.push(String::from(*item));
                }
            },
            ResolvedType::ResolvedStruct(key) => {
                state.patterns.push(String::from(key));
            }
        }
    }

    pub fn collect_part_syntax(&self, part: &AstRulePart,
                               state: &mut CollectState, 
                               syntax_data: &mut SyntaxData,
                               data: &LangData<'a>) -> CollectPartReturn
    {
        // Todo: helper method
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
        let in_group = state.group_stack.len() > 0;
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
                        if state.is_first {
                            self.add_parent_entry(key, state, syntax_data, data);
                        } else if state.is_end {
                            // Branch to "next level" entry,
                            // name_sub.., which continues collecting
                            // regexes, and is included at this level
                            // This elements end should be
                            // prepended to the sub, and used with negative
                            // lookahead here.
                            if state.is_last {
                                return CollectPartReturn::CollectSub;
                            } else {
                                // Use currently collected regexes as
                                // lookaheads in this "end" matcher, then as begin
                                // match for a child part where this entry is.
                                return CollectPartReturn::CollectSub;
                            }
                        } else {
                            self.add_patterns(key, state, data);
                            // Switch to end "mode"
                            return CollectPartReturn::CollectEnd;
                        }
                    },
                    &TypedPart::ListPart{key} => {
                        if state.is_first {
                            self.add_parent_entry(key, state, syntax_data, data);
                        } else if state.is_end {
                            if state.is_last {
                                // If this is last part, just add as parent entry
                                self.add_parent_entry(key, state, syntax_data, data);
                            } else {
                                // Use currently collected regexes as
                                // lookaheads in this "end" matcher, then as begin
                                // match for a child part where this entry is.
                                return CollectPartReturn::CollectSub;
                            }
                        } else {
                            self.add_patterns(key, state, data);
                            // Switch to end "mode"
                            return CollectPartReturn::CollectEnd;
                        }
                    },
                    &TypedPart::CharPart{chr, ..} => {
                        state.add_regex(part.not, part.optional, &to_regex(&chr.to_string()), annot_name.or(None), in_group);
                    },
                    &TypedPart::TagPart{tag, ..} => {
                        state.add_regex(part.not, part.optional, &to_regex(tag), annot_name.or(Some("keyword.other")), in_group);
                    },
                    &TypedPart::IntPart{..} => {
                        state.add_regex(part.not, part.optional, "[-\\+]?[1-9]+", annot_name.or(Some("constant.numeric")), in_group);
                    },
                    &TypedPart::IdentPart{..} => {
                        state.add_regex(part.not, part.optional, "[_]*[a-zA-Z][a-zA-Z0-9_]*", annot_name.or(Some("variable.other")), in_group);
                    },
                    &TypedPart::FnPart{key, ..} => {
                        panic!("Fn not implemented: {}", key);
                    },
                    &TypedPart::StringPart{..} => {
                        state.add_regex(part.not, part.optional, "\"(?:[^\"\\\\]|\\.)*\"", annot_name.or(Some("string.quoted")), in_group);
                    },
                    &TypedPart::StrPart{..} => {
                        state.add_regex(part.not, part.optional, "\"(?:[^\"\\\\]|\\.)*\"", annot_name.or(Some("string.quoted")), in_group);
                    },
                    &TypedPart::WSPart => {
                        state.add_regex(false, false, "\\s+", None, in_group);
                    },
                }
            },
            &AstRuleToken::Tag(tag) => {
                state.add_regex(part.not, part.optional, &to_regex(tag), annot_name.or(Some("keyword.other")), in_group);
            },
            &AstRuleToken::Func(ident, ..) => {
                panic!("Fn not implemented: {}", ident);
            },
            &AstRuleToken::Group(ref _parts) => {
                // Groups should be flattened from flatten_syntax_parts
            }
        } 
        CollectPartReturn::Continue
    }

    // Creates a flat structure of parts and markers
    // for group start and end.
    // Group parts are flattened into the vector
    pub fn flatten_syntax_parts<'b>(
            mut v: Vec<CollectPart<'a, 'b>>, 
            parts: &'b Vec<AstRulePart<'a>>, 
            opt_group: bool, 
            not_group: bool)
            -> Vec<CollectPart<'a, 'b>>
    {
        for part in parts {
            match &part.token {
                &AstRuleToken::Key(..) |
                &AstRuleToken::Tag(..) |
                &AstRuleToken::Func(..) => {
                    v.push(CollectPart::Part {
                        part,
                        opt_group,
                        not_group
                    });
                },
                &AstRuleToken::Group(ref group_parts) => {
                    if part.optional {
                        v.push(CollectPart::OptGroupStart);
                        v = Self::flatten_syntax_parts(
                            v, 
                            group_parts, 
                            true,
                            part.not || not_group
                        );
                        v.push(CollectPart::OptGroupEnd);
                    } else if part.not {
                        v.push(CollectPart::NotGroupStart);
                        v = Self::flatten_syntax_parts(
                            v, 
                            group_parts, 
                            part.optional || opt_group, 
                            true
                        );
                        v.push(CollectPart::NotGroupEnd);
                    } else {
                        v = Self::flatten_syntax_parts(
                            v, 
                            group_parts, 
                            part.optional || opt_group, 
                            true
                        );
                    }
                }
            }
        }
        v
    }

    pub fn add_syntax_entries(&self,
                              syntax_data: &mut SyntaxData, 
                              data: &LangData<'a>)
    {
        let v = Self::flatten_syntax_parts(Vec::new(), &self.parts, false, false);
        self.add_syntax_entries_from_syntax_parts(
            &v,
            syntax_data,
            data,
            self.ast_type.to_string(),
            0,
            None
        );
    }

    fn add_syntax_entries_from_syntax_parts<'b>(
                              &self,
                              sparts: &Vec<CollectPart<'a, 'b>>,
                              mut syntax_data: &mut SyntaxData, 
                              data: &LangData<'a>,
                              match_key: String,
                              from_index: usize,
                              from_state: Option<CollectState>)
    {
        let mut collect_state = match from_state {
            Some(s) => s,
            _ => CollectState::new(false)
        };
        // If there is existing group_stack, open to current group
        collect_state.open_regex_groups();
        let mut collect_state_begin = None;
        let parts_len = sparts.len();
        for (i, spart) in sparts[from_index..].iter().enumerate() {
            let i = i + from_index;
            if i == parts_len - 1 {
                collect_state.is_last = true;
            }
            match spart {
                &CollectPart::Part{ref part, ..} => {
                    match self.collect_part_syntax(part, &mut collect_state, &mut syntax_data, data) {
                        CollectPartReturn::CollectEnd => {
                            if collect_state.is_first {
                                collect_state.is_first = false;
                            }
                            collect_state.close_regex_groups();
                            // Put collect_state in begin,
                            // and create new collect_state for end
                            collect_state_begin = Some(collect_state);
                            collect_state = CollectState::new(true);
                            if let Some(ref begin) = collect_state_begin {
                                // Transfer group stack so we can open to current group
                                collect_state.group_stack = begin.group_stack.clone();
                                collect_state.open_regex_groups();
                            }
                        },
                        CollectPartReturn::Continue => {
                            if collect_state.is_first {
                                collect_state.is_first = false;
                            }
                        },
                        CollectPartReturn::CollectSub => {
                            // Collect state may be in the middle of a
                            // opt or not group. So close down groups
                            // in regexes
                            collect_state.close_regex_groups();
                            let mut sub_state = CollectState::new(false);
                            sub_state.regexes = collect_state.regexes.clone();
                            sub_state.captures = collect_state.captures.clone();
                            // Transfer group stack so we can open to current group
                            sub_state.group_stack = collect_state.group_stack.clone();
                            collect_state.captures.clear();
                            sub_state.only_optional = collect_state.only_optional;
                            sub_state.is_first = false;
                            collect_state.is_subbed = true;
                            let mut sub_key = match_key.clone();
                            sub_key.push_str("_sub");
                            if let Some(ref mut begin) = collect_state_begin {
                                begin.patterns.push(sub_key.clone());
                            }
                            // Branch to sub, then break
                            self.add_syntax_entries_from_syntax_parts(
                                sparts, &mut syntax_data, data, sub_key, i, Some(sub_state)
                            );
                            break;
                        }
                    }
                },
                &CollectPart::OptGroupStart => {
                    collect_state.group_stack.push(CollectStateGroup::Opt);
                    collect_state.regexes.push(PartRegex::OptStart);
                },
                &CollectPart::OptGroupEnd => {
                    collect_state.group_stack.pop();
                    collect_state.regexes.push(PartRegex::OptEnd);
                },
                &CollectPart::NotGroupStart => {
                    collect_state.group_stack.push(CollectStateGroup::Not);
                    collect_state.regexes.push(PartRegex::NotStart);
                },
                &CollectPart::NotGroupEnd => {
                    collect_state.group_stack.pop();
                    collect_state.regexes.push(PartRegex::NotEnd);
                },
            }
        }
        if let Some(collect_begin) = collect_state_begin {
            // Begin end entry
            syntax_data.entries.insert(
                match_key,
                SyntaxEntry::BeginEnd {
                    begin: collect_begin,
                    end: collect_state
                }
            );
        } else {
            // Match entry
            syntax_data.entries.insert(
                match_key,
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

pub struct AstRulePartMemberData<'a> {
    pub member_name: &'a str,
    pub part_key: &'a str,
    pub optional: bool,
    pub not: bool,
    pub tag: bool
}

pub struct GenParserData {
    group_num: u32,
    group_stack: Vec<String>,
    // Member name, (Group name, Ref)
    member_map: HashMap<String, (String, String)>,
    opt_group: bool
}
impl GenParserData {
    pub fn new() -> GenParserData {
        GenParserData {
            group_num: 1,
            group_stack: Vec::new(),
            member_map: HashMap::new(),
            opt_group: false
        }
    }

    pub fn push_group(&mut self) -> String {
        let mut group_name = "group_".to_string();
        group_name.push_str(&self.group_num.to_string());
        self.group_stack.push(group_name.clone());
        self.group_num += 1;
        group_name
    }

    pub fn pop_group(&mut self) {
        let _ = self.group_stack.pop();
    }
}

impl<'a, 'b> AstRulePart<'a> {
    pub fn collect_ast_member_data(&self,
                                   mut list: Vec<AstRulePartMemberData<'a>>, 
                                   parent_opt: bool,
                                   typed_parts: &HashMap<&'a str, TypedPart<'a>>)
                                   -> Vec<AstRulePartMemberData<'a>> 
        {
        match &self.token {
            &AstRuleToken::Key(key) => {
                let typed_part = typed_parts.get(key).unwrap();
                if typed_part.is_auto_member() {
                    list.push(AstRulePartMemberData {
                        member_name: self.member_key.unwrap_or(key),
                        part_key: key,
                        optional: parent_opt || self.optional,
                        not: self.not,
                        tag: false
                    });
                } else {
                    match typed_part {
                        &TypedPart::CharPart { .. }
                        | &TypedPart::FnPart { .. }
                        | &TypedPart::WSPart { .. } => {
                            // Count as member if
                            // member key is given
                            if let Some(member_key) = self.member_key {
                                list.push(AstRulePartMemberData {
                                    member_name: member_key,
                                    part_key: key,
                                    optional: parent_opt || self.optional,
                                    not: self.not,
                                    tag: false
                                });
                            }
                        },
                        &TypedPart::TagPart { .. } => {
                            // This might be handled
                            // in AstRuleToken::Tag now
                            if let Some(member_key) = self.member_key {
                                list.push(AstRulePartMemberData {
                                    member_name: member_key,
                                    part_key: key,
                                    optional: parent_opt || self.optional,
                                    not: self.not,
                                    tag: true
                                });
                            }
                        }
                        _ => {}
                    }
                }
            },
            &AstRuleToken::Tag(string) => {
                if let Some(member_key) = self.member_key {
                    list.push(AstRulePartMemberData {
                        member_name: member_key,
                        part_key: string,
                        optional: parent_opt || self.optional,
                        not: self.not,
                        tag: true
                    });
                }
            },
            &AstRuleToken::Func(..) => {
                if let Some(member_key) = self.member_key {
                    list.push(AstRulePartMemberData {
                        member_name: member_key,
                        part_key: "",
                        optional: parent_opt || self.optional,
                        not: self.not,
                        tag: false
                    });
                }
            },
            &AstRuleToken::Group(ref group_parts) => {
                if self.not {
                    if let Some(member_name) = self.member_key {
                        list.push(AstRulePartMemberData {
                            member_name,
                            part_key: "",
                            optional: parent_opt || self.optional,
                            not: true,
                            tag: false
                        });
                    }
                } else {
                    for group_part in group_parts {
                        list = group_part.collect_ast_member_data(list, parent_opt || self.optional, typed_parts);
                    }
                }
            }
        }
        list
    }

    pub fn gen_part_parser(&self, mut s: String, data: &'b LangData<'a>, gen_data: &'b mut GenParserData) -> String {
        if let &AstRuleToken::Group(ref parts) = &self.token {
            if self.not {
                if let Some(member_name) = self.member_key {
                    append!(s, data.sc(member_name) "_k: ");
                }
                append!(s, "until_done_result!(do_parse!(");
                for part in parts {
                    s = part.gen_part_parser(s, data, gen_data);
                }
                append!(s, "))");
            } else if self.optional {
                // Collect member names
                let mut member_names = Vec::new();
                for part in parts {
                    if let Some(member_name) = part.member_key {
                        member_names.push(member_name);
                    }
                }
                let num_members = member_names.len();
                indent!(s 2);
                let group_name = if num_members > 0 {
                    let group_name = gen_data.push_group();
                    gen_data.opt_group = true;
                    append!(s, &group_name ": ");
                    Some(group_name)
                } else {
                    None
                };
                append!(s, "opt!(do_parse!(\n");
                for part in parts {
                    s = part.gen_part_parser(s, data, gen_data);
                }
                if num_members > 0 {
                    append!(s 3, "(");
                    let is_tuple = if num_members > 1 { true } else { false };
                    if is_tuple {
                        // Make tuple
                        s += "(";
                    }
                    for (i, member_name) in member_names.iter().enumerate() {
                        append!(s, member_name "_k");
                        if i < num_members - 1 {
                            s += ", ";
                        }
                        let mut member_ref = group_name.clone().unwrap();
                        if is_tuple {
                            let it_string = i.to_string();
                            append!(member_ref, "." it_string.as_ref());
                        }
                        gen_data.member_map.insert(String::from(*member_name), (group_name.clone().unwrap(), member_ref));
                    }
                    if is_tuple {
                        s += ")";
                    }
                    s += ")";
                }
                append!(s, ")) >>\n");
            } else {
                // Just forward to parts for now
                for part in parts {
                    s = part.gen_part_parser(s, data, gen_data);
                }
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
                s = self.gen_parser(s, data, gen_data);
                s += " >> (res)))";
            } else {
                s = self.gen_parser(s, data, gen_data);
            }
            if self.not {
                s += ")";
            }
            s += " >>\n";
            s
        }
    }
    pub fn gen_parser(&self, mut s: String, data: &'b LangData<'a>, gen_data: &'b mut GenParserData) -> String {
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
                    s = part.gen_part_parser(s, data, gen_data);
                }
                s
            }
        }
    }

    pub fn gen_parser_struct_assign(&self,
                                    mut s: String, 
                                    struct_data: Option<&AstStruct<'a>>, 
                                    data: &'b LangData<'a>, 
                                    gen_data: &'b mut GenParserData) -> String {
        if let &AstRuleToken::Group(ref parts) = &self.token {
            // Just forward to parts for now
            for part in parts {
                s = part.gen_parser_struct_assign(s, struct_data, data, gen_data);
            }
        } else if let Some(member_key) = self.member_key {
            let is_boxed = match struct_data {
                Some(struct_data) => {
                    struct_data.members.get(member_key).unwrap().boxed
                }
                _ => false,
            };
            append!(s 3, data.sc(member_key) ": ");
            // Check if there is a member ref from grouped
            let (member_ref, group_key, opt_group) = if let Some(ref member_mapped) = gen_data.member_map.get(member_key) {
                (
                    String::from(member_mapped.1.as_ref()),
                    Some(String::from(member_mapped.0.as_ref())),
                    gen_data.opt_group
                )
            } else {
                let mut member_ref = String::from(self.member_key.unwrap());
                member_ref.push_str("_k");
                (member_ref, None, false)
            };
            if is_boxed {
                s += "Box::new(";
            }
            if let Some(ref group_key) = group_key {
                if opt_group {
                    append!(s, group_key.as_ref() ".map(|" group_key.as_ref() "| { ");
                }
            }
            if self.not {
                // Not is collected as str
                s += "std::str::from_utf8(";
                s += &member_ref;
                s += ").unwrap()";
            } else {
                match &self.token {
                    &AstRuleToken::Key(key) => {
                        s = data.typed_parts.get(key).unwrap().gen_parser_val(s, self, member_ref)
                    },
                    &AstRuleToken::Tag(..) => {
                        if self.optional {
                            append!(s, &member_ref ".is_some()");
                        } else {
                            s += "true";
                        }
                    }
                    &AstRuleToken::Func(..) => {
                        s += &member_ref;
                    },
                    &AstRuleToken::Group(..) => {}
                }
            }
            if opt_group {
                s += " })";
            }
            if is_boxed {
                s += ")";
            }
            s += ",\n";
        }
        s
    }

    pub fn add_to_source(&self, mut s: String, data: &LangData<'a>, parent_opt: bool) -> String {
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
                                                        self.optional || parent_opt,
                                                        data
                                                    ),
                &AstRuleToken::Tag(quoted) => {
                    if self.optional || parent_opt {
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
                    if self.optional {
                        // Check member keys for is_some()
                        let member_key_parts = parts.iter()
                                                    .filter(|p| p.member_key.is_some())
                                                    .collect::<Vec<_>>();
                        let mlen = member_key_parts.len();
                        if mlen > 0 {
                            append!(s 2, "if ");
                            for (i, part) in member_key_parts.iter().enumerate() {
                                if let Some(member_key) = part.member_key {
                                    append!(s, "node." member_key ".is_some()");
                                    if i < mlen - 1 {
                                        s += ", ";
                                    }
                                }
                            }
                            append!(s, " {\n");
                            for part in parts {
                                s = part.add_to_source(s, data, true);
                            }
                            append!(s 2, "}\n");
                        }
                    } else {
                        // Just forward to parts for now
                        for part in parts {
                            s = part.add_to_source(s, data, parent_opt);
                        }
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
        let mut gen_data = GenParserData::new();
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
                    s = part.gen_part_parser(s, data, &mut gen_data);
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
                        s = part.gen_parser_struct_assign(s, struct_data, data, &mut gen_data);
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
