use lang_data::data::*;
use std::fs::File;
use std::io::Write;
use util::SortedHashMap;

pub struct CodegenParsers<'a, 'd: 'a> {
    data: &'a LangData<'d>
}
impl<'a, 'd> CodegenParsers<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenParsers<'a, 'd> {
        CodegenParsers { data }
    }

    pub fn gen(&self) {
        let mut s = String::with_capacity(
            self.data.ast_data.len() * 100
            + self.data.list_data.len() * 100
        );
        // Ast data
        for (key, ast_data) in self.data.ast_data.sorted_iter() {
            match ast_data.rules.len() {
                0 => {},
                1 => {
                    let rule = ast_data.rules.first().unwrap();
                    s += "named!(";
                    s += ast_data.ast_type;
                    s += "<";
                    s += ast_data.ast_type;
                    s += ">,\n    ";
                    s = rule.gen_rule(s, ast_data.ast_type, self.data, false);
                    s += "\n);\n\n";
                },
                len => {
                    // Alt rule
                    s += "named!(";
                    s += ast_data.ast_type;
                    s += "<";
                    s += ast_data.ast_type;
                    s += ">, alt_complete!(\n    ";
                    for (i, rule) in ast_data.rules.iter().enumerate() {
                        s = rule.gen_rule(s, ast_data.ast_type, self.data, true);
                        if i < len - 1 {
                            s += "\n    | ";
                        }
                    }
                    s += "\n));\n\n";
                }
            }
        }
        // List data
        for (key, list_data) in self.data.list_data.sorted_iter() {
            match list_data.rules.len() {
                0 => {},
                1 => {
                    let rule = list_data.rules.first().unwrap();
                    s += "named!(";
                    s += list_data.key;
                    s += ", many0!(\n    ";
                    s = rule.ast_rule.gen_rule(s, list_data.key, self.data, false);
                    s += "\n));\n\n";
                },
                len => {
                    // Alt rule
                    s += "named!(";
                    s += list_data.key;
                    s += ", many0!(alt_complete!(\n    ";
                    for (i, rule) in list_data.rules.iter().enumerate() {
                        s = rule.ast_rule.gen_rule(s, list_data.key, self.data, true);
                        if i < len - 1 {
                            s += "\n    | ";
                        }
                    }
                    s += "\n));\n\n";
                }
            }
        }
        let mut file = File::create("gen/parsers.rs").expect("Could not open file");
        file.write_all(s.as_bytes()).expect("Could not write ast file");
    }
}