use lang_data::data::*;
use lang_data::ast::*;
use lang_data::rule::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum TypedPart<'a> {
    AstPart {
        key: &'a str,
    },
    ListPart {
        key: &'a str,
    },
    CharPart {
        key: &'a str,
        chr: char,
    },
    TagPart {
        key: &'a str,
        tag: &'a str,
    },
    IntPart {
        key: &'a str,
    },
    IdentPart {
        key: &'a str,
    },
    FnPart {
        key: &'a str,
        fnc: &'a str,
        tpe: &'a str,
    },
    StringPart {
        key: &'a str,
    },
    StrPart {
        key: &'a str,
    },
    WSPart,
}
impl<'a> TypedPart<'a> {
    pub fn is_auto_member(&self) -> bool {
        match self {
            &TypedPart::AstPart { .. }
            | &TypedPart::ListPart { .. }
            | &TypedPart::IntPart { .. }
            | &TypedPart::StringPart { .. }
            | &TypedPart::StrPart { .. }
            | &TypedPart::IdentPart { .. } => true,
            _ => false,
        }
    }

    pub fn gen_parser(&self, mut s: String, data: &LangData) -> String {
        use lang_data::typed_part::TypedPart::*;
        if data.debug {
            s += "debug_wrap!(";
        }
        match self {
            &AstPart { key } => {
                s += data.sc(key);
            }
            &ListPart { key } => {
                s += data.sc(key);
            }
            &CharPart { chr, .. } => {
                s += "char!('";
                s.push(chr);
                s += "')";
            }
            &TagPart { tag, .. } => {
                append!(s, "tag!(\"" tag "\")");
            }
            &IntPart { .. } => {
                s += "parse_int";
            }
            &IdentPart { .. } => {
                s += "ident";
            }
            &FnPart { fnc, .. } => {
                s += fnc;
            }
            &StringPart { .. } => {
                s += "quoted_str";
            }
            &StrPart { .. } => {
                s += "quoted_str";
            }
            &WSPart => s += "sp",
        }
        if data.debug {
            s += ")";
        }
        s
    }

    pub fn gen_parser_val(&self, mut s: String, part: &'a AstRulePart, data: &LangData) -> String {
        use lang_data::typed_part::TypedPart::*;
        let member_key = data.sc(part.member_key.unwrap());
        match self {
            &AstPart { .. } | &ListPart { .. } => {
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
            }
            &TagPart { .. } => {
                if part.optional {
                    append!(s, member_key "_k.is_some()");
                }
            }
            &IntPart { .. } => {
                append!(s, member_key "_k");
            }
            &IdentPart { .. } => {
                append!(s, member_key "_k");
            }
            &FnPart { .. } => {
                append!(s, member_key "_k");
            }
            &StringPart { .. } => {
                append!(s, "String::from(" member_key "_k)");
            }
            &StrPart { .. } => {
                append!(s, member_key "_k");
            }
            &WSPart => {
                append!(s, member_key "_k");
            }
        }
        s
    }

