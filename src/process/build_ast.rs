use lang_data::*;
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

    fn reg_struct(data: &mut HashMap<&'d str, AstStruct<'d>>, name: &'d str) {
        if !data.contains_key(name) {
            data.insert(
                name,
                AstStruct::new()
            );
        } else {
            // Increment counter
            let ast_struct = data.get_mut(name).unwrap();
            ast_struct.num_patterns += 1;
        }
    }

    fn reg_struct_member(data: &mut HashMap<&'d str, AstStruct<'d>>, 
                             struct_name: &'d str, member_name: &'d str,
                             token_key: &'d str, optional: bool) {
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
                AstStructMember::new(member_name, token_key, optional)
            );
        }
    }

    fn build_from_ast_data(ast_data: &HashMap<&'d str, AstData<'d>>,
                           struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
                           enum_data: &mut HashMap<&'d str, AstEnum<'d>>) {
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
                        Self::reg_struct(struct_data, rule.ast_type);
                        for part_key in &rule.part_keys {
                            Self::reg_struct_member(
                                struct_data,
                                rule.ast_type, 
                                part_key,
                                part_key,
                                false);
                        }
                    }
                }
            }
            if is_enum {
                let mut e = AstEnum::new();
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
            }
        }
    }

    fn build_from_list_data(list_data: &HashMap<&'d str, ListData<'d>>,
                            struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
                            enum_data: &mut HashMap<&'d str, AstEnum<'d>>) {
        for (_key, list_data) in list_data {
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
                        Self::reg_struct(struct_data, rule.ast_type);
                        for part_key in &rule.part_keys {
                            Self::reg_struct_member(
                                struct_data,
                                rule.ast_type, 
                                part_key,
                                part_key,
                                false);
                        }
                    }
                }
                if is_enum {
                    let mut e = AstEnum::new();
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
                }
            }
        }
    }

    fn build_enums_from_ast_data(ast_data: &HashMap<&'d str, AstData<'d>>,
                                 data: &mut HashMap<&'d str, AstEnum<'d>>) {
        for (_key, ast_data) in ast_data {
            match ast_data.rules.len() {
                0 => (),
                1 => {
                    let rule = &ast_data.rules[0];

                },
                len => {

                }
            }
            for rule in &ast_data.rules {

            }
        }
    }

    fn build_enums_from_list_data(list_data: &HashMap<&'d str, ListData<'d>>,
                                  data: &mut HashMap<&'d str, AstEnum<'d>>) {
        
    }

    pub fn build_ast(&mut self) {
        Self::build_from_ast_data(
            &self.data.ast_data, 
            &mut self.data.ast_structs,
            &mut self.data.ast_enums);
        Self::build_from_list_data(
            &self.data.list_data, 
            &mut self.data.ast_structs,
            &mut self.data.ast_enums);
        Self::build_enums_from_ast_data(&self.data.ast_data, &mut self.data.ast_enums);
        Self::build_enums_from_list_data(&self.data.list_data, &mut self.data.ast_enums);
    }
}