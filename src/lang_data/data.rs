use std::collections::HashMap;
use lang_data::ast::*;
use lang_data::rule::*;
use lang_data::typed_part::*;

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
pub struct TokenData<'a> {
    pub key: &'a str,
    pub tag: &'a str
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


impl<'a> ListData<'a> {
    pub fn new(key: &'a str) -> ListData<'a> {
        ListData {
            key,
            rules: Vec::new()
        }
    }
}

pub struct SnakeCased<'a> {
    pub cache: HashMap<&'a str, String>
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
                if is_first { is_first = false; }
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
            None => panic!("Could not find snake cased for: {}", key)
        }
    }
}

pub struct LangData<'a> {
    pub typed_parts: HashMap<&'a str, TypedPart<'a>>,
    pub ast_data: HashMap<&'a str, AstData<'a>>,
    pub list_data: HashMap<&'a str, ListData<'a>>,
    pub ast_structs: HashMap<&'a str, AstStruct<'a>>,
    pub ast_enums: HashMap<&'a str, AstEnum<'a>>,
    pub type_refs: HashMap<&'a str, AstType<'a>>,
    pub snake_cased: SnakeCased<'a>
}

impl<'a> LangData<'a> {
    pub fn new() -> LangData<'a> {
        LangData {
            typed_parts: HashMap::new(),
            ast_data: HashMap::new(),
            list_data: HashMap::new(),
            ast_structs: HashMap::new(),
            ast_enums: HashMap::new(),
            type_refs: HashMap::new(),
            snake_cased: SnakeCased { cache: HashMap::new() }
        }
    }

    pub fn sc(&self, key: &str) -> &str {
        self.snake_cased.get_str(key)
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