    pub fn gen_visitor(
        &self,
        mut s: String,
        member: &AstStructMember,
        _ast_struct: &AstStruct,
        data: &LangData,
    ) -> String {
        use lang_data::typed_part::TypedPart::*;
        match self {
            &AstPart { key } => {
                if member.optional {
                    append!(s 2, "match node." member.sc() " {\n");
                    append!(s 3, "Some(ref inner) => self.visit_" data.sc(key) "(inner),\n");
                    append!(s 3, "None => {}\n");
                    append!(s 2, "}\n");
                } else {
                    append!(s 2, "self.visit_" data.sc(key) "(&node." member.sc() ");\n");
                }
            }
            &ListPart { key } => {
                if member.optional {
                    append!(s 2, "node." member.sc() " match {\n");
                    append!(s 3, "Some(ref inner) => {\n");
                    append!(s 4, "for item in &inner {\n");
                    match data.rule_types.get(key).unwrap() {
                        &RuleType::SingleType(ref type_name) => {
                            append!(s 5, "self.visit_" data.sc(type_name) "(item);\n");
                        }
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
                        }
                        &RuleType::ManyType(ref type_name, ..) => {
                            append!(s 3, "self.visit_" data.sc(type_name) "(item);\n");
                        }
                    }
                    append!(s 2, "}\n");
                }
            }
            _ => {}
        }
        s
    }

    pub fn needs_lifetime(&self, data: &LangData<'a>, visited: &mut HashSet<&'a str>) -> bool {
        use lang_data::typed_part::TypedPart::*;
        match self {
            &AstPart { key } => {
                let rule_type = data.rule_types
                    .get(key)
                    .expect(&format!("Coult not get ast {}", key));
                rule_type.needs_lifetime(data, visited)
            }
            &ListPart { key } => {
                let rule_type = data.rule_types
                    .get(key)
                    .expect(&format!("Coult not get list {}", key));
                rule_type.needs_lifetime(data, visited)
            }
            &CharPart { .. } => false,
            &TagPart { .. } => false,
            &IntPart { .. } => false,
            &IdentPart { .. } => true,
            // Depends on function todo
            &FnPart { .. } => true,
            &StringPart { .. } => false,
            &StrPart { .. } => true,
            &WSPart => true,
        }
    }

    pub fn is_option(&self, member: &AstStructMember<'a>) -> bool {
        use self::TypedPart::*;
        if member.optional {
            match self {
                &CharPart { .. } | &TagPart { .. } => false,
                _ => true,
            }
        } else {
            false
        }
    }

    pub fn add_to_source(
        &self, 
        mut s: String, 
        member_key: Option<&'a str>,
        optional: bool,
        data: &LangData<'a>) -> String {
        match self {
            &TypedPart::AstPart{key} => {
                if let Some(member_key) = member_key {
                    if optional {
                        append!(s 2, "if let Some(ref some_val) = node." data.sc(member_key) " {\n    ");
                    }
                    let ast_type = data.resolve(key).get_ast_type();
                    append!(s 2, "s = Self::" data.sc(ast_type) "(s, ");
                    if optional {
                        s += "some_val);\n";
                        s += "        }\n";
                    } else {
                        append!(s, "&node." data.sc(member_key) ");\n");
                    }
                }
            },
            &TypedPart::ListPart{key} => {
                if let Some(member_key) = member_key {
                    let list_data = data.list_data.get(key).unwrap();
                    let sep_part = list_data.sep.map(|sep_key| { data.typed_parts.get(sep_key).unwrap() });
                    if optional {
                        append!(s 2, "if let Some(ref some_val) = node." data.sc(member_key) " {\n    ");
                    }
                    let ast_type = data.resolve(key).get_ast_type();
                    append!(s 2, "let len = ");
                    if optional { s += "some_val"; } else { append!(s, "node." data.sc(member_key)); }
                    s += ".len();\n";
                    append!(s 2, "for (i, item) in ");
                    if optional { s += "some_val"; } else { append!(s, "node." data.sc(member_key)); }
                    s += ".iter().enumerate() {\n";
                    append!(s 3, "s = Self::" data.sc(ast_type) "(s, item);\n");
                    if let Some(sep_part) = sep_part {
                        append!(s 3, "if i < len - 1 { ");
                        s = sep_part.add_to_source(s, None, false, data);
                        s += " }\n";
                    }
                    append!(s 2, "}\n");
                    if optional {
                        s += "    }\n";
                    }
                }
            },
            &TypedPart::IntPart{..} => {
                if let Some(member_key) = member_key {
                    if optional {
                        append!(s 2, "if let Some(some_val) = node." member_key " {\n    ");
                        append!(s 3, "s += &some_val.to_string();\n");
                        s += "        }";
                    } else {
                        append!(s 2, "s += &node." member_key ".to_string();\n");
                    }
                }
            },
            &TypedPart::StringPart{..} => {
                if let Some(member_key) = member_key {
                    if optional {
                        append!(s 2, "if let Some(some_val) = node." data.sc(member_key) " {\n    ");
                        append!(s 3, "s += \"\\\"\";\n");
                        append!(s 3, "s += some_val.as_str();\n");
                        append!(s 3, "s += \"\\\"\";\n");
                        s += "        }";
                    } else {
                        append!(s 2, "s += \"\\\"\";\n");
                        append!(s 2, "s += node." data.sc(member_key) ".as_str();\n");
                        append!(s 2, "s += \"\\\"\";\n");
                    }
                }
            },
            &TypedPart::StrPart{..} => {
                if let Some(member_key) = member_key {
                    if optional {
                        append!(s 2, "if let Some(some_val) = node." member_key " {\n    ");
                        append!(s 3, "s += \"\\\"\";\n");
                        append!(s 3, "s += some_val;\n");
                        append!(s 3, "s += \"\\\"\";\n");
                        s += "        }";
                    } else {
                        append!(s 2, "s += \"\\\"\";\n");
                        append!(s 2, "s += node." member_key ";\n");
                        append!(s 2, "s += \"\\\"\";\n");
                    }
                }
            },
            &TypedPart::IdentPart{..}
            | &TypedPart::FnPart{..} // Todo: Fn should probably be able to handle own
             => {
                if let Some(member_key) = member_key {
                    if optional {
                        append!(s 2, "if let Some(some_val) = node." member_key " {\n    ");
                        append!(s 3, "s += some_val;\n");
                        s += "        }";
                    } else {
                        append!(s 2, "s += node." member_key ";\n");
                    }
                }
            },
            &TypedPart::CharPart{chr, ..} => {
                if let Some(member_key) = member_key {
                    // Assuming parsed to bool
                    append!(s 2, "if node." member_key " {\n    ");
                    append!(s 3, "s.push('");
                    s.push(chr);
                    s += "');\n";
                    s += "    }";
                } else {
                    append!(s 2, "s.push('");
                    s.push(chr);
                    s += "');\n";
                }
            },
            &TypedPart::TagPart{tag, ..} => {
                if let Some(member_key) = member_key {
                    // Assuming parsed to bool
                    append!(s 2, "if node." member_key " {\n    ");
                    append!(s 3, "s += \"" tag "\";\n");
                    s += "    }";
                } else {
                    append!(s 2, "s += \"" tag "\";\n");
                }
            },
            &TypedPart::WSPart => {
                append!(s 2, "s += \" \"");
            }
        }
        s
    }

    pub fn add_type(
        &self,
        mut s: String,
        data: &LangData<'a>,
    ) -> String {
        use self::TypedPart::*;
        match self {
            &AstPart { key } => {
                s += data.rule_types
                    .get(key)
                    .expect(&format!("Coult not get ast {}", key))
                    .get_type_name(data);
                if self.needs_lifetime(data, &mut HashSet::new()) {
                    s += "<'a>";
                }
                s
            }
            &ListPart { key } => {
                s += "Vec<";
                s += data.rule_types
                    .get(key)
                    .expect(&format!("Coult not get list {}", key))
                    .get_type_name(data);
                if self.needs_lifetime(data, &mut HashSet::new()) {
                    s += "<'a>";
                }
                s += ">";
                s
            }
            &IntPart { .. } => {
                s += "u32";
                s
            }
            &IdentPart { .. } => {
                s += "&'a str";
                s
            }
            &CharPart { .. } => {
                s += "bool";
                s
            }
            &TagPart { .. } => {
                s += "bool";
                s
            }
            &StringPart { .. } => {
                s += "String";
                s
            }
            &StrPart { .. } => {
                s += "&'a str";
                s
            }
            &FnPart { tpe, .. } => {
                s += tpe;
                s
            }
            &WSPart => {
                s += "&'a str";
                s
            }
        }
    }
}
