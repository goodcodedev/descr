extern crate json_descr;
use lang_data::data::*;
use lang_data::rule::*;
use std::collections::HashMap;
use self::json_descr::lang::ast::*;
use itertools::Itertools;

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
        collect: CollectState
    },
    BeginEnd {
        begin: CollectState,
        end: CollectState
    }
}
impl SyntaxEntry {
    
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

    // Chain in parent entries and create
    // vec of JsObjects with includes
    fn collect_pattern_includes(items: &Vec<&str>, syntax_data: &SyntaxData) -> JsVal {
        JsVal::array_val(items
            .iter()
            .map(|item| {
                syntax_data.parent_entries.get(item)
            })
            .filter(|e| { e.is_some() })
            .flat_map(|e| { e.unwrap() })
            .chain(items.iter())
            .unique()
            .map(|item| {
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
        )
    }

    fn collect_captures(captures: &Vec<String>) -> JsVal {
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
    }

    pub fn collect_repository_item(&self, key: &str, syntax_data: &SyntaxData) -> ObjectPair {
        match self {
            &SyntaxEntry::Match{ref collect} => {
                ObjectPair::new(
                    String::from(key),
                    JsVal::js_object(vec![
                        ObjectPair::new(
                            "name".to_string(),
                            JsVal::string_val(String::from(key))
                        ),
                        ObjectPair::new(
                            "match".to_string(),
                            JsVal::string_val(SyntaxEntry::escape(&collect.regex))
                        ),
                        ObjectPair::new(
                            "captures".to_string(),
                            SyntaxEntry::collect_captures(&collect.captures)
                        )
                    ])
                )
            },
            &SyntaxEntry::BeginEnd{ref begin, ref end} => {
                ObjectPair::new(
                    String::from(key),
                    JsVal::js_object(vec![
                        ObjectPair::new(
                            "name".to_string(),
                            JsVal::string_val(String::from(key))
                        ),
                        ObjectPair::new(
                            "begin".to_string(),
                            JsVal::string_val(SyntaxEntry::escape(&begin.regex))
                        ),
                        ObjectPair::new(
                            "end".to_string(),
                            JsVal::string_val(SyntaxEntry::escape(&end.regex))
                        ),
                        ObjectPair::new(
                            "beginCaptures".to_string(),
                            SyntaxEntry::collect_captures(&begin.captures)
                        ),
                        ObjectPair::new(
                            "endCaptures".to_string(),
                            SyntaxEntry::collect_captures(&end.captures)
                        ),
                        ObjectPair::new(
                            "patterns".to_string(),
                            SyntaxEntry::collect_pattern_includes(
                                &begin.patterns.iter().map(|p| { &**p }).collect(), 
                                &syntax_data
                            )
                        )
                    ])
                )
            }
        }
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
        // Expand patterns that only have
        // optional parts to include it's
        // patterns matches
        for (key, entry) in &syntax_data.entries {
            match entry {
                &SyntaxEntry::Match{ref collect} => {
                    // We have no subpatterns to combine
                    // with, so can't expand.
                    // Possibly mark only_optional
                    // as invalid
                },
                &SyntaxEntry::BeginEnd{ref begin, ref end} => {
                    if begin.only_optional {
                        for pattern in &begin.patterns {
                            let expanded = begin.clone();
                            let sub_entry = syntax_data.entries.get(&**pattern).unwrap();
                            match sub_entry {
                                &SyntaxEntry::Match{ref collect} => {
                                    // Combine start + match + end into new match
                                    let mut new_collect = begin.clone();
                                    new_collect.append(collect);
                                    new_collect.append(end);
                                    let entry = SyntaxEntry::Match {
                                        collect: new_collect
                                    };
                                },
                                &SyntaxEntry::BeginEnd{
                                    begin: ref inner_begin,
                                    end: ref inner_end
                                } => {
                                    // Combine start + beginEnd.start (patterns) beginEnd.end + end
                                    let mut new_begin = begin.clone();
                                    new_begin.append(inner_begin);
                                    let mut new_end = inner_end.clone();
                                    new_end.append(end);
                                    let entry = SyntaxEntry::BeginEnd {
                                        begin: new_begin,
                                        end: new_end
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }
        syntax_data
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
                SyntaxEntry::collect_pattern_includes(&vec![key], &syntax_data)
            },
            ResolvedType::ResolvedEnum(key) => {
                let ast_enum = self.data.ast_enums.get(key).unwrap();
                SyntaxEntry::collect_pattern_includes(&ast_enum.items, &syntax_data)
            }
        };
        root.items.push(ObjectPair::new(
            "patterns".to_string(),
            root_patterns
        ));
        let mut repository = JsObject::new(Vec::new());
        for (key, entry) in &syntax_data.entries {
            repository.items.push(entry.collect_repository_item(key, &syntax_data))
        }
        root.items.push(ObjectPair::new(
            "repository".to_string(),
            repository.as_js_val()
        ));
        root
    }
    
}