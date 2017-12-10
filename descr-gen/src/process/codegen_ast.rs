use lang_data::data::*;
use descr_common::util::SortedHashMap;
use std::collections::HashSet;

pub struct CodegenAst<'a, 'd: 'a> {
    data: &'a LangData<'d>,
}
impl<'a, 'd> CodegenAst<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenAst<'a, 'd> {
        CodegenAst { data }
    }

    pub fn gen(&self) -> String {
        // Try to allocate ideally enough to contain the source
        let mut s = String::with_capacity(
            25 * 3 * self.data.ast_structs.len() + 25 * 3 * self.data.ast_enums.len(),
        );
        for (key, ast_struct) in self.data.ast_structs.sorted_iter() {
            if self.data.simple_structs.contains(key) {
                continue;
            }
            append!(s, "#[derive(Debug)]\n");
            append!(s, "pub struct ");
            s = ast_struct.add_type(s, self.data);
            s += " {\n";
            for (_key, member) in ast_struct.members.sorted_iter() {
                append!(s 1, "pub " member.sc() ": ");
                let is_option = member.tpe.is_option(member, self.data);
                if is_option {
                    s += "Option<";
                }
                if member.boxed {
                    s += "Box<";
                }
                s = member.tpe.add_type(s, self.data);
                if member.boxed {
                    s += ">";
                }
                if is_option {
                    s += ">";
                }
                s += ",\n";
            }
            s += "}\n\n";
            s += "impl";
            let needs_lifetime = ast_struct.needs_lifetime(self.data, &mut HashSet::new());
            if needs_lifetime {
                s += "<'a>";
            }
            s += " ";
            s = ast_struct.add_type(s, self.data);
            s += " {\n";
            s += "    pub fn new(";
            let num_members = ast_struct.members.len();
            for (i, m_ordered) in ast_struct.members_ordered.iter().enumerate() {
                let member = ast_struct.members.get(m_ordered).unwrap();
                append!(s, member.sc() ": ");
                s = member.tpe.add_type(s, self.data);
                if i < num_members - 1 {
                    s += ", ";
                }
            }
            s += ") -> ";
            s = ast_struct.add_type(s, self.data);
            s += " {\n";
            append!(s 2, ast_struct.name " {\n");
            for (i, m_ordered) in ast_struct.members_ordered.iter().enumerate() {
                let member = ast_struct.members.get(m_ordered).unwrap();
                if member.boxed {
                    append!(s 3, member.sc() ": Box::new(" member.sc() ")\n");
                } else {
                    append!(s 3, member.sc());
                }
                if i < num_members - 1 {
                    s += ",\n";
                } else {
                    s += "\n";
                }
            }
            append!(s 2, "}\n");
            s += "    }\n}\n\n";
        }
        for (key, enum_data) in self.data.ast_enums.sorted_iter() {
            let is_simple = self.data.simple_enums.contains(key);
            append!(s, "#[derive(Debug)]\n");
            append!(s, "pub enum ");
            s = enum_data.add_type(s, self.data);
            s += " {\n";
            for item in &enum_data.items {
                if is_simple {
                    append!(s 1, item ",\n");
                } else {
                    append!(s 1, item "Item(");
                    let is_boxed = enum_data.boxed_items.contains(item);
                    if is_boxed {
                        s += "Box<";
                    }
                    s = self.data.add_ast_type(s, item);
                    if is_boxed {
                        s += ">";
                    }
                    s += "),\n";
                }
            }
            s += "}\n\n";
            if !is_simple {
                // Enum impl, with methods to create each item
                s += "impl";
                let needs_lifetime = enum_data.needs_lifetime(self.data, &mut HashSet::new());
                if needs_lifetime {
                    s += "<'a>";
                }
                s += " ";
                s = enum_data.add_type(s, self.data);
                s += " {";
                for item in &enum_data.items {
                    s += "\n";
                    let ast_struct = self.data.ast_structs.get(item).unwrap();
                    let num_members = ast_struct.members.len();
                    append!(s 1, "pub fn " self.data.sc(item) "(");
                    for (i, m_ordered) in ast_struct.members_ordered.iter().enumerate() {
                        let member = ast_struct.members.get(m_ordered).unwrap();
                        append!(s, member.sc() ": ");
                        s = member.tpe.add_type(s, self.data);
                        if i < num_members - 1 {
                            s += ", ";
                        }
                    }
                    s += ") -> ";
                    s = enum_data.add_type(s, self.data);
                    s += " {\n";
                    append!(s 2, enum_data.name "::" item "Item(");
                    let is_boxed = enum_data.boxed_items.contains(item);
                    if is_boxed {
                        s += "Box::new(";
                    }
                    // Ast constructor created above
                    append!(s, item "::new(");
                    for (i, m_ordered) in ast_struct.members_ordered.iter().enumerate() {
                        let member = ast_struct.members.get(m_ordered).unwrap();
                        s += member.sc();
                        if i < num_members - 1 {
                            s += ", ";
                        }
                    }
                    s += "))";
                    if is_boxed {
                        s += ")";
                    }
                    s += "\n    }\n";
                }
                s += "}\n\n";
            }
        }
        s
    }
}
