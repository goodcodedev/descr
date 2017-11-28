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
                    append!(s 2, "self." member.name "match {\n");
                    append!(s 3, "Some(ref inner) => self.visit_" key "(inner),\n");
                    append!(s 3, "None => {}\n");
                    append!(s 2, "}\n");
                } else {
                    append!(s 1, "self.visit_" key "(node." member.name ");\n");
                }
            },
            &ListPart { key } => {
                if member.optional {
                    append!(s 2, "self." member.name " match {\n");
                    append!(s 3, "Some(ref inner) => {\n");
                    append!(s 4, "for item in &inner {\n");
                    let list_data = data.list_data.get(key).unwrap();
                    match data.ast_enums.get(list_data.key) {
                        Some(ref ast_enum) => {
                            append!(s 5, "match item {\n");
                            for item in &ast_enum.items {
                                append!(s 6, item "Item(ref inner) => self.visit_" item "(inner);\n");
                            }
                            append!(s 5, "}\n");
                        },
                        None => {
                            // Regular ast node
                            append!(s 5, "self.visit_" key "(item);\n");
                        }
                    };
                } else {
                    append!(s 2, "for item in &self." member.name "{\n");
                    append!(s 3, "self.visit_" key "(item);\n");
                    append!(s 2, "}\n");
                }
            },
            _ => {}
        }
        s
    }
}