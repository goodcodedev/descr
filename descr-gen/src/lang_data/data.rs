use std::collections::HashMap;
use std::collections::HashSet;
use lang_data::ast::*;
use lang_data::rule::*;
use lang_data::typed_part::*;
use lang_data::annotations::*;

/// Data for an ast entry
/// Either single, or multiple rules
#[derive(Debug)]
pub struct AstData<'a> {
    pub key: &'a str,
    pub ast_type: &'a str,
    pub rules: Vec<AstRule<'a>>,
    pub annots: AnnotList<'a>
}
impl<'a> AstData<'a> {
    pub fn new(key: &'a str, ast_type: &'a str, annots: AnnotList<'a>) -> AstData<'a> {
        AstData {
            key,
            ast_type,
            rules: Vec::new(),
            annots
        }
    }
}

#[derive(Debug)]
pub struct ListData<'a> {
    pub key: &'a str,
    pub ast_type: Option<&'a str>,
    pub sep: Option<&'a str>,
    pub rules: Vec<ListRule<'a>>,
    pub annots: AnnotList<'a>
}


impl<'a> ListData<'a> {
    pub fn new(key: &'a str, ast_type: Option<&'a str>, sep: Option<&'a str>, annots: AnnotList<'a>) -> ListData<'a> {
        ListData {
            key,
            ast_type,
            sep,
            rules: Vec::new(),
            annots
        }
    }
}

pub struct SnakeCased<'a> {
    pub cache: HashMap<&'a str, String>,
}
impl<'a> SnakeCased<'a> {
    pub fn reg(&mut self, key: &'a str) {
        if !self.cache.contains_key(key) {
            // Capacity, len + 3 or 4
            let mut s = String::with_capacity(key.len() + 3);
            let mut is_first = true;
            for chr in key.chars() {
                if chr.is_uppercase() {
                    if !is_first {
                        s.push('_');
                    }
                    for lower in chr.to_lowercase() {
                        s.push(lower);
                    }
                } else {
                    s.push(chr);
                }
                if is_first {
                    is_first = false;
                }
            }
            self.cache.insert(key, s);
        }
    }

    pub fn get(&mut self, key: &'a str) -> String {
        if !self.cache.contains_key(key) {
            self.reg(key);
        }
        self.cache.get(key).unwrap().clone()
    }

    pub fn get_str(&self, key: &str) -> &str {
        match self.cache.get(key) {
            Some(ref s) => s.as_str(),
            None => panic!("Could not find snake cased for: {}", key),
        }
    }
}

pub enum ResolvedType<'a> {
    ResolvedStruct(&'a str),
    ResolvedEnum(&'a str),
}
impl<'a> ResolvedType<'a> {
    pub fn needs_lifetime(&self, data: &LangData<'a>, visited: &mut HashSet<&'a str>) -> bool {
        match self {
            &ResolvedType::ResolvedEnum(key) => {
                data.ast_enums.get(key).unwrap().needs_lifetime(data, visited)
            }
            &ResolvedType::ResolvedStruct(key) => {
                data.ast_structs.get(key).unwrap().needs_lifetime(data, visited)
            }
        }
    }

    pub fn is_simple(&self, data: &LangData<'a>) -> bool {
        match self {
            &ResolvedType::ResolvedEnum(key) => data.ast_enums.get(key).unwrap().is_simple(data),
            &ResolvedType::ResolvedStruct(key) => data.ast_structs.get(key).unwrap().is_simple(),
        }
    }

    pub fn get_ast_type(&self) -> &'a str {
        match self {
            &ResolvedType::ResolvedEnum(key) => key,
            &ResolvedType::ResolvedStruct(key) => key,
        }
    }
}

pub struct LangData<'a> {
    pub typed_parts: HashMap<&'a str, TypedPart<'a>>,
    pub ast_data: HashMap<&'a str, AstData<'a>>,
    pub list_data: HashMap<&'a str, ListData<'a>>,
    pub ast_structs: HashMap<&'a str, AstStruct<'a>>,
    pub ast_enums: HashMap<&'a str, AstEnum<'a>>,
    // Type information for rules
    // Abstracts over ast/enum/reference
    pub rule_types: HashMap<&'a str, RuleType<'a>>,
    pub snake_cased: SnakeCased<'a>,
    // Assumed to be first item
    pub start_key: Option<&'a str>,
    pub simple_enums: HashSet<&'a str>,
    // Structs are currently removed,
    // So not sure if this is needed,
    // possibly key to enum is needed
    // sometime.
    pub simple_structs: HashSet<&'a str>,
    pub debug: bool,
    // Key: enum/struct name, Set: Parents - can be from
    // member to owning struct/enum, or from
    // struct/enum to another where it is a member
    // Can be traversed to check for references up
    // in the tree, where the member or item should be
    // boxed to avoid infinite structures
    pub parent_refs: ParentRefs<'a>,
}
#[derive(Debug)]
pub struct ParentRefs<'a> {
    pub refs: HashMap<&'a str, HashSet<ParentRef<'a>>>,
}
impl<'a> ParentRefs<'a> {
    pub fn add_ref(&mut self, key: &'a str, parent_ref: ParentRef<'a>) {
        if !self.refs.contains_key(key) {
            self.refs.insert(key, HashSet::new());
        }
        self.refs.get_mut(key).unwrap().insert(parent_ref);
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ParentRef<'a> {
    StructMember {
        struct_name: &'a str,
        member_name: &'a str,
    },
    EnumItem {
        enum_name: &'a str,
        item_name: &'a str,
    },
}

impl<'a> LangData<'a> {
    pub fn new(debug: bool) -> LangData<'a> {
        LangData {
            typed_parts: HashMap::new(),
            ast_data: HashMap::new(),
            list_data: HashMap::new(),
            ast_structs: HashMap::new(),
            ast_enums: HashMap::new(),
            rule_types: HashMap::new(),
            snake_cased: SnakeCased {
                cache: HashMap::new(),
            },
            start_key: None,
            simple_enums: HashSet::new(),
            simple_structs: HashSet::new(),
            debug,
            parent_refs: ParentRefs {
                refs: HashMap::new(),
            },
        }
    }

