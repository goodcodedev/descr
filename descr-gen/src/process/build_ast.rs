use lang_data::data::*;
use lang_data::ast::*;
use lang_data::typed_part::*;
use lang_data::rule::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct BuildAst<'a, 'd: 'a> {
    data: &'a mut LangData<'d>,
}
impl<'a, 'd: 'a> BuildAst<'a, 'd> {
    pub fn new(data: &'a mut LangData<'d>) -> BuildAst<'a, 'd> {
        BuildAst { data }
    }

    fn reg_struct(
        data: &mut HashMap<&'d str, AstStruct<'d>>,
        name: &'d str,
        snake_cased: &mut SnakeCased<'d>,
    ) {
        if !data.contains_key(name) {
            data.insert(name, AstStruct::new(name, snake_cased.get(name)));
        } else {
            // Increment counter to match against arg counter
            let ast_struct = data.get_mut(name).unwrap();
            ast_struct.num_patterns += 1;
        }
    }

    fn reg_struct_member(
        data: &mut HashMap<&'d str, AstStruct<'d>>,
        struct_name: &'d str,
        member_name: &'d str,
        part_key: &'d str,
        optional: bool,
        not: bool,
        tag: bool,
        snake_cased: &mut SnakeCased<'d>,
    ) {
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
                    not,
                    if not {
                        AstMemberType::NotString
                    } else if tag {
                        AstMemberType::TagBool(part_key)
                    } else {
                        AstMemberType::KeyedToken(part_key)
                    },
                ),
            );
        }
    }

    fn process_parts_rule(
        rule: &AstPartsRule<'d>,
        struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
        typed_parts: &HashMap<&'d str, TypedPart<'d>>,
        snake_cased: &mut SnakeCased<'d>,
    ) {
        for part in &rule.parts {
            if let Some((member_key, part_key, is_tag)) = match &part.token {
                &AstRuleToken::Key(key) => {
                    let typed_part = typed_parts.get(key).unwrap();
                    if typed_part.is_auto_member() {
                        Some((part.member_key.unwrap_or(key), key, false))
                    } else {
                        match typed_part {
                            &TypedPart::CharPart { .. }
                            | &TypedPart::FnPart { .. }
                            | &TypedPart::WSPart { .. } => {
                                // Count as member if
                                // member key is given
                                part.member_key.map(|member_key| {
                                    (member_key, key, false)
                                })
                            },
                            &TypedPart::TagPart { .. } => {
                                // This might be handled
                                // in AstRuleToken::Tag now
                                part.member_key.map(|member_key| {
                                    (member_key, key, true)
                                })
                            }
                            _ => None
                        }
                    }
                },
                &AstRuleToken::Tag(string) => {
                    part.member_key.map(|member_key| { (member_key, string, true) })
                },
                &AstRuleToken::Func(..) => {
                    part.member_key.map(|member_key| { (member_key, "", false) })
                },
                &AstRuleToken::Group(..) => None
            } {
                Self::reg_struct_member(
                    struct_data,
                    rule.ast_type,
                    member_key,
                    part_key,
                    part.optional,
                    part.not,
                    is_tag,
                    snake_cased,
                );
            }
            /*
            match &part.token {
                &AstRuleToken::Key(key) => {
                    use lang_data::typed_part::TypedPart::*;
                    match typed_parts.get(key).unwrap() {
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
                                false,
                                snake_cased,
                            );
                        }
                        &CharPart { .. } | &FnPart { .. } | &WSPart => {
                            // Count as member if member name is given
                            if let Some(member_key) = part.member_key {
                                Self::reg_struct_member(
                                    struct_data,
                                    rule.ast_type,
                                    member_key,
                                    key,
                                    part.optional,
                                    part.not,
                                    false,
                                    snake_cased,
                                );
                            }
                        }
                        &TagPart { .. } => {
                            // I think this is handled below in AstRuleToken::Tag.
                            // Need big cleanup..
                            // Count as member if member name is given
                            if let Some(member_key) = part.member_key {
                                Self::reg_struct_member(
                                    struct_data,
                                    rule.ast_type,
                                    member_key,
                                    key,
                                    part.optional,
                                    part.not,
                                    true,
                                    snake_cased,
                                );
                            }
                        }
                    }
                }
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
                            true,
                            snake_cased,
                        );
                    }
                }
                &AstRuleToken::Func(..) => {
                    if let Some(member_key) = part.member_key {
                        Self::reg_struct_member(
                            struct_data,
                            rule.ast_type,
                            member_key,
                            "",
                            part.optional,
                            part.not,
                            false,
                            snake_cased,
                        );
                    }
                }
            }
            */
        }
    }

    fn build_from_ast_data(
        ast_data: &HashMap<&'d str, AstData<'d>>,
        struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
        enum_data: &mut HashMap<&'d str, AstEnum<'d>>,
        typed_parts: &HashMap<&'d str, TypedPart<'d>>,
        rule_types: &mut HashMap<&'d str, RuleType<'d>>,
        snake_cased: &mut SnakeCased<'d>,
    ) {
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
                    }
                    &AstRule::PartsRule(ref rule) => {
                        types.insert(rule.ast_type);
                        Self::reg_struct(struct_data, rule.ast_type, snake_cased);
                        Self::process_parts_rule(rule, struct_data, typed_parts, snake_cased);
                    }
                }
            }
            match types.len() {
                0 => {}
                1 => {
                    rule_types.insert(ast_data.key, RuleType::SingleType(ast_data.ast_type));
                }
                _ => {
                    // Several types registered, create enum
                    let mut e = AstEnum::new(ast_data.ast_type, snake_cased.get(ast_data.ast_type));
                    let mut added = HashSet::new();
                    for rule in &ast_data.rules {
                        let item = match rule {
                            &AstRule::RefRule(key_ref) => key_ref,
                            &AstRule::PartsRule(ref rule) => rule.ast_type,
                        };
                        if !added.contains(item) {
                            e.items.push(item);
                            added.insert(item);
                        }
                    }
                    enum_data.insert(ast_data.ast_type, e);
                    rule_types.insert(ast_data.key, RuleType::ManyType(ast_data.ast_type));
                }
            }
        }
    }

    fn build_from_list_data(
        list_data: &HashMap<&'d str, ListData<'d>>,
        struct_data: &mut HashMap<&'d str, AstStruct<'d>>,
        enum_data: &mut HashMap<&'d str, AstEnum<'d>>,
        typed_parts: &HashMap<&'d str, TypedPart<'d>>,
        rule_types: &mut HashMap<&'d str, RuleType<'d>>,
        snake_cased: &mut SnakeCased<'d>,
    ) {
        for (key, list_data) in list_data {
            snake_cased.reg(key);
            let mut types = HashSet::new();
            list_data.ast_type.map(|t| {
                types.insert(t);
            });
            for rule in &list_data.rules {
                match &rule.ast_rule {
                    &AstRule::RefRule(key_ref) => {
                        // Ref to another ast
                        types.insert(key_ref);
                    }
                    &AstRule::PartsRule(ref rule) => {
                        types.insert(rule.ast_type);
                        Self::reg_struct(struct_data, rule.ast_type, snake_cased);
                        Self::process_parts_rule(rule, struct_data, typed_parts, snake_cased);
                    }
                }
            }
            match types.len() {
                0 => {}
                1 => {
                    let v = types.iter().collect::<Vec<_>>();
                    let type_name = v.first().unwrap();
                    rule_types.insert(list_data.key, RuleType::SingleType(type_name));
                }
                _ => {
                    let enum_name = match list_data.ast_type {
                        Some(ast_type) => ast_type,
                        None => list_data.key,
                    };
                    let mut e = AstEnum::new(enum_name, snake_cased.get(enum_name));
                    let mut added = HashSet::new();
                    for rule in &list_data.rules {
                        let item = match &rule.ast_rule {
                            &AstRule::RefRule(key_ref) => key_ref,
                            &AstRule::PartsRule(ref rule) => rule.ast_type,
                        };
                        if !added.contains(item) {
                            e.items.push(item);
                            added.insert(item);
                        }
                    }
                    enum_data.insert(enum_name, e);
                    rule_types.insert(list_data.key, RuleType::ManyType(enum_name));
                }
            }
        }
    }

    pub fn check_simple(&mut self) {
        // Check for simple enums
        // Not sure how to best do this,
        // it makes the result a bit
        // unpredictable. Maybe some
        // annotations later
        for (key, enum_data) in &self.data.ast_enums {
            if enum_data.is_simple(self.data) {
                self.data.simple_enums.insert(key);
                for item in &enum_data.items {
                    self.data.simple_structs.insert(item);
                    self.data.ast_structs.remove(item);
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
            &mut self.data.rule_types,
            &mut self.data.snake_cased,
        );
        Self::build_from_list_data(
            &self.data.list_data,
            &mut self.data.ast_structs,
            &mut self.data.ast_enums,
            &self.data.typed_parts,
            &mut self.data.rule_types,
            &mut self.data.snake_cased,
        );
        self.check_simple();
        // Build parent refs
        // Set member type
        for (struct_name, struct_data) in &self.data.ast_structs {
            for (member_name, member) in &struct_data.members {
                match self.data.get_ast_key(member.part_key) {
                    Some(ast_key) => {
                        // Insert struct/enum members
                        match self.data.resolve(member.part_key) {
                            ResolvedType::ResolvedStruct(key) => {
                                self.data.parent_refs.add_ref(
                                    key,
                                    ParentRef::StructMember {
                                        struct_name,
                                        member_name,
                                    },
                                );
                            }
                            ResolvedType::ResolvedEnum(key) => {
                                self.data.parent_refs.add_ref(
                                    key,
                                    ParentRef::StructMember {
                                        struct_name,
                                        member_name,
                                    },
                                );
                            }
                        }
                    }
                    None => {}
                }
            }
        }
        for (enum_name, enum_data) in &self.data.ast_enums {
            if enum_data.is_simple(self.data) {
                continue;
            }
            for item in &enum_data.items {
                // Insert struct/enum members
                match self.data.resolve(item) {
                    ResolvedType::ResolvedStruct(key) => {
                        self.data.parent_refs.add_ref(
                            key,
                            ParentRef::EnumItem {
                                enum_name,
                                item_name: item,
                            },
                        );
                    }
                    ResolvedType::ResolvedEnum(key) => {
                        self.data.parent_refs.add_ref(
                            key,
                            ParentRef::EnumItem {
                                enum_name,
                                item_name: item,
                            },
                        );
                    }
                }
            }
        }
        //println!("{:#?}", self.data.parent_refs);
        // Go through each struct member, and check
        // for parent reference.
        // If found, set the reference to boxed
        let mut to_box = Vec::new();
        let mut visited = HashMap::new();
        use std::collections::HashSet;
        for (struct_name, struct_data) in &self.data.ast_structs {
            for (member_name, member) in &struct_data.members {
                match self.data.get_ast_key(member.part_key) {
                    Some(ast_key) => {
                        if !visited.contains_key(ast_key) {
                            visited.insert(ast_key, HashSet::new());
                        }
                        to_box.append(&mut Self::set_boxed(
                            ast_key,
                            struct_name,
                            &self.data.parent_refs,
                            visited.get_mut(ast_key).unwrap(),
                        ));
                    }
                    None => {}
                }
            }
        }
        //println!("{:#?}", to_box);
        for to_box_item in &to_box {
            match to_box_item {
                &ToBox::StructMember {
                    struct_name,
                    member_name,
                } => {
                    let struct_data = self.data.ast_structs.get_mut(struct_name).unwrap();
                    let member_data = struct_data.members.get_mut(member_name).unwrap();
                    member_data.boxed = true;
                }
                &ToBox::EnumItem {
                    enum_name,
                    item_name,
                } => {
                    let enum_data = self.data.ast_enums.get_mut(enum_name).unwrap();
                    enum_data.boxed_items.insert(item_name);
                }
            }
        }
    }

    fn set_boxed(
        to_box: &'d str,
        from_ast: &'d str,
        parent_refs: &ParentRefs<'d>,
        visited: &mut HashSet<&'d str>,
    ) -> Vec<ToBox<'d>> {
        let mut v = Vec::new();
        if visited.contains(from_ast) {
            return v;
        }
        visited.insert(from_ast);
        if parent_refs.refs.contains_key(from_ast) {
            for parent_ref in parent_refs.refs.get(from_ast).unwrap() {
                //println!("Traversed: {}, {:?}",to_box, parent_ref);
                match parent_ref {
                    &ParentRef::StructMember {
                        struct_name,
                        member_name,
                    } => {
                        if struct_name == to_box {
                            v.push(ToBox::StructMember {
                                struct_name,
                                member_name,
                            });
                        }
                        v.append(&mut Self::set_boxed(
                            to_box,
                            struct_name,
                            parent_refs,
                            visited,
                        ));
                    }
                    &ParentRef::EnumItem {
                        enum_name,
                        item_name,
                    } => {
                        if enum_name == to_box {
                            v.push(ToBox::EnumItem {
                                enum_name,
                                item_name,
                            });
                        }
                        v.append(&mut Self::set_boxed(
                            to_box,
                            enum_name,
                            parent_refs,
                            visited,
                        ));
                    }
                }
            }
        }
        v
    }
}

#[derive(Debug)]
enum ToBox<'a> {
    StructMember {
        struct_name: &'a str,
        member_name: &'a str,
    },
    EnumItem {
        enum_name: &'a str,
        item_name: &'a str,
    },
}
