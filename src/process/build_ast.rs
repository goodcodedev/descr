use lang_data::data::*;
use lang_data::ast::*;
use lang_data::typed_part::*;
use lang_data::rule::*;
use std::collections::HashMap;

pub struct BuildAst<'a, 'd: 'a> {
    data: &'a mut LangData<'d>
}
impl<'a, 'd: 'a> BuildAst<'a, 'd> {
    pub fn new(data: &'a mut LangData<'d>) -> BuildAst<'a, 'd> {
        BuildAst {
            data
        }
    }

    fn reg_struct(data: &mut HashMap<&'d str, AstStruct<'d>>, name: &'d str, snake_cased: &mut SnakeCased<'d>) {
        if !data.contains_key(name) {
            data.insert(
                name,
                AstStruct::new(name, snake_cased.get(name))
            );
        } else {
            // Increment counter
            let ast_struct = data.get_mut(name).unwrap();
            ast_struct.num_patterns += 1;
        }
    }

    fn reg_struct_member(data: &mut HashMap<&'d str, AstStruct<'d>>, 
                             struct_name: &'d str, member_name: &'d str,
                             part_key: &'d str, optional: bool,
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
                AstStructMember::new(member_name, snake_cased.get(member_name), part_key, struct_name, optional)
            );
        }
    }

    fn process_parts_rule(rule: &AstPartsRule<'d>,
                          struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
                          typed_parts: &HashMap<&'d str, TypedPart<'d>>,
                          snake_cased: &mut SnakeCased<'d>) {
        for part in &rule.parts {
            let typed_part = typed_parts.get(part.part_key).unwrap();
            use lang_data::typed_part::TypedPart::*;
            match typed_part {
                &AstPart { .. }
                | &ListPart { .. }
                | &IntPart { .. }
                | &IdentPart { .. } => {
                    let member_key = part.member_key.unwrap_or(part.part_key);
                    Self::reg_struct_member(
                        struct_data,
                        rule.ast_type, 
                        member_key,
                        part.part_key,
                        part.optional,
                        snake_cased);
                },
                &CharPart { .. }
                | &TagPart { .. } => {
                    // Count as member if member name is given
                    if let Some(member_key) = part.member_key {
                        Self::reg_struct_member(
                            struct_data,
                            rule.ast_type, 
                            member_key,
                            part.part_key,
                            part.optional,
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
            let mut is_enum = false;
            for rule in &ast_data.rules {
                match rule {
                    &AstRule::RefRule(key_ref) => {
                        // Ref to another ast
                        if !is_enum && key_ref != ast_data.ast_type {
                            is_enum = true;
                        }
                    },
                    &AstRule::PartsRule(ref rule) => {
                        if rule.ast_type != ast_data.ast_type {
                            is_enum = true;
                        }
                        Self::reg_struct(struct_data, rule.ast_type, snake_cased);
                        Self::process_parts_rule(rule, struct_data, typed_parts, snake_cased);
                    }
                }
            }
            if is_enum {
                let mut e = AstEnum::new(ast_data.ast_type, snake_cased.get(ast_data.ast_type));
                for rule in &ast_data.rules {
                    match rule {
                        &AstRule::RefRule(key_ref) => {
                            e.items.push(key_ref);
                        },
                        &AstRule::PartsRule(ref rule) => {
                            e.items.push(rule.ast_type);
                        }
                    }
                }
                enum_data.insert(ast_data.ast_type, e);
                type_refs.insert(ast_data.ast_type, AstType::AstEnum(ast_data.ast_type));
            } else {
                type_refs.insert(ast_data.ast_type, AstType::AstStruct(ast_data.ast_type));
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
            let mut last_type = None;
            let mut is_enum = false;
            for rule in &list_data.rules {
                match &rule.ast_rule {
                    &AstRule::RefRule(key_ref) => {
                        // Ref to another ast
                        if !is_enum && last_type.is_some() && last_type.unwrap() != key_ref {
                            is_enum = true;
                        }
                        last_type = Some(key_ref);
                    },
                    &AstRule::PartsRule(ref rule) => {
                        if !is_enum && last_type.is_some() && last_type.unwrap() != rule.ast_type {
                            is_enum = true;
                        }
                        last_type = Some(rule.ast_type);
                        Self::reg_struct(struct_data, rule.ast_type, snake_cased);
                        Self::process_parts_rule(rule, struct_data, typed_parts, snake_cased);
                    }
                }
                if is_enum {
                    let mut e = AstEnum::new(list_data.key, snake_cased.get(list_data.key));
                    for rule in &list_data.rules {
                        match &rule.ast_rule {
                            &AstRule::RefRule(key_ref) => {
                                e.items.push(key_ref);
                            },
                            &AstRule::PartsRule(ref rule) => {
                                e.items.push(rule.ast_type);
                            }
                        }
                    }
                    enum_data.insert(list_data.key, e);
                    type_refs.insert(list_data.key, AstType::AstEnum(last_type.unwrap()));
                } else {
                    type_refs.insert(list_data.key, AstType::AstStruct(last_type.unwrap()));
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