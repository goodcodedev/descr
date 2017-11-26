use lang_data::*;
use std::fs::File;
use std::io::Write;

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
        for (key, ast_data) in &self.data.ast_data {
            match ast_data.rules.len() {
                0 => {},
                1 => {
                    let rule = ast_data.rules.first().unwrap();
                    s += "named!(";
                    s += ast_data.ast_type;
                    s += "<";
                    s += ast_data.ast_type;
                    s += ">,\n";
                    rule.gen_rule(&mut s, self.data);
                    s += ");\n";
                },
                len => {
                    // Alt rule
                    s += "named!(";
                    s += ast_data.ast_type;
                    s += "<";
                    s += ast_data.ast_type;
                    s += ">, alt_complete!(\n";
                    for (i, rule) in ast_data.rules.iter().enumerate() {
                        rule.gen_rule(&mut s, self.data);
                        if i < len {
                            s += "\n| ";
                        }
                    }
                    s += "));\n";
                }
            }
        }
        let mut file = File::create("gen/parsers.rs").expect("Could not open file");
        file.write_all(s.as_bytes()).expect("Could not write ast file");
    }
}