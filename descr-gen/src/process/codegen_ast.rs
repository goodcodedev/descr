use lang_data::data::*;
use descr_common::util::SortedHashMap;

pub struct CodegenAst<'a, 'd: 'a> {
    data: &'a LangData<'d>
}
impl<'a, 'd> CodegenAst<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenAst<'a, 'd> {
        CodegenAst { data }
    }

    pub fn gen(&self) -> String {
        // Try to allocate ideally enough to contain the source
        let mut s = String::with_capacity(
            25 * 3 * self.data.ast_structs.len()
            + 25 * 3 * self.data.ast_enums.len()
        );
        for (_key, ast_struct) in self.data.ast_structs.sorted_iter() {
            append!(s, "#[derive(Debug)]\n");
            append!(s, "pub struct ");
            s = ast_struct.add_type(s);
            s += " {\n";
            for (_key, member) in ast_struct.members.sorted_iter() {
                append!(s 1, "pub " member.sc() ": ");
                // If member comes from quote (could be
                // interpreted as boolean todo), or
                // a not (!) part, it doesn't have a
                // "typed part" now.
                // There should possibly be an enum here
                // like TypedRulePart
                let tpe = if member.not {
                    None
                } else {
                    Some(self.data.typed_parts.get(member.part_key).unwrap())
                };
                use lang_data::typed_part::TypedPart::*;
                let is_option = member.optional && match tpe {
                    Some(tpe) => match tpe {
                        &CharPart { .. } | &TagPart { .. } => false,
                        _ => true
                    },
                    _ => false
                };
                if is_option { s += "Option<"; }
                if member.not {
                    s += "&'a str";
                } else {
                    match tpe {
                        Some(tpe) => match tpe {
                            &AstPart { key } => {
                                s += self.data.type_refs.get(key).unwrap().get_type_name();
                                s += "<'a>";
                            },
                            &ListPart { key } => {
                                s += "Vec<";
                                s += self.data.type_refs.get(key).unwrap().get_type_name();
                                s += "<'a>>";
                            },
                            &IntPart { .. } => s += "i32",
                            &IdentPart { .. } => s += "&'a str",
                            &CharPart { .. } => s += "bool",
                            &TagPart { .. } => s += "bool",
                            &StringPart { .. } => s += "&'a str",
                            &FnPart { tpe, .. } => s += tpe
                        },
                        _ => {
                            panic!("Not implemented")
                        }
                    }
                }
                if is_option { s += ">"; }
                s += ",\n";
            }
            s += "}\n\n";
        }
        for (_key, enum_data) in self.data.ast_enums.sorted_iter() {
            append!(s, "#[derive(Debug)]\n");
            append!(s, "pub enum ");
            s = enum_data.add_type(s);
            s += " {\n";
            for item in &enum_data.items {
                append!(s 1, item "Item(");
                s = self.data.add_ast_type(s, item);
                s += "),\n";
            }
            s += "}\n\n";
        }
        s
    }
}