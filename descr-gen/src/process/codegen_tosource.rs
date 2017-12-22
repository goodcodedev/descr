use lang_data::data::*;
use lang_data::rule::*;
use descr_common::util::*;
use std::collections::HashMap;

pub struct CodegenToSource<'a, 'd: 'a> {
    data: &'a LangData<'d>
}
struct AstRules<'a, 'd: 'a> {
    pub rules: HashMap<&'a str, Vec<&'a AstPartsRule<'d>>>
}
impl<'a, 'd: 'a> AstRules<'a, 'd> {
    pub fn insert(&mut self, ast_type: &'d str, rule: &'a AstPartsRule<'d>) {
        if !self.rules.contains_key(ast_type) {
            self.rules.insert(ast_type, Vec::new());
        }
        self.rules.get_mut(ast_type).unwrap().push(rule);
    }
}
impl<'a, 'd> CodegenToSource<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenToSource<'a, 'd> {
        CodegenToSource { data }
    }

    fn collect_rules(&self) -> AstRules<'a, 'd> {
        let mut rules = AstRules { rules: HashMap::new() };
        // Add rules by ast key
        for (_key, ast_data) in &self.data.ast_data {
            for rule in &ast_data.rules {
                if let &AstRule::PartsRule(ref parts_rule) = rule {
                    rules.insert(parts_rule.ast_type, parts_rule);
                }
            }
        }
        for (_key, list_data) in &self.data.list_data {
            for rule in &list_data.rules {
                if let &AstRule::PartsRule(ref parts_rule) = &rule.ast_rule {
                    rules.insert(parts_rule.ast_type, parts_rule);
                }
            }
        }
        rules
    }

    pub fn gen(&self) -> String {
        let mut s = String::with_capacity(
            self.data.ast_data.len() * 100
            + self.data.list_data.len() * 100
        );
        let ast_rules = self.collect_rules();
        // Create code for each rule under
        // function for ast type
        s += "use super::ast::*;\n\n";
        s += "pub struct ToSource;\n";
        s += "#[allow(unused_variables,dead_code)]\n";
        s += "impl<'a> ToSource {\n";
        for (ast_type, rules) in &ast_rules.rules {
            if self.data.simple_structs.contains(ast_type) {
                continue;
            }
            append!(s, "    pub fn " self.data.sc(ast_type) "(mut s: String, node: &'a " ast_type ") -> String {\n");
            for rule in rules {
                // Todo: Possibly create if statement
                // if there are several rules for the
                // same type, comparing optional
                // members at least
                for part in &rule.parts {
                    s += "        s += \" \";\n";
                    s = part.add_to_source(s, self.data, false);
                }
            }
            s += "        s\n";
            s += "    }\n\n";
        }
        // Ast enums
        for (key, ast_enum) in self.data.ast_enums.sorted_iter() {
            let is_simple = self.data.simple_enums.contains(key);
            append!(s 1, "pub fn " ast_enum.sc() "(");
            if is_simple {
                s += "mut ";
            }
            append!(s, "s: String, node: &'a " ast_enum.name ") -> String {\n");
            if is_simple {
                append!(s 2, "match node {\n");
                for enum_item in &ast_enum.items {
                    append!(s 3, "&" ast_enum.name "::" enum_item " => {\n");
                    indent!(s 4);
                    let rules = ast_rules.rules.get(enum_item).unwrap();
                    for rule in rules {
                        for part in &rule.parts {
                            s += "s += \" \";\n";
                            s = part.add_to_source(s, self.data, false);
                        }
                    }
                    append!(s 3, "},\n");
                }
                s += "        }\n";
                s += "        s\n";
                s += "    }\n\n";
                continue;
            }
            append!(s 2, "match node {\n");
            for enum_item in &ast_enum.items {
                append!(s 3, "&" ast_enum.name "::" enum_item "Item(ref inner) => Self::" self.data.sc(enum_item) "(s, inner),\n");
            }
            s += "        }\n    }\n\n";
        }
        s += "}";
        s
    }
}