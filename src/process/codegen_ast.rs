use lang_data::data::*;
use lang_data::ast::*;
use std::fs::File;
use std::io::Write;
use util::SortedHashMap;

pub struct CodegenAst<'a, 'd: 'a> {
    data: &'a LangData<'d>
}
impl<'a, 'd> CodegenAst<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenAst<'a, 'd> {
        CodegenAst { data }
    }

    pub fn gen(&self) {
        // Try to allocate ideally enough to contain the source
        let mut s = String::with_capacity(
            25 * 3 * self.data.ast_structs.len()
            + 25 * 3 * self.data.ast_enums.len()
        );
        for (key, ast_struct) in self.data.ast_structs.sorted_iter() {
            append!(s, "pub struct " key "<'a> {\n");
            for (key, member) in &ast_struct.members {
                append!(s 1, "pub " member.sc() ": ");
                let tpe = self.data.typed_parts.get(member.part_key).unwrap();
                use lang_data::typed_part::TypedPart::*;
                let is_option = member.optional && match tpe {
                    &CharPart { .. } | &TagPart { .. } => false,
                    _ => true
                };
                if is_option { s += "Option<"; }
                match tpe {
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
                    &TagPart { .. } => s += "bool"
                }
                if is_option { s += ">"; }
                s += ",\n";
            }
            s += "}\n\n";
        }
        for (key, enum_data) in self.data.ast_enums.sorted_iter() {
            append!(s, "pub enum " key " {\n");
            for item in &enum_data.items {
                append!(s 1, item "Item(" item "),\n");
            }
            s += "}\n\n";
        }
        let mut file = File::create("gen/ast.rs").expect("Could not open file");
        file.write_all(s.as_bytes()).expect("Could not write ast file");
    }
}