use lang_data::data::*;
use std::fs::File;
use std::io::Write;
use util::*;

pub struct CodegenParsers<'a, 'd: 'a> {
    data: &'a LangData<'d>
}
impl<'a, 'd> CodegenParsers<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenParsers<'a, 'd> {
        CodegenParsers { data }
    }

    pub fn gen(&self) -> String {
        let mut s = String::with_capacity(
            self.data.ast_data.len() * 100
            + self.data.list_data.len() * 100
        );
        s += "extern crate descr_common;\n";
        s += "use self::descr_common::parsers::*;\n";
        s += "extern crate nom;\n";
        s += "use self::nom::*;\n";
        s += "use super::ast::*;\n\n";
        // Ast data
        for (key, ast_data) in self.data.ast_data.sorted_iter() {
            match ast_data.rules.len() {
                0 => {},
                1 => {
                    let rule = ast_data.rules.first().unwrap();
                    append!(s, "named!(pub " self.data.sc(ast_data.ast_type) "<" ast_data.ast_type ">,\n    ");
                    s = rule.gen_rule(s, ast_data.ast_type, self.data, false);
                    s += "\n);\n\n";
                },
                len => {
                    // Alt rule
                    append!(s, "named!(pub " self.data.sc(ast_data.ast_type) "<" ast_data.ast_type ">, alt_complete!(\n    ");
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
                    append!(s,
                        "named!(pub "
                        self.data.sc(list_data.key)
                        "<Vec<"
                        self.data.type_refs.get(list_data.key).unwrap().get_type_name()
                        ">>, ");
                    match list_data.sep {
                        Some(sep) => {
                            append!(s, "separated_list!(");
                            s = self.data.typed_parts.get(sep).unwrap().gen_parser(s, self.data);
                            s += ", \n    ";
                        },
                        None => {
                            append!(s, "many0!(\n    ");
                        }
                    }
                    s = rule.ast_rule.gen_rule(s, list_data.key, self.data, false);
                    s += "\n));\n\n";
                },
                len => {
                    // Alt rule
                    append!(s, "named!(pub "
                        self.data.sc(list_data.key)
                        "<Vec<"
                        self.data.type_refs.get(list_data.key).unwrap().get_type_name()
                        ">>, ");
                    match list_data.sep {
                        Some(sep) => {
                            append!(s, "separated_list!(");
                            s = self.data.typed_parts.get(sep).unwrap().gen_parser(s, self.data);
                            s += ", ";
                        },
                        None => {
                            append!(s, "many0!(");
                        }
                    }
                    append!(s, "alt_complete!(\n    ");
                    for (i, rule) in list_data.rules.iter().enumerate() {
                        s = rule.ast_rule.gen_rule(s, list_data.key, self.data, true);
                        if i < len - 1 { s += "\n    | "; }
                    }
                    s += "\n));\n\n";
                }
            }
        }
        s
    }
}