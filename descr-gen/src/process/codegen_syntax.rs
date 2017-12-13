extern crate json_descr;
use lang_data::data::*;
use lang_data::rule::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use self::json_descr::lang::ast::*;
use itertools::Itertools;

#[derive(Debug)]
pub struct SyntaxData {
    pub entries: HashMap<String, SyntaxEntry>,
    pub root_entries: Vec<String>,
    // Parents named by key, will have
    // Vec<String> items merged at it's
    // level in includes
    pub parent_entries: HashMap<String, Vec<String>>
}
impl SyntaxData {
    pub fn add_parent_entry<S: Into<String>>(&mut self, parent: S, entry: S) {
        let parent = parent.into();
        if !self.parent_entries.contains_key(&parent) {
            self.parent_entries.insert(parent.clone(), Vec::new());
        }
        self.parent_entries
            .get_mut(&parent)
            .unwrap()
            .push(entry.into());
    }

    pub fn expand_pattern_list(&self, list: &Vec<String>) -> Vec<String> {
        list
            .iter()
            .flat_map(|item| {
                self.get_parent_entries(item.clone(), Vec::new())
            })
            .chain(list.iter().map(|item| { item.clone() }))
            //.chain(items.iter().map(|i| { String::from(*i) }).collect::<Vec<_>>())
            .unique()
            .collect::<Vec<_>>()
    }

    // Recursively get merged parent entries
    pub fn get_parent_entries<S: Into<String>>(&self, parent: S, mut v: Vec<String>) -> Vec<String> {
        let parent = parent.into();
        match self.parent_entries.get(&parent) {
            None => {},
            Some(entries) => {
                for entry in entries {
                    v.push(entry.clone());
                    v = self.get_parent_entries(entry.clone(), v);
                }
            }
        }
        v
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
    
    fn escape(string: String) -> String {
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
            .flat_map(|item| {
                syntax_data.get_parent_entries(item.to_string(), Vec::new())
            })
            .chain(items.iter().map(|e| { e.to_string() }))
            //.chain(items.iter().map(|i| { String::from(*i) }).collect::<Vec<_>>())
            .unique()
            .map(|item| {
                let mut key_ref = String::with_capacity(item.len() + 1);
                key_ref.push('#');
                key_ref.push_str(&item);
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
                            JsVal::string_val(SyntaxEntry::escape(collect.get_regex()))
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
                            JsVal::string_val(SyntaxEntry::escape(begin.get_regex()))
                        ),
                        ObjectPair::new(
                            "end".to_string(),
                            JsVal::string_val(SyntaxEntry::escape(end.get_end_regex(syntax_data, begin)))
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
                    &AstRule::RefRule(ref_key) => {
                        // Add as parent to base type if different
                        if ref_key != ast_data.ast_type {
                            syntax_data.add_parent_entry(ast_data.ast_type, ref_key);
                        }
                    },
                    &AstRule::PartsRule(ref parts_rule) => {
                        // Add as parent to base type if different
                        if parts_rule.ast_type != ast_data.ast_type {
                            syntax_data.add_parent_entry(ast_data.ast_type, parts_rule.ast_type);
                        }
                        parts_rule.add_syntax_entries(&mut syntax_data, self.data);
                    }
                }
            }
        }
        for (key, list_data) in &self.data.list_data {
            for rule in &list_data.rules {
                match &rule.ast_rule {
                    &AstRule::RefRule(ref_key) => {
                        // Add as parent to list type if specified
                        if let Some(ast_key) = list_data.ast_type {
                            if ref_key != ast_key {
                                syntax_data.add_parent_entry(ast_key, ref_key);
                            }
                        }
                    },
                    &AstRule::PartsRule(ref parts_rule) => {
                        // Add as parent to list type if specified
                        if let Some(ast_key) = list_data.ast_type {
                            if parts_rule.ast_type != ast_key {
                                syntax_data.add_parent_entry(ast_key, parts_rule.ast_type);
                            }
                        }
                        parts_rule.add_syntax_entries(&mut syntax_data, self.data);
                    }
                }
            }
        }
        // Expand patterns that only have
        // optional parts to include it's
        // patterns matches
        {
            let replacements = Self::get_replacements(&syntax_data);
            println!("{:#?}", replacements);
            // Replace parent_refs
            syntax_data.parent_entries = syntax_data.parent_entries
                .into_iter()
                .map(|(key, parent_entries)| {
                    (
                        key,
                        parent_entries
                            .into_iter()
                            .flat_map(|entry| {
                                match replacements.get(&entry) {
                                    None => vec!(entry),
                                    Some(rtuples) => {
                                        rtuples
                                            .iter()
                                            .map(|&(ref new_key, ref _new_entry)| {
                                                new_key.clone()
                                            })
                                            .collect::<Vec<String>>()
                                    }
                                }
                            })
                            .collect()
                    )
                })
                .collect();
            // Replace patterns
            // Only replacing begins, don't think end's
            // will have patterns.
            for (_key, entry) in syntax_data.entries.iter_mut() {
                match entry {
                    &mut SyntaxEntry::Match{..} => {},
                    &mut SyntaxEntry::BeginEnd{ref mut begin, ref mut end} => {
                        begin.patterns = begin.patterns
                            .iter()
                            .flat_map(|p| {
                                match replacements.get(p) {
                                    None => vec!(p.clone()),
                                    Some(rtuples) => {
                                        rtuples
                                            .iter()
                                            .map(|&(ref new_key, ref _new_entry)| {
                                                new_key.clone()
                                            })
                                            .collect::<Vec<String>>()
                                    }
                                }
                            })
                            .collect::<Vec<String>>();
                    }
                }
            }
            // Replace entries
            replacements
                .into_iter()
                .for_each(|(key, rtuples)| {
                    syntax_data.entries.remove(&key);
                    rtuples
                        .into_iter()
                        .for_each(|(new_key, new_entry)| {
                            syntax_data.entries.insert(new_key, new_entry);
                        });
                });
            /*
            syntax_data.entries = syntax_data.entries
                .into_iter()
                .fold(HashMap::new(), |mut entries, (key, entry)| {
                    match replacements.get(&key) {
                        None => {
                            // Keep
                            entries.insert(key, entry);
                            entries
                        },
                        Some(rtuples) => {
                            // Replace
                            for &(ref new_key, ref new_entry) in rtuples {
                                entries.insert(new_key.clone(), new_entry);
                            }
                            entries
                        }
                    }
                });
            */
        }
        syntax_data
    }

    pub fn get_replacements(syntax_data: &SyntaxData) -> HashMap<String, Vec<(String, SyntaxEntry)>> {
        syntax_data.entries
            .iter()
            .fold(HashMap::new(), |mut replacements, (key, entry)| {
                match entry {
                    &SyntaxEntry::Match{ref collect} => {
                        // We have no subpatterns to combine
                        // with, so can't expand.
                        // Possibly mark only_optional
                        // as invalid
                        replacements
                    },
                    &SyntaxEntry::BeginEnd{ref begin, ref end} => {
                        if begin.only_optional {
                            // Expand this match to include each of
                            // it's patterns.
                            // This turns out a little weird for
                            // ex Source(items) items:Item[] { Item1, Item2 } 
                            let rtuples = match replacements.entry(key.clone()) {
                                Vacant(p) => p.insert(Vec::with_capacity(begin.patterns.len())),
                                Occupied(p) => p.into_mut()
                            };
                            for pattern in &begin.patterns {
                                let expanded = begin.clone();
                                let sub_entry = syntax_data.entries.get(&**pattern).unwrap();
                                let mut new_key = key.clone();
                                new_key.push('_');
                                new_key.push_str(&pattern);
                                let new_entry = match sub_entry {
                                    &SyntaxEntry::Match{ref collect} => {
                                        // Combine start + match + end into new match
                                        let mut new_collect = begin.clone();
                                        new_collect.patterns = Vec::new();
                                        new_collect.append(collect);
                                        new_collect.append(end);
                                        SyntaxEntry::Match {
                                            collect: new_collect
                                        }
                                    },
                                    &SyntaxEntry::BeginEnd{
                                        begin: ref inner_begin,
                                        end: ref inner_end
                                    } => {
                                        // Combine start + beginEnd.start (patterns) beginEnd.end + end
                                        let mut new_begin = begin.clone();
                                        new_begin.patterns = Vec::new();
                                        new_begin.append(inner_begin);
                                        let mut new_end = inner_end.clone();
                                        new_end.append(end);
                                        SyntaxEntry::BeginEnd {
                                            begin: new_begin,
                                            end: new_end
                                        }
                                    }
                                };
                                // Push tuple
                                rtuples.push((new_key, new_entry));
                            }
                        }
                        replacements
                    }
                }
            })
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