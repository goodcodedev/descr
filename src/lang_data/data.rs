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