use lang_data::data::*;
use lang_data::ast::*;
use lang_data::rule::*;

#[derive(Debug)]
pub enum TypedPart<'a> {
    AstPart     { key: &'a str },
    ListPart    { key: &'a str },
    CharPart    { key: &'a str, chr: char },
    TagPart     { key: &'a str, tag: &'a str },
    IntPart     { key: &'a str },
    IdentPart   { key: &'a str },
    FnPart      { key: &'a str, fnc: &'a str, tpe: &'a str }
}
impl<'a> TypedPart<'a> {
    pub fn gen_parser(&self, mut s: String, data: &LangData) -> String {
        use lang_data::typed_part::TypedPart::*;
        match self {
            &AstPart { key } => { s += data.sc(key); },
            &ListPart { key } => { s += data.sc(key); },
            &CharPart { chr, .. } => {
                s += "char!('";
                s.push(chr);
                s += "')";
            },
            &TagPart { tag, .. } => { append!(s, "tag!(\"" tag "\")"); },
            &IntPart { .. } => { s += "int"; },
            &IdentPart { .. } => { s += "ident"; },
            &FnPart { fnc, .. } => { s += fnc; }
        }
        s
    }

    pub fn gen_parser_val(&self, mut s: String, part: &'a AstRulePart, data: &LangData) -> String {
        use lang_data::typed_part::TypedPart::*;
        let member_key = data.sc(part.member_key.unwrap());
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
            },
            &FnPart { .. } => {
                append!(s, member_key "_k");
            }
        }
        s
    }

    pub fn gen_visitor(&self, mut s: String, member: &AstStructMember,
                       _ast_struct: &AstStruct, data: &LangData) -> String {
        use lang_data::typed_part::TypedPart::*;
        match self {
            &AstPart { key } => {
                if member.optional {
                    append!(s 2, "self." member.sc() "match {\n");
                    append!(s 3, "Some(ref inner) => self.visit_" key "(inner),\n");
                    append!(s 3, "None => {}\n");
                    append!(s 2, "}\n");
                } else {
                    append!(s 2, "self.visit_" data.sc(key) "(&node." member.sc() ");\n");
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