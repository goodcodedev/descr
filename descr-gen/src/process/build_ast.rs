use lang_data::data::*;
use lang_data::ast::*;
use lang_data::typed_part::*;
use lang_data::rule::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct BuildAst<'a, 'd: 'a> {
    data: &'a mut LangData<'d>
}
impl<'a, 'd: 'a> BuildAst<'a, 'd> {
    pub fn new(data: &'a mut LangData<'d>) -> BuildAst<'a, 'd> {
        BuildAst {
            data
        }
    }

    fn reg_struct(data: &mut HashMap<&'d str, AstStruct<'d>>, name: &'d str, key: &'d str,
                  snake_cased: &mut SnakeCased<'d>, type_refs: &mut HashMap<&'d str, AstType<'d>>) {
        if !data.contains_key(name) {
            data.insert(
                name,
                AstStruct::new(name, snake_cased.get(name))
            );
        } else {
            // Increment counter to match against arg counter
            let ast_struct = data.get_mut(name).unwrap();
            ast_struct.num_patterns += 1;
        }
        if !type_refs.contains_key(key) {
            type_refs.insert(key, AstType::AstStruct(name));
        }
    }

    fn reg_struct_member(data: &mut HashMap<&'d str, AstStruct<'d>>, 
                             struct_name: &'d str, member_name: &'d str,
                             part_key: &'d str, optional: bool, not: bool,
                             snake_cased: &mut SnakeCased<'d>) {
        let ast_struct = data.get_mut(struct_name).unwrap();
        if ast_struct.members.contains_key(member_name) {
            let struct_member = ast_struct.members.get_mut(member_name).unwrap();
            struct_member.num_patterns += 1;
            if optional {
                struct_member.optional = true;
            }
        } else {
            ast_struct.members.insert(
                member_name,
                AstStructMember::new(
                    member_name, 
                    snake_cased.get(member_name), 
                    part_key, 
                    struct_name, 
                    optional,
                    not)
            );
        }
    }

    fn process_parts_rule(rule: &AstPartsRule<'d>,
                          struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
                          typed_parts: &HashMap<&'d str, TypedPart<'d>>,
                          snake_cased: &mut SnakeCased<'d>) {
        for part in &rule.parts {
            match &part.token {
                &AstRuleToken::Key(key) => {
                    let typed_part = typed_parts.get(key).unwrap();
                    use lang_data::typed_part::TypedPart::*;
                    match typed_part {
                        &AstPart { .. }
                        | &ListPart { .. }
                        | &IntPart { .. }
                        | &StringPart { .. }
                        | &IdentPart { .. } => {
                            // Count as member by default
                            let member_key = part.member_key.unwrap_or(key);
                            Self::reg_struct_member(
                                struct_data,
                                rule.ast_type, 
                                member_key,
                                key,
                                part.optional,
                                part.not,
                                snake_cased);
                        },
                        &CharPart { .. }
                        | &FnPart { .. }
                        | &TagPart { .. } => {
                            // Count as member if member name is given
                            if let Some(member_key) = part.member_key {
                                Self::reg_struct_member(
                                    struct_data,
                                    rule.ast_type, 
                                    member_key,
                                    key,
                                    part.optional,
                                    part.not,
                                    snake_cased);
                            }
                        }
                    }
                },
                &AstRuleToken::Tag(string) => {
                    // Count as member if member name is given
                    if let Some(member_key) = part.member_key {
                        Self::reg_struct_member(
                            struct_data,
                            rule.ast_type, 
                            member_key,
                            string,
                            part.optional,
                            part.not,
                            snake_cased);
                    }
                }
            }
        }
    }

    fn build_from_ast_data(ast_data: &HashMap<&'d str, AstData<'d>>,
                           struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
                           enum_data: &mut HashMap<&'d str, AstEnum<'d>>,
                           typed_parts: &HashMap<&'d str, TypedPart<'d>>,
                           type_refs: &mut HashMap<&'d str, AstType<'d>>,
                           snake_cased: &mut SnakeCased<'d>) {
        for (_key, ast_data) in ast_data {
            // Collect types to check if
            // this should be an enum
            let mut types = HashSet::new();
            types.insert(ast_data.ast_type);
            for rule in &ast_data.rules {
                match rule {
                    &AstRule::RefRule(key_ref) => {
                        // Ref to another ast
                        types.insert(key_ref);
                    },
                    &AstRule::PartsRule(ref rule) => {
                        types.insert(rule.ast_type);
                        Self::reg_struct(struct_data, rule.ast_type, rule.ast_type, snake_cased, type_refs);
                        Self::process_parts_rule(rule, struct_data, typed_parts, snake_cased);
                    }
                }
            }
            match types.len() {
                0 => {},
                1 => {
                    //type_refs.insert(ast_data.ast_type, AstType::AstStruct(ast_data.ast_type));
                },
                _ => {
                    // Several types registered, create enum
                    let mut e = AstEnum::new(ast_data.ast_type, snake_cased.get(ast_data.ast_type));
                    let mut added = HashSet::new();
                    for rule in &ast_data.rules {
                        let item = match rule {
                            &AstRule::RefRule(key_ref) => {
                                key_ref
                            },
                            &AstRule::PartsRule(ref rule) => {
                                rule.ast_type
                            }
                        };
                        if !added.contains(item) {
                            e.items.push(item);
                            added.insert(item);
                        }
                    }
                    enum_data.insert(ast_data.ast_type, e);
                    type_refs.insert(ast_data.ast_type, AstType::AstEnum(ast_data.ast_type));
                }
            }
        }
    }

    fn build_from_list_data(list_data: &HashMap<&'d str, ListData<'d>>,
                            struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
                            enum_data: &mut HashMap<&'d str, AstEnum<'d>>,
                            typed_parts: &HashMap<&'d str, TypedPart<'d>>,
                            type_refs: &mut HashMap<&'d str, AstType<'d>>,
                            snake_cased: &mut SnakeCased<'d>) {
        for (key, list_data) in list_data {
            snake_cased.reg(key);
            let mut types = HashSet::new();
            list_data.ast_type.map(|t| { types.insert(t); });
            for rule in &list_data.rules {
                match &rule.ast_rule {
                    &AstRule::RefRule(key_ref) => {
                        // Ref to another ast
                        types.insert(key_ref);
                    },
                    &AstRule::PartsRule(ref rule) => {
                        types.insert(rule.ast_type);
                        Self::reg_struct(struct_data, rule.ast_type, rule.ast_type, snake_cased, type_refs);
                        Self::process_parts_rule(rule, struct_data, typed_parts, snake_cased);
                    }
                }
            }
            match types.len() {
                0 => {},
                1 => {
                    let ast_name = match list_data.ast_type {
                        Some(t) => t,
                        None => match &list_data.rules[0].ast_rule {
                            &AstRule::RefRule(r) => r,
                            &AstRule::PartsRule(ref rule) => rule.ast_type
                        }
                    };
                    type_refs.insert(list_data.key, AstType::AstStruct(ast_name));
                },
                _ => {
                    let enum_name = match list_data.ast_type {
                        Some(ast_type) => ast_type,
                        None => list_data.key
                    };
                    let mut e = AstEnum::new(enum_name, snake_cased.get(enum_name));
                    let mut added = HashSet::new();
                    for rule in &list_data.rules {
                        let item = match &rule.ast_rule {
                            &AstRule::RefRule(key_ref) => {
                                key_ref
                            },
                            &AstRule::PartsRule(ref rule) => {
                                rule.ast_type
                            }
                        };
                        if !added.contains(item) {
                            e.items.push(item);
                            added.insert(item);
                        }
                    }
                    enum_data.insert(list_data.key, e);
                    type_refs.insert(list_data.key, AstType::AstEnum(enum_name));
                }
            }
        }
    }

    pub fn build_ast(&mut self) {
        Self::build_from_ast_data(
            &self.data.ast_data, 
            &mut self.data.ast_structs,
            &mut self.data.ast_enums,
            &self.data.typed_parts,
            &mut self.data.type_refs,
            &mut self.data.snake_cased);
        Self::build_from_list_data(
            &self.data.list_data, 
            &mut self.data.ast_structs,
            &mut self.data.ast_enums,
            &self.data.typed_parts,
            &mut self.data.type_refs,
            &mut self.data.snake_cased);
    }
}