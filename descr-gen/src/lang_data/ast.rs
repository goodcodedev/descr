use lang_data::data::*;
use std::collections::HashMap;
use std::collections::HashSet;

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
    pub tpe: AstMemberType<'a>,
    pub boxed: bool,
}
impl<'a> AstStructMember<'a> {
    pub fn new(
        name: &'a str,
        snake_case: String,
        part_key: &'a str,
        type_name: &'a str,
        optional: bool,
        not: bool,
        tpe: AstMemberType<'a>,
    ) -> AstStructMember<'a> {
        AstStructMember {
            num_patterns: 0,
            name,
            snake_case,
            part_key,
            type_name,
            optional,
            not,
            tpe,
            boxed: false,
        }
    }

    pub fn gen_visitor(&self, s: String, ast_struct: &AstStruct, data: &LangData) -> String {
        match self.tpe {
            AstMemberType::KeyedToken(part_key) => {
                let typed_part = data.typed_parts.get(part_key).unwrap();
                typed_part.gen_visitor(s, self, ast_struct, data)
            }
            _ => s,
        }
    }

    pub fn sc(&self) -> &str {
        self.snake_case.as_str()
    }
}

#[derive(Debug)]
pub enum AstMemberType<'a> {
    KeyedToken(&'a str),
    TagBool(&'a str),
    NotString,
}

impl<'a> AstMemberType<'a> {
    pub fn needs_lifetime(&self, data: &LangData<'a>, visited: &mut HashSet<&'a str>) -> bool {
        match self {
            &AstMemberType::KeyedToken(key) => {
                let part = data.typed_parts.get(key).unwrap();
                part.needs_lifetime(data, visited)
            }
            &AstMemberType::NotString => true,
            &AstMemberType::TagBool(..) => false,
        }
    }

    pub fn is_option(&self, member: &AstStructMember<'a>, data: &LangData<'a>) -> bool {
        match self {
            &AstMemberType::KeyedToken(key) => {
                let part = data.typed_parts.get(key).unwrap();
                part.is_option(member)
            }
            // Not sure if it makes sense with option + not
            &AstMemberType::NotString => false,
            &AstMemberType::TagBool(..) => false,
        }
    }

    pub fn add_type(
        &self,
        mut s: String,
        data: &LangData<'a>,
    ) -> String {
        match self {
            &AstMemberType::KeyedToken(key) => {
                let part = data.typed_parts.get(key).unwrap();
                part.add_type(s, data)
            }
            &AstMemberType::NotString => {
                s += "&'a str";
                s
            }
            &AstMemberType::TagBool(..) => {
                s += "bool";
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
    pub members: HashMap<&'a str, AstStructMember<'a>>,
    pub members_ordered: Vec<&'a str>
}
impl<'a> AstStruct<'a> {
    pub fn new(name: &'a str, snake_case: String) -> AstStruct<'a> {
        AstStruct {
            name,
            snake_case,
            num_patterns: 0,
            members: HashMap::new(),
            members_ordered: Vec::new()
        }
    }
    pub fn sc(&self) -> &str {
        self.snake_case.as_str()
    }
    pub fn needs_lifetime(&self, data: &LangData<'a>, visited: &mut HashSet<&'a str>) -> bool {
        if visited.contains(self.name) {
            false
        } else {
            visited.insert(self.name);
            self.members
                .values()
                .any(|member| member.tpe.needs_lifetime(data, visited))
        }
    }
    pub fn add_type(&self, mut s: String, data: &LangData<'a>) -> String {
        s += self.name;
        if self.needs_lifetime(data, &mut HashSet::new()) {
            s += "<'a>";
        }
        s
    }

    pub fn is_simple(&self) -> bool {
        self.members.len() == 0
    }
}

#[derive(Debug)]
pub struct AstEnum<'a> {
    pub name: &'a str,
    pub snake_case: String,
    pub items: Vec<&'a str>,
    pub boxed_items: HashSet<&'a str>,
}
impl<'a> AstEnum<'a> {
    pub fn new(name: &'a str, snake_case: String) -> AstEnum<'a> {
        AstEnum {
            name,
            snake_case,
            items: Vec::new(),
            boxed_items: HashSet::new(),
        }
    }

    pub fn needs_lifetime(&self, data: &LangData<'a>, visited: &mut HashSet<&'a str>) -> bool {
        if visited.contains(self.name) {
            false
        } else {
            visited.insert(self.name);
            if data.simple_enums.contains(self.name) {
                false
            } else {
                self.items
                    .iter()
                    .any(|item| data.resolve(item).needs_lifetime(data, visited))
            }
        }
    }

    pub fn add_type(&self, mut s: String, data: &LangData<'a>) -> String {
        s += self.name;
        if self.needs_lifetime(data, &mut HashSet::new()) {
            s += "<'a>";
        }
        s
    }

    pub fn is_simple(&self, data: &LangData<'a>) -> bool {
        if data.simple_enums.contains(self.name) {
            true
        } else {
            let mut is_simple = true;
            for item in &self.items {
                if !data.resolve(item).is_simple(data) {
                    is_simple = false;
                    break;
                }
            }
            is_simple
        }
    }

    pub fn sc(&self) -> &str {
        self.snake_case.as_str()
    }
}

// Hm does it make sense to split in single/many
// Was useful at one point to distinguish enums,
// but data.resolve finds ast type.
// It *is* needed to find "next key"
// in rules like listItems[] WS AstKey
#[derive(Debug)]
pub enum RuleType<'a> {
    SingleType(&'a str),
    ManyType(&'a str),
}
impl<'a> RuleType<'a> {
    pub fn get_type_name(&self, data: &LangData<'a>) -> &str {
        match self {
            &RuleType::SingleType(type_name) => match data.resolve(type_name) {
                ResolvedType::ResolvedEnum(key) => key,
                ResolvedType::ResolvedStruct(key) => key,
            },
            &RuleType::ManyType(type_name) => match data.resolve(type_name) {
                ResolvedType::ResolvedEnum(key) => key,
                ResolvedType::ResolvedStruct(key) => key,
            },
        }
    }

    pub fn needs_lifetime(&self, data: &LangData<'a>, visited: &mut HashSet<&'a str>) -> bool {
        match self {
            &RuleType::SingleType(type_name) => match data.resolve(type_name) {
                ResolvedType::ResolvedEnum(key) => {
                    data.ast_enums.get(key).unwrap().needs_lifetime(data, visited)
                }
                ResolvedType::ResolvedStruct(key) => {
                    data.ast_structs.get(key).unwrap().needs_lifetime(data, visited)
                }
            },
            &RuleType::ManyType(type_name) => match data.resolve(type_name) {
                ResolvedType::ResolvedEnum(key) => {
                    data.ast_enums.get(key).unwrap().needs_lifetime(data, visited)
                }
                ResolvedType::ResolvedStruct(key) => {
                    data.ast_structs.get(key).unwrap().needs_lifetime(data, visited)
                }
            },
        }
    }
}