    // Gets ast key (struct/enum) from part_key
    pub fn get_ast_key(&self, key: &'a str) -> Option<&'a str> {
        if self.typed_parts.contains_key(key) {
            match self.typed_parts.get(key).unwrap() {
                &TypedPart::AstPart { key } => Some(key),
                &TypedPart::ListPart { key } => Some(key),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Resolve key to enum or struct
    /// Key could be enum or struct key, or
    /// rule item key
    pub fn resolve(&self, key: &'a str) -> ResolvedType<'a> {
        if self.ast_enums.contains_key(key) {
            ResolvedType::ResolvedEnum(key)
        } else if self.ast_structs.contains_key(key) {
            ResolvedType::ResolvedStruct(key)
        } else {
            match self.rule_types.get(key) {
                Some(rule_type) => match rule_type {
                    &RuleType::SingleType(type_key) => {
                        if key != type_key {
                            self.resolve(type_key)
                        } else {
                            panic!("Could not find singletype key: {}", key);
                        }
                    }
                    &RuleType::ManyType(type_key) => {
                        if key != type_key {
                            self.resolve(type_key)
                        } else {
                            panic!("Could not find manytype key: {}", key);
                        }
                    }
                },
                None => {
                    println!("{:#?}", self.ast_enums.keys());
                    println!("{:#?}", self.ast_structs.keys());
                    println!("{:#?}", self.rule_types);
                    panic!("Could not find key: {}", key);
                }
            }
        }
    }

    pub fn sc(&self, key: &str) -> &str {
        self.snake_cased.get_str(key)
    }

    pub fn add_ast_type(&self, s: String, key: &str) -> String {
        match self.resolve(key) {
            ResolvedType::ResolvedEnum(rkey) => self.ast_enums.get(rkey).unwrap().add_type(s, self),
            ResolvedType::ResolvedStruct(rkey) => {
                self.ast_structs.get(rkey).unwrap().add_type(s, self)
            }
        }
    }

    fn add_tag_token(&mut self, key: &'a str, tag: &'a str) {
        self.typed_parts
            .insert(key, TypedPart::TagPart { key: key, tag: tag });
    }

    fn add_char_token(&mut self, key: &'a str, chr: char) {
        self.typed_parts
            .insert(key, TypedPart::CharPart { key, chr });
    }

    fn add_fn_token(&mut self, key: &'a str, fnc: &'a str, tpe: &'a str) {
        self.typed_parts
            .insert(key, TypedPart::FnPart { key, fnc, tpe });
    }

    /// Resolve typed part assuming keys
    /// are registered
    pub fn resolve_typed_part(&mut self, key: &'a str) {
        if self.ast_data.contains_key(key) {
            self.typed_parts.insert(key, TypedPart::AstPart { key });
        } else if self.list_data.contains_key(key) {
            self.typed_parts.insert(key, TypedPart::ListPart { key });
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
                "STAR" => self.add_char_token("STAR", '*'),
                "EXCL" => self.add_char_token("EXCL", '!'),
                "DOT" => self.add_char_token("DOT", '.'),
                "QUESTION" => self.add_char_token("QUESTION", '?'),
                "QUOTE" => self.add_char_token("QUOTE", '"'),
                "WS" => {
                    self.typed_parts.insert("WS", TypedPart::WSPart);
                }
                //"WS" => self.add_fn_token("WS", "sp", "&'a str"),
                "string" => {
                    self.typed_parts
                        .insert("string", TypedPart::StringPart { key });
                }
                "ident" => {
                    self.typed_parts.insert(key, TypedPart::IdentPart { key });
                }
                "int" => {
                    self.typed_parts.insert(key, TypedPart::IntPart { key });
                }
                _ => {
                    println!("Could not find token: {}", key);
                    println!("Structs: {:#?}", self.ast_structs.keys());
                    println!("Enums: {:#?}", self.ast_enums.keys());
                    println!("Ast rules: {:#?}", self.ast_data.keys());
                    println!("List rules: {:#?}", self.list_data.keys());
                    panic!("Could not find token: {}", key)
                }
            };
        }
    }
}
