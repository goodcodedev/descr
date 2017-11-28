use std::collections::HashMap;

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