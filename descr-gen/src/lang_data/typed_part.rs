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
    FnPart      { key: &'a str, fnc: &'a str, tpe: &'a str },
    StringPart  { key: &'a str }
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
            &IntPart { .. } => { s += "parse_int"; },
            &IdentPart { .. } => { s += "ident"; },
            &FnPart { fnc, .. } => { s += fnc; },
            &StringPart { .. } => { s += "quoted_str"; }
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
                } else {
                    // Not sure if it would make
                    // sense to store char.
                    // True captures that the pattern
                    // matched.
                    append!(s, "true");
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
            },
            &StringPart { .. } => {
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
                    match data.rule_types.get(key).unwrap() {
                        &RuleType::SingleType(ref type_name) => {
                            append!(s 5, "self.visit_" data.sc(type_name) "(item);\n");
                        },
                        &RuleType::ManyType(ref type_name, ..) => {
                            append!(s 5, "self.visit_" data.sc(type_name) "(item);\n");
                        }
                    }
                    append!(s 4, "}\n");
                } else {
                    append!(s 2, "for item in &node." member.sc() " {\n");
                    match data.rule_types.get(key).unwrap() {
                        &RuleType::SingleType(ref type_name) => {
                            append!(s 3, "self.visit_" data.sc(type_name) "(item);\n");
                        },
                        &RuleType::ManyType(ref type_name, ..) => {
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

    pub fn needs_lifetime(&self, data: &LangData<'a>) -> bool {
        use lang_data::typed_part::TypedPart::*;
        match self {
            &AstPart { key } => {
                let rule_type = data.rule_types.get(key).expect(&format!("Coult not get ast {}", key));
                rule_type.needs_lifetime(data)
            },
            &ListPart { key } => {
                let rule_type = data.rule_types.get(key).expect(&format!("Coult not get list {}", key));
                rule_type.needs_lifetime(data)
            },
            &CharPart { .. } => false,
            &TagPart { .. } => false,
            &IntPart { .. } => false,
            &IdentPart { .. } => true,
            // Depends on function todo
            &FnPart { .. } => true,
            &StringPart { .. } => true
        }
    }

    pub fn is_option(&self, member: &AstStructMember<'a>) -> bool {
        use self::TypedPart::*;
        if member.optional {
            match self {
                &CharPart { .. } | &TagPart { .. } => false,
                _ => true
            }
        } else {
            false
        }
    }

    pub fn add_type(&self, mut s: String, 
                    member: &AstStructMember<'a>, data: &LangData<'a>) 
                    -> String
    {
        use self::TypedPart::*;
        match self {
            &AstPart { key } => {
                s += data.rule_types.get(key).expect(&format!("Coult not get ast {}", key)).get_type_name(data);
                if member.tpe.needs_lifetime(data) {
                    s += "<'a>";
                }
                s
            },
            &ListPart { key } => {
                s += "Vec<";
                s += data.rule_types.get(key).expect(&format!("Coult not get list {}", key)).get_type_name(data);
                if member.tpe.needs_lifetime(data) {
                    s += "<'a>";
                }
                s += ">";
                s
            },
            &IntPart { .. }     => { s += "i32"; s },
            &IdentPart { .. }   => { s += "&'a str"; s },
            &CharPart { .. }    => { s += "bool"; s },
            &TagPart { .. }     => { s += "bool"; s },
            &StringPart { .. }  => { s += "&'a str"; s },
            &FnPart { tpe, .. } => { s += tpe; s }
        }
    }
}