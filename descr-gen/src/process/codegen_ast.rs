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
            // Struct constructor
            s += "    pub fn new(";
            let num_members = ast_struct.members.len();
            for (i, m_ordered) in ast_struct.members_ordered.iter().enumerate() {
                let member = ast_struct.members.get(m_ordered)
                    .expect("Could not find struct member");
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
                let member = ast_struct.members.get(m_ordered)
                    .expect("Could not find struct member");
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
            append!(s 2, "}\n    }\n");
            // as_<enum> methods
            match self.data.parent_refs.refs.get(ast_struct.name) {
                Some(parent_refs) => {
                    for parent_ref in parent_refs.iter() {
                        match parent_ref {
                            &ParentRef::EnumItem{enum_name, item_name} => {
                                let ast_enum = self.data.ast_enums.get(enum_name)
                                    .expect("Could not find parent enum");
                                s += "\n";
                                let enum_needs_lifetime = ast_enum.needs_lifetime(self.data, &mut HashSet::new());
                                append!(s 1, "pub fn as_" ast_enum.sc());
                                if enum_needs_lifetime && !needs_lifetime {
                                    s += "<'a>";
                                }
                                s += "(self) -> ";
                                s = ast_enum.add_type(s, self.data);
                                s += " {\n";
                                append!(s 2, enum_name "::" item_name "Item(");
                                let is_boxed = ast_enum.boxed_items.contains(item_name);
                                if is_boxed {
                                    s += "Box::new(";
                                }
                                s += "self";
                                if is_boxed {
                                    s += ")";
                                }
                                s += ")\n";
                                append!(s 1, "}\n");
                            },
                            _ => {}
                        }
                    }
                },
                None => {}
            }
            s += "}\n\n";
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
                    match self.data.resolve(item) {
                        ResolvedType::ResolvedStruct(key) => {
                            s += "\n";
                            let ast_struct = self.data.ast_structs.get(key)
                                .expect("Could not find ast struct");
                            let num_members = ast_struct.members.len();
                            append!(s 1, "pub fn " self.data.sc(item) "(");
                            for (i, m_ordered) in ast_struct.members_ordered.iter().enumerate() {
                                let member = ast_struct.members.get(m_ordered)
                                    .expect("Could not find ast struct");
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
                                let member = ast_struct.members.get(m_ordered)
                                    .expect("Could not find struct member");
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
                        },
                        ResolvedType::ResolvedEnum(key) => {}
                    }
                }
                s += "}\n\n";
            }
        }
        s
    }
}
