use lang_data::data::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AstStructMember<'a> {
    pub num_patterns: u32,
    pub name: &'a str,
    pub snake_case: String,
    // TODO: Refactor to account for tag
    pub part_key: &'a str,
    pub type_name: &'a str,
    pub optional: bool,
    pub not: bool,
    pub tpe: AstMemberType<'a>
}
impl<'a> AstStructMember<'a> {
    pub fn new(name: &'a str, snake_case: String, 
               part_key: &'a str, type_name: &'a str, 
               optional: bool, not: bool,
               tpe: AstMemberType<'a>) 
               -> AstStructMember<'a> {
        AstStructMember {
            num_patterns: 0,
            name,
            snake_case,
            part_key,
            type_name,
            optional,
            not,
            tpe
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

#[derive(Debug)]
pub enum AstMemberType<'a> {
    KeyedToken(&'a str),
    NotString,
}

impl<'a> AstMemberType<'a> {
    pub fn needs_lifetime(&self, data: &LangData) -> bool {
        match self {
            &AstMemberType::KeyedToken(key) => {
                let part = data.typed_parts.get(key).unwrap();
                part.needs_lifetime(data)
            },
            &AstMemberType::NotString => true
        }
    }

    pub fn is_option(&self, member: &AstStructMember<'a>, data: &LangData<'a>) -> bool {
        match self {
            &AstMemberType::KeyedToken(key) => {
                let part = data.typed_parts.get(key).unwrap();
                part.is_option(member)
            },
            // Not sure if it makes sense with option + not
            &AstMemberType::NotString => false
        }
    }

    pub fn add_type(&self, mut s: String, member: &AstStructMember<'a>, data: &LangData<'a>) -> String {
        match self {
            &AstMemberType::KeyedToken(key) => {
                let part = data.typed_parts.get(key).unwrap();
                part.add_type(s, member, data)
            },
            &AstMemberType::NotString => {
                s += "&'a str";
                s
            }
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
    pub fn needs_lifetime(&self, data: &LangData<'a>) -> bool {
        self.members.values().any(|member| { member.tpe.needs_lifetime(data) })
    }
    pub fn add_type(&self, mut s: String, data: &LangData<'a>) -> String {
        s += self.name;
        if self.needs_lifetime(data) {
            s += "<'a>";
        }
        s
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

    pub fn needs_lifetime(&self, data: &LangData<'a>) -> bool {
        self.items.iter().any(|item| {
            data.type_refs.get(item).unwrap().needs_lifetime(data)
        })
    }

    pub fn add_type(&self, mut s: String, data: &LangData<'a>) -> String {
        s += self.name;
        if self.needs_lifetime(data) {
            s += "<'a>";
        }
        s
    }

    pub fn sc(&self) -> &str {
        self.snake_case.as_str()
    }
}

#[derive(Debug)]
pub enum AstType<'a> {
    AstStruct(&'a str),
    AstEnum(&'a str, bool)
}
impl<'a> AstType<'a> {
    pub fn get_type_name(&self) -> &str {
        match self {
            &AstType::AstStruct(type_name) => type_name,
            &AstType::AstEnum(type_name, ..) => type_name
        }
    }

    pub fn needs_lifetime(&self, data: &LangData<'a>) -> bool {
        match self {
            &AstType::AstStruct(key) => {
                let struct_data = data.ast_structs.get(key).expect(&format!("Could not get ast struct {}", key));
                struct_data.needs_lifetime(data)
            },
            &AstType::AstEnum(key, ..) => {
                let enum_data = data.ast_enums.get(key).expect(&format!("Could not get enum: {}", key));
                enum_data.needs_lifetime(data)
            }
        }
    }
}