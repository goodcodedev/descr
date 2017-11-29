use lang_data::data::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AstStructMember<'a> {
    pub num_patterns: u32,
    pub name: &'a str,
    pub snake_case: String,
    pub part_key: &'a str,
    pub type_name: &'a str,
    pub optional: bool
}
impl<'a> AstStructMember<'a> {
    pub fn new(name: &'a str, snake_case: String, part_key: &'a str, type_name: &'a str, optional: bool) -> AstStructMember<'a> {
        AstStructMember {
            num_patterns: 0,
            name,
            snake_case,
            part_key,
            type_name,
            optional: optional
        }
    }

    pub fn gen_visitor(&self, s: String, ast_struct: &AstStruct, data: &LangData) -> String {
        let typed_part = data.typed_parts.get(self.part_key).unwrap();
        typed_part.gen_visitor(s, self, ast_struct, data)
    }

    pub fn sc(&self) -> &str {
        self.snake_case.as_str()
    }
}

/// There is a counter on both
/// structs and struct members
/// After instances are registered,
/// the counter is used to check
/// which fields are optional
#[derive(Debug)]
pub struct AstStruct<'a> {
    pub name: &'a str,
    pub snake_case: String,
    pub num_patterns: u32,
    pub members: HashMap<&'a str, AstStructMember<'a>>
}
impl<'a> AstStruct<'a> {
    pub fn new(name: &'a str, snake_case: String) -> AstStruct<'a> {
        AstStruct {
            name,
            snake_case,
            num_patterns: 0,
            members: HashMap::new()
        }
    }
    pub fn sc(&self) -> &str {
        self.snake_case.as_str()
    }
}

#[derive(Debug)]
pub struct AstEnum<'a> {
    pub name: &'a str,
    pub snake_case: String,
    pub items: Vec<&'a str>
}
impl<'a> AstEnum<'a> {
    pub fn new(name: &'a str, snake_case: String) -> AstEnum<'a> {
        AstEnum {
            name,
            snake_case,
            items: Vec::new()
        }
    }

    pub fn sc(&self) -> &str {
        self.snake_case.as_str()
    }
}

#[derive(Debug)]
pub enum AstType<'a> {
    AstStruct(&'a str),
    AstEnum(&'a str)
}
impl<'a> AstType<'a> {
    pub fn get_type_name(&self) -> &str {
        match self {
            &AstType::AstStruct(type_name) => type_name,
            &AstType::AstEnum(type_name) => type_name
        }
    }
}