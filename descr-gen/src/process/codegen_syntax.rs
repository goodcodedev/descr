use lang_data::data::*;
use lang_data::rule::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct SyntaxData<'a> {
    pub entries: HashMap<&'a str, SyntaxEntry>,
    pub root_entries: Vec<String>,
    pub parent_entries: HashMap<&'a str, Vec<&'a str>>
}
impl<'a> SyntaxData<'a> {
    pub fn add_parent_entry(&mut self, parent: &'a str, entry: &'a str) {
        if !self.parent_entries.contains_key(parent) {
            self.parent_entries.insert(parent, Vec::new());
        }
        self.parent_entries
            .get_mut(parent)
            .unwrap()
            .push(entry);
    }
}

#[derive(Debug)]
pub enum SyntaxEntry {
    Match {
        regex: String,
        captures: Vec<String>
    },
    BeginEnd {
        begin: String,
        end: String,
        begin_captures: Vec<String>,
        end_captures: Vec<String>,
        patterns: Vec<String>
    }
}

pub struct CodegenSyntax<'a, 'd: 'a> {
    data: &'a LangData<'d>
}
impl<'a, 'd> CodegenSyntax<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenSyntax<'a, 'd> {
        CodegenSyntax { data }
    }

    pub fn gen(&self) {
        let syntax_data = self.gen_syntax_data();
        let start_key = self.data.start_key.expect("Could not get start key");
        let start_part = self.data.typed_parts.get(start_key).expect("Could not get start part");
        let mut s = String::with_capacity(
            self.data.ast_data.len() * 80
            + self.data.list_data.len() * 80
        );
        println!("Syntax data: {:#?}", syntax_data);

    }
    
    pub fn gen_syntax_data(&self) -> SyntaxData {
        let mut syntax_data = SyntaxData {
            entries: HashMap::new(),
            root_entries: Vec::new(),
            parent_entries: HashMap::new()
        };
        for (key, ast_data) in &self.data.ast_data {
            for rule in &ast_data.rules {
                match rule {
                    &AstRule::RefRule(ref_key) => {},
                    &AstRule::PartsRule(ref parts_rule) => {
                        parts_rule.add_syntax_entries(&mut syntax_data, self.data);
                    }
                }
            }
        }
        for (key, list_data) in &self.data.list_data {
            for rule in &list_data.rules {
                match &rule.ast_rule {
                    &AstRule::RefRule(ref_key) => {},
                    &AstRule::PartsRule(ref parts_rule) => {
                        parts_rule.add_syntax_entries(&mut syntax_data, self.data);
                    }
                }
            }
        }
        syntax_data
    }
}