use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PartKey<'a> {
    pub name: Option<&'a str>,
    pub key: &'a str
}

impl<'a> PartKey<'a> {
    pub fn get_name(&self) -> &'a str {
        match &self.name {
            &None => self.key,
            &Some(ref str) => str
        }
    }
}

#[derive(Debug)]
pub enum TypedPart<'a> {
    AstPart {
        key: &'a str,
    },
    ListPart{
        key: &'a str,
    },
    CharPart {
        key: &'a str,
        chr: char
    },
    TagPart {
        key: &'a str,
        tag: &'a str
    },
    IntPart {
        key: &'a str
    },
    IdentPart {
        key: &'a str
    }
}
impl<'a> TypedPart<'a> {
    pub fn gen_parser(&self, mut s: String) -> String {
        use TypedPart::*;
        match self {
            &AstPart { key } => {
                s += key;
            },
            &ListPart { key } => {
                s += key;
            },
            &CharPart { key, chr } => {
                s += "char!('";
                s.push(chr);
                s += "')";
            },
            &TagPart { key, tag } => {
                s += "tag!(\"";
                s += tag;
                s += "\")";
            },
            &IntPart { key } => {
                s += "int";
            },
            &IdentPart { key } => {
                s += "ident";
            }
        }
        s
    }

    pub fn gen_val(&self, mut s: String, part: &'a AstRulePart) -> String {
        use TypedPart::*;
        let member_key = part.member_key.unwrap();
        match self {
            &AstPart { .. }
            | &ListPart { .. } => {
                s += member_key;
                s += "_k";
            }
            &CharPart { .. } => {
                if part.optional {
                    s += member_key;
                    s += "_k.is_some()";
                }
            },
            &TagPart { .. } => {
                if part.optional {
                    s += member_key;
                    s += "_k.is_some()";
                }
            },
            &IntPart { .. } => {
                s += member_key;
                s += "_k";
            },
            &IdentPart { .. } => {
                s += member_key;
                s += "_k";
            }
        }
        s
    }
}

#[derive(Debug)]
pub struct TokenData<'a> {
    pub key: &'a str,
    pub tag: &'a str
}

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
    pub fn gen_rule(&self, mut s: String, data: &LangData) -> String {
        use AstRule::*;
        match self {
            &RefRule(rule_ref) => {
                s += rule_ref;
            },
            &PartsRule(ref parts_rule) => {
                s += "do_parse!(\n";
                for part in &parts_rule.parts {
                    s += "        sp >> "; 
                    let typed_part = part.get_typed_part(data);
                    if let Some(member_name) = part.member_key {
                        s += member_name;
                        s += "_k: ";
                    }
                    if part.optional {
                        s += "opt!(";
                        s = typed_part.gen_parser(s);
                        s += ")";
                    } else {
                        s = typed_part.gen_parser(s);
                    }
                    s += " >>\n";
                }
                s += "        (";
                s += parts_rule.ast_type;
                s += " {\n";
                for part in &parts_rule.parts {
                    if let Some(member_key) = part.member_key {
                        let typed_part = part.get_typed_part(data);
                        s += "            ";
                        s += member_key;
                        s += ": ";
                        s = typed_part.gen_val(s, part);
                        s += ",\n";
                    }
                }
                s += "    }))";
            }
        }
        s
    }
}

/// Data for an ast entry
/// Either single, or multiple rules
#[derive(Debug)]
pub struct AstData<'a> {
    pub key: &'a str,
    pub ast_type: &'a str,
    pub rules: Vec<AstRule<'a>>
}
impl<'a> AstData<'a> {
    pub fn new(key: &'a str, ast_type: &'a str) -> AstData<'a> {
        AstData {
            key,
            ast_type,
            rules: Vec::new()
        }
    }
}

pub struct AstDataItem<'a> {
    pub key: &'a str,
    //pub token_list: Vec<TypedPart>
}

#[derive(Debug)]
pub struct ListData<'a> {
    pub key: &'a str,
    pub rules: Vec<ListRule<'a>>
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

impl<'a> ListData<'a> {
    pub fn new(key: &'a str) -> ListData<'a> {
        ListData {
            key,
            rules: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct AstStructMember<'a> {
    pub num_patterns: u32,
    pub name: &'a str,
    pub token_key: &'a str,
    pub optional: bool
}
impl<'a> AstStructMember<'a> {
    pub fn new(name: &'a str, token_key: &'a str, optional: bool) -> AstStructMember<'a> {
        AstStructMember {
            num_patterns: 0,
            name,
            token_key,
            optional: optional
        }
    }
}

/// There is a counter on both
/// structs and struct members
/// After instances are registered,
/// the counter is used to check
/// which fields are optional
#[derive(Debug)]
pub struct AstStruct<'a> {
    pub num_patterns: u32,
    pub members: HashMap<&'a str, AstStructMember<'a>>
}
impl<'a> AstStruct<'a> {
    pub fn new() -> AstStruct<'a> {
        AstStruct {
            num_patterns: 0,
            members: HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct AstEnum<'a> {
    pub items: Vec<&'a str>
}
impl<'a> AstEnum<'a> {
    pub fn new() -> AstEnum<'a> {
        AstEnum {
            items: Vec::new()
        }
    }
}

pub struct LangData<'a> {
    pub typed_parts: HashMap<&'a str, TypedPart<'a>>,
    pub ast_data: HashMap<&'a str, AstData<'a>>,
    pub list_data: HashMap<&'a str, ListData<'a>>,
    pub ast_structs: HashMap<&'a str, AstStruct<'a>>,
    pub ast_enums: HashMap<&'a str, AstEnum<'a>>
}

impl<'a> LangData<'a> {
    pub fn new() -> LangData<'a> {
        LangData {
            typed_parts: HashMap::new(),
            ast_data: HashMap::new(),
            list_data: HashMap::new(),
            ast_structs: HashMap::new(),
            ast_enums: HashMap::new()
        }
    }

    fn add_tag_token(&mut self, key: &'a str, tag: &'a str) {
        self.typed_parts.insert(
            key,
            TypedPart::TagPart {
                key: key,
                tag: tag
            }
        );
    }

    fn add_char_token(&mut self, key: &'a str, chr: char) {
        self.typed_parts.insert(
            key,
            TypedPart::CharPart { key, chr }
        );
    }

    pub fn reg_struct(&mut self, name: &'a str) {
        if !self.ast_structs.contains_key(name) {
            self.ast_structs.insert(
                name,
                AstStruct::new()
            );
        } else {
            // Increment counter
            let ast_struct = self.ast_structs.get_mut(name).unwrap();
            ast_struct.num_patterns += 1;
        }
    }

    pub fn reg_struct_member(&mut self, struct_name: &'a str, member_name: &'a str, token_key: &'a str, optional: bool) {
        let ast_struct = self.ast_structs.get_mut(struct_name).unwrap();
        if ast_struct.members.contains_key(member_name) {
            let struct_member = ast_struct.members.get_mut(member_name).unwrap();
            struct_member.num_patterns += 1;
            if optional {
                struct_member.optional = true;
            }
        } else {
            ast_struct.members.insert(
                member_name,
                AstStructMember::new(member_name, token_key, optional)
            );
        }
    }

    pub fn ensure_enum(&mut self, name: &'a str) -> &AstEnum<'a> {
        if !self.ast_enums.contains_key(name) {
            self.ast_enums.insert(name, AstEnum::new());
        }
        self.ast_enums.get_mut(name).unwrap()
    }

    pub fn ensure_enum_item(&mut self, enum_name: &'a str, item_name: &'a str) {
        self.ast_enums.get_mut(enum_name).unwrap().items.push(item_name);
    }

    /// Resolve typed part assuming keys
    /// are registered
    pub fn resolve_typed_part(&mut self, key: &'a str) {
        if self.ast_data.contains_key(key) {
            self.typed_parts.insert(
                key,
                TypedPart::AstPart {
                    key
                }
            );
        } else if self.list_data.contains_key(key) {
            self.typed_parts.insert(
                key,
                TypedPart::ListPart {
                    key
                }
            );
        } else {
            // Some hardcoded tokens, these could
            // be from some standard library later
            match key {
                "LPAREN" => self.add_char_token("LPAREN", '('),
                "RPAREN" => self.add_char_token("RPAREN", ')'),
                "LBRACE" => self.add_char_token("LBRACE", '{'),
                "RBRACE" => self.add_char_token("RBRACE", '}'),
                "LBRACKET" => self.add_char_token("LBRACKET", '['),
                "RBRACKET" => self.add_char_token("RBRACKET", ']'),
                "COMMA" => self.add_char_token("COMMA", ','),
                "COLON" => self.add_char_token("COLON", ':'),
                "SEMICOLON" => self.add_char_token("SEMICOLON", ';'),
                "EQUAL" => self.add_char_token("EQUAL", '='),
                "LT" => self.add_char_token("LT", '<'),
                "GT" => self.add_char_token("GT", '>'),
                "LTE" => self.add_tag_token("LTE", "<="),
                "GTE" => self.add_tag_token("GTE", ">="),
                "QUESTION" => self.add_char_token("QUESTION", '?'),
                "ident" => {
                    self.typed_parts.insert(
                        key,
                        TypedPart::IdentPart { key }
                    );
                },
                "int" => {
                    self.typed_parts.insert(
                        key,
                        TypedPart::IntPart { key }
                    );
                }
                _ => panic!("Could not find token: {}", key)
            };
        }
    }
}