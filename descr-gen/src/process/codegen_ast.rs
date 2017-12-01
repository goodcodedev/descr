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
            s = ast_struct.add_type(s, self.data);
            s += " {\n";
            for (_key, member) in ast_struct.members.sorted_iter() {
                append!(s 1, "pub " member.sc() ": ");
                let is_option = member.tpe.is_option(member, self.data);
                if is_option { s += "Option<"; }
                s = member.tpe.add_type(s, member, self.data);
                if is_option { s += ">"; }
                s += ",\n";
            }
            s += "}\n\n";
        }
        for (_key, enum_data) in self.data.ast_enums.sorted_iter() {
            append!(s, "#[derive(Debug)]\n");
            append!(s, "pub enum ");
            s = enum_data.add_type(s, self.data);
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