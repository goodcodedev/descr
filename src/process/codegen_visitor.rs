use lang_data::data::*;
use std::fs::File;
use std::io::Write;
use util::*;

pub struct CodegenVisitor<'a, 'd: 'a> {
    pub data: &'a LangData<'d>
}
impl<'a, 'd> CodegenVisitor<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenVisitor<'a, 'd> {
        CodegenVisitor { data }
    }

    pub fn gen(&self) -> String {
        let mut s = String::with_capacity(
            1024
        );
        s += "use super::ast::*;\n\n";
        s += "pub trait Visitor<'a, 'b> {\n";
        // Ast structs
        for (key, ast_struct) in self.data.ast_structs.sorted_iter() {
            append!(s 1, "fn visit_" ast_struct.sc() "(&mut self, node: &'b " key ") {\n");
            for (key, member) in ast_struct.members.sorted_iter() {
                s = member.gen_visitor(s, ast_struct, self.data);
            }
            s += "    }\n\n";
        }
        // Ast enums
        for (key, ast_enum) in self.data.ast_enums.sorted_iter() {
            append!(s 1, "fn visit_" ast_enum.sc() "(&mut self, node: &'b " key ") {\n");
            append!(s 2, "match node {\n");
            for enum_item in &ast_enum.items {
                append!(s 3, "&" key "::" enum_item "Item(ref inner) => self.visit_" self.data.sc(enum_item) "(inner),\n");
            }
            s += "        }\n    }\n\n";
        }
        s += "}";
        s
    }
}