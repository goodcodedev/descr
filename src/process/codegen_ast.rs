use lang_data::*;
use std::fs::File;
use std::io::Write;
use std::mem;

pub struct CodegenAst<'a> {
    data: &'a LangData<'a>
}
impl<'a> CodegenAst<'a> {
    pub fn new(data: &'a LangData<'a>) -> CodegenAst<'a> {
        CodegenAst { data }
    }

    pub fn gen(&self) {
        let mut s = String::new();
        for (key, ast_struct) in &self.data.ast_structs {
            s += "pub struct ";
            s += key;
            s += " {\n";
            for (key, member) in &ast_struct.members {
                s += "    pub ";
                s += key;
                s += ": ";
                let tpe = self.data.typed_parts.get(member.token_key).unwrap();
                use lang_data::TypedPart::*;
                match tpe {
                    &AstPart { key } => s += key,
                    &ListPart { key } => s += key,
                    &IntPart { .. } => s += "i32",
                    &IdentPart { .. } => s += "String",
                    &CharPart { .. } => s += "bool",
                    &TagPart { .. } => s += "bool"
                }
                s += ",\n";
            }
            s += " }\n\n";
        }
        for (key, enum_data) in &self.data.ast_enums {
            s += "pub enum ";
            s += key;
            s += " {\n";
            for item in &enum_data.items {
                s += "    ";
                s += item;
                s += "Item(";
                s += item;
                s += "),\n";
            }
            s += "}\n\n";
        }
        let mut file = File::create("gen/ast.rs").expect("Could not open file");
        file.write_all(s.as_bytes()).expect("Could not write ast file");
    }
}