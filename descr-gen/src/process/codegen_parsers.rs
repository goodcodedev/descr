use lang_data::data::*;
use descr_common::util::*;

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
        s += "use descr_common::parsers::*;\n";
        s += "extern crate nom;\n";
        s += "use self::nom::*;\n";
        s += "use std;\n";
        s += "use super::ast::*;\n\n";
        // Start key
        match self.data.start_key {
            Some(start_key) => {
                match self.data.rule_types.get(start_key) {
                    Some(ref rule_type) => {
                        append!(s, "named!(pub start<" rule_type.get_type_name(self.data) ">, "
                                "do_parse!(res: " self.data.sc(start_key) " >> (res)));\n\n");
                    },
                    _ => {}
                }
            },
            None => {}
        }
        // Ast data
        for (key, ast_data) in self.data.ast_data.sorted_iter() {
            let rule_type = self.data.rule_types.get(key).unwrap();
            match ast_data.rules.len() {
                0 => {},
                1 => {
                    let rule = ast_data.rules.first().unwrap();
                    append!(s, "named!(pub " self.data.sc(ast_data.ast_type) "<" ast_data.ast_type ">,\n    ");
                    s = rule.gen_rule(s, self.data, rule_type);
                    s += "\n);\n\n";
                },
                len => {
                    // Alt rule
                    append!(s, "named!(pub " self.data.sc(ast_data.ast_type) "<" ast_data.ast_type ">, alt_complete!(\n    ");
                    for (i, rule) in ast_data.rules.iter().enumerate() {
                        s = rule.gen_rule(s, self.data, rule_type);
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
            let rule_type = self.data.rule_types.get(key).unwrap();
            match list_data.rules.len() {
                0 => {},
                1 => {
                    let rule = list_data.rules.first().unwrap();
                    append!(s,
                        "named!(pub "
                        self.data.sc(list_data.key)
                        "<Vec<"
                        self.data.rule_types.get(list_data.key).unwrap().get_type_name(self.data)
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
                    s = rule.ast_rule.gen_rule(s, self.data, rule_type);
                    s += "\n));\n\n";
                },
                len => {
                    // Alt rule
                    append!(s, "named!(pub "
                        self.data.sc(list_data.key)
                        "<Vec<"
                        self.data.rule_types.get(list_data.key).unwrap().get_type_name(self.data)
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
                        s = rule.ast_rule.gen_rule(s, self.data, rule_type);
                        if i < len - 1 { s += "\n    | "; }
                    }
                    s += "\n)));\n\n";
                }
            }
        }
        s
    }
}