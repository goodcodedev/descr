extern crate json_descr;
use lang_data::data::*;
use lang_data::rule::*;
use std::collections::HashMap;
use self::json_descr::lang::ast::*;

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
impl<'a, 'd: 'a> CodegenSyntax<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenSyntax<'a, 'd> {
        CodegenSyntax { data }
    }

    pub fn gen(&self) {
        use self::json_descr::lang::to_source::ToSource;
        let syntax_data = self.gen_syntax_data();
        let root = self.gen_js_object(syntax_data);
        let mut s = String::new();
        println!("Js source: {}", ToSource::js_object(s, &root));
    }

    fn escape(string: &String) -> String {
        let mut s = String::with_capacity(string.len() + 10);
        for chr in string.chars() {
            match chr {
                '"' => s.push_str("\\\""),
                '\\' => s.push_str("\\\\"),
                other => s.push(other)
            }
        }
        s
    }

    pub fn gen_js_object(&self, syntax_data: SyntaxData) -> JsObject {
        let mut s = String::with_capacity(
            self.data.ast_data.len() * 80
            + self.data.list_data.len() * 80
        );
        println!("Syntax data: {:#?}", syntax_data);
        let mut root = JsObject::new(Vec::new());
        root.items.push(ObjectPair::new(
            "$schema".to_string(),
            JsVal::string_val("https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json".to_string())
        ));
        root.items.push(ObjectPair::new(
            "name".to_string(),
            JsVal::string_val(self.data.name.clone())
        ));
        let mut scope_name = String::from("source.");
        scope_name.push_str(&self.data.name);
        root.items.push(ObjectPair::new(
            "scopeName".to_string(),
            JsVal::string_val(scope_name)
        ));
        // Start part
        let start_key = self.data.start_key.expect("Could not get start key");
        // If start is an struct, include key
        // if it is an enum, include all items
        let root_patterns = match self.data.resolve(start_key) {
            ResolvedType::ResolvedStruct(key) => {
                let mut key_ref = String::with_capacity(key.len() + 1);
                key_ref.push('#');
                key_ref.push_str(key);
                vec![
                    JsVal::js_object(vec![
                        ObjectPair::new(
                            "include".to_string(),
                            JsVal::string_val(key_ref)
                        )
                    ])
                ]
            },
            ResolvedType::ResolvedEnum(key) => {
                let ast_enum = self.data.ast_enums.get(key).unwrap();
                ast_enum.items.iter().map(|item| {
                    let mut key_ref = String::with_capacity(item.len() + 1);
                    key_ref.push('#');
                    key_ref.push_str(item);
                    JsVal::js_object(vec![
                        ObjectPair::new(
                            "include".to_string(),
                            JsVal::string_val(key_ref)
                        )
                    ])
                }).collect::<Vec<_>>()
            }
        };
        root.items.push(ObjectPair::new(
            "patterns".to_string(),
            JsVal::array_val(root_patterns)
        ));
        let mut repository = JsObject::new(Vec::new());
        for (key, entry) in &syntax_data.entries {
            match entry {
                &SyntaxEntry::Match{ref regex, ref captures} => {
                    repository.items.push(
                        ObjectPair::new(
                            String::from(*key),
                            JsVal::js_object(vec![
                                ObjectPair::new(
                                    "name".to_string(),
                                    JsVal::string_val(String::from(*key))
                                ),
                                ObjectPair::new(
                                    "match".to_string(),
                                    JsVal::string_val(Self::escape(regex))
                                ),
                                ObjectPair::new(
                                    "captures".to_string(),
                                    JsVal::js_object(captures
                                        .iter()
                                        .enumerate()
                                        .map(|(i, capture_name)| {
                                            let num = i + 1;
                                            ObjectPair::new(
                                                num.to_string(),
                                                JsVal::js_object(vec![
                                                    ObjectPair::new(
                                                        "name".to_string(),
                                                        JsVal::string_val(capture_name.clone())
                                                    )
                                                ])
                                            )
                                        }).collect::<Vec<_>>()
                                    )
                                )
                            ])
                        )
                    )
                },
                &SyntaxEntry::BeginEnd{
                    ref begin,
                    ref end,
                    ref begin_captures,
                    ref end_captures,
                    ref patterns
                } => {
                    repository.items.push(
                        ObjectPair::new(
                            String::from(*key),
                            JsVal::js_object(vec![
                                ObjectPair::new(
                                    "name".to_string(),
                                    JsVal::string_val(String::from(*key))
                                ),
                                ObjectPair::new(
                                    "begin".to_string(),
                                    JsVal::string_val(Self::escape(begin))
                                ),
                                ObjectPair::new(
                                    "end".to_string(),
                                    JsVal::string_val(Self::escape(end))
                                ),
                                ObjectPair::new(
                                    "beginCaptures".to_string(),
                                    JsVal::js_object(begin_captures
                                        .iter()
                                        .enumerate()
                                        .map(|(i, capture_name)| {
                                            let num = i + 1;
                                            ObjectPair::new(
                                                num.to_string(),
                                                JsVal::js_object(vec![
                                                    ObjectPair::new(
                                                        "name".to_string(),
                                                        JsVal::string_val(capture_name.clone())
                                                    )
                                                ])
                                            )
                                        }).collect::<Vec<_>>()
                                    )
                                ),
                                ObjectPair::new(
                                    "endCaptures".to_string(),
                                    JsVal::js_object(begin_captures
                                        .iter()
                                        .enumerate()
                                        .map(|(i, capture_name)| {
                                            let num = i + 1;
                                            ObjectPair::new(
                                                num.to_string(),
                                                JsVal::js_object(vec![
                                                    ObjectPair::new(
                                                        "name".to_string(),
                                                        JsVal::string_val(capture_name.clone())
                                                    )
                                                ])
                                            )
                                        }).collect::<Vec<_>>()
                                    )
                                ),
                                ObjectPair::new(
                                    "patterns".to_string(),
                                    JsVal::array_val(patterns
                                        .iter()
                                        .map(|pattern| {
                                            let mut key_ref = String::from("#");
                                            key_ref.push_str(&pattern);
                                            JsVal::js_object(vec![
                                                ObjectPair::new(
                                                    "include".to_string(),
                                                    JsVal::string_val(key_ref)
                                                )
                                            ])
                                        }).collect::<Vec<_>>()
                                    )
                                )
                            ])
                        )
                    )
                }
            }
        }
        root.items.push(ObjectPair::new(
            "repository".to_string(),
            repository.as_js_val()
        ));
        root
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