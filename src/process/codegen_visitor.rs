use lang_data::*;
use std::fs::File;
use std::io::Write;
use util::SortedHashMap;

pub struct CodegenVisitor<'a, 'd: 'a> {
    pub data: &'a LangData<'d>
}
impl<'a, 'd> CodegenVisitor<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenVisitor<'a, 'd> {
        CodegenVisitor { data }
    }

    pub fn gen(&self) {
        let mut s = String::with_capacity(
            1024
        );
        s += "trait Visitor {\n";
        for (key, ast_struct) in self.data.ast_structs.sorted_iter() {
            s += "    pub fn visit_";
            s += key;
            s += "(node: ";
            s += key;
            s += ") {\n";
            for (key, member) in ast_struct.members.sorted_iter() {
                s += "        ";
                if member.optional {
                    s += "self.";
                    s += key;
                    s += " match {\n";
                    s += "            Some(ref inner) => self.visit_";
                    s += member.name;
                    s += "(inner)";
                    s += "),\n";
                    s += "            None => {}\n        }\n";
                } else {
                    s += "self.visit_";
                    s += member.name;
                    s += "(node.";
                    s += key;
                    s += ");\n";
                }
            }
            s += "    }\n\n";
        }
        for (key, ast_enum) in self.data.ast_enums.sorted_iter() {
            s += "    pub fn visit_";
            s += key;
            s += "(node: ";
            s += key;
            s += ") {\n";
            s += "        match node {\n";
            for enum_item in &ast_enum.items {
                s += "            ";
                s += enum_item;
                s += "Item(ref inner) => self.visit_";
                s += enum_item;
                s += "(inner);\n";
            }
            s += "        }\n    }\n\n";
        }
        s += "}";
        let mut file = File::create("gen/visitor.rs").expect("Could not open file");
        file.write_all(s.as_bytes()).expect("Could not write to file");
    }
}