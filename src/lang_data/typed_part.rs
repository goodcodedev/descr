use lang_data::ast::*;
use lang_data::rule::*;

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

    pub fn gen_val(&self, mut s: String, part: &'a AstRulePart) -> String {
        use lang_data::typed_part::TypedPart::*;
        let member_key = part.member_key.unwrap();
        match self {
            &AstPart { .. }
            | &ListPart { .. } => {
                s += member_key;
                s += "_k";
            }
            &CharPart { .. } => {
                if part.optional {
                    s += member_key;
                    s += "_k.is_some()";
                }
            },
            &TagPart { .. } => {
                if part.optional {
                    s += member_key;
                    s += "_k.is_some()";
                }
            },
            &IntPart { .. } => {
                s += member_key;
                s += "_k";
            },
            &IdentPart { .. } => {
                s += member_key;
                s += "_k";
            }
        }
        s
    }
}