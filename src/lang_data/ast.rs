use lang_data::data::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AstStructMember<'a> {
    pub num_patterns: u32,
    pub name: &'a str,
    pub part_key: &'a str,
    pub type_name: &'a str,
    pub optional: bool
}
impl<'a> AstStructMember<'a> {
    pub fn new(name: &'a str, part_key: &'a str, type_name: &'a str, optional: bool) -> AstStructMember<'a> {
        AstStructMember {
            num_patterns: 0,
            name,
            part_key,
            type_name,
            optional: optional
        }
    }

    pub fn gen_visitor(&self, mut s: String, ast_struct: &AstStruct, data: &LangData) -> String {
        let typed_part = data.typed_parts.get(self.part_key).unwrap();
        typed_part.gen_visitor(s, self, ast_struct, data)
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