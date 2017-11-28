use lang_data::data::*;
use lang_data::ast::*;
use lang_data::rule::*;
use util::*;

#[derive(Debug)]
pub enum TypedPart<'a> {
    AstPart     { key: &'a str },
    ListPart    { key: &'a str },
    CharPart    { key: &'a str, chr: char },
    TagPart     { key: &'a str, tag: &'a str },
    IntPart     { key: &'a str },
    IdentPart   { key: &'a str }
}
impl<'a> TypedPart<'a> {
    pub fn gen_parser(&self, mut s: String) -> String {
        use lang_data::typed_part::TypedPart::*;
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

    pub fn gen_parser_val(&self, mut s: String, part: &'a AstRulePart) -> String {
        use lang_data::typed_part::TypedPart::*;
        let member_key = part.member_key.unwrap();
        match self {
            &AstPart { .. }
            | &ListPart { .. } => {
                append!(s, member_key "_k");
            }
            &CharPart { .. } => {
                if part.optional {
                    append!(s, member_key "_k.is_some()");
                }
            },
            &TagPart { .. } => {
                if part.optional {
                    append!(s, member_key "_k.is_some()");
                }
            },
            &IntPart { .. } => {
                append!(s, member_key "_k");
            },
            &IdentPart { .. } => {
                append!(s, member_key "_k");
            }
        }
        s
    }

    pub fn gen_visitor(&self, mut s: String, member: &AstStructMember,
                       ast_struct: &AstStruct, data: &LangData) -> String {
        use lang_data::typed_part::TypedPart::*;
        match self {
            &AstPart { key } => {
                if member.optional {
                    append!(s 2, "self." member.sc() "match {\n");
                    append!(s 3, "Some(ref inner) => self.visit_" key "(inner),\n");
                    append!(s 3, "None => {}\n");
                    append!(s 2, "}\n");
                } else {
                    append!(s 2, "self.visit_" data.sc(key) "(node." member.sc() ");\n");
                }
            },
            &ListPart { key } => {
                if member.optional {
                    append!(s 2, "self." member.sc() " match {\n");
                    append!(s 3, "Some(ref inner) => {\n");
                    append!(s 4, "for item in &inner {\n");
                    match data.type_refs.get(key).unwrap() {
                        &AstType::AstStruct(ref type_name) => {
                            append!(s 5, "self.visit_" data.sc(type_name) "(item);\n");
                        },
                        &AstType::AstEnum(ref type_name) => {
                            append!(s 5, "self.visit_" data.sc(type_name) "(item);\n");
                        }
                    }
                    append!(s 4, "}\n");
                } else {
                    append!(s 2, "for item in &node." member.sc() " {\n");
                    match data.type_refs.get(key).unwrap() {
                        &AstType::AstStruct(ref type_name) => {
                            append!(s 3, "self.visit_" data.sc(type_name) "(item);\n");
                        },
                        &AstType::AstEnum(ref type_name) => {
                            append!(s 3, "self.visit_" data.sc(type_name) "(item);\n");
                        }
                    }
                    append!(s 2, "}\n");
                }
            },
            _ => {}
        }
        s
    }
}