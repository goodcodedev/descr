use visit_ast::*;
use ast::*;
use ast;
use lang_data::*;

pub struct BuildParsers<'a, 'd: 'a> {
    data: &'a mut LangData<'d>
}
impl<'a, 'd: 'a> BuildParsers<'a, 'd> {
    pub fn new(data: &'a mut LangData<'d>) -> BuildParsers<'a, 'd> {
        BuildParsers {
            data
        }
    }
    pub fn add_tokens_to_rule(&mut self, is_ast: bool, ident: &'d str, name: &'d str, 
                              token_list: &Vec<TokenNode<'d>>) {
        use lang_data::AstRule::*;
        let rule = {
            if is_ast {
                let ast_data = self.data.ast_data.get_mut(ident).unwrap();
                ast_data.rules.push(PartsRule(AstPartsRule::new(name)));
                match ast_data.rules.last_mut().unwrap() {
                    &mut PartsRule(ref mut parts_rule) => parts_rule,
                    _ => panic!()
                }
            } else {
                let list_data = self.data.list_data.get_mut(ident).unwrap();
                list_data.rules.push(ListRule::new(
                    Some(ident),
                    PartsRule(AstPartsRule::new(name))
                ));
                match &mut list_data.rules.last_mut().unwrap().ast_rule {
                    &mut PartsRule(ref mut parts_rule) => parts_rule,
                    _ => panic!()
                }
            }
        };
        for (i, token) in token_list.iter().enumerate() {
            use ast::TokenNode::*;
            use lang_data::TypedPart::*;
            match token {
                &TokenKey(ast::TokenKey{ident, optional}) => {
                    let part = self.data.typed_parts.get(ident).unwrap();
                    rule.part_keys.push(ident);
                    // Key index on parts considered
                    // members
                    let member_key = match part {
                        &AstPart { .. }
                        | &ListPart { .. }
                        | &IntPart { .. }
                        | &IdentPart { .. } => {
                            rule.member_idxs.insert(ident, i);
                            rule.idx_members.insert(i, ident);
                            Some(ident)
                        },
                        _ => None
                    };
                    rule.parts.push(AstRulePart {
                        part_key: ident,
                        member_key,
                        optional
                    });
                },
                &TokenNamedKey(ast::TokenNamedKey{name, key, optional}) => {
                    let part = self.data.typed_parts.get(key).unwrap();
                    rule.part_keys.push(key);
                    // Key index by name
                    // This includes more types
                    // as a way to set members
                    let member_key = match part {
                        &AstPart { .. }
                        | &ListPart { .. }
                        | &IntPart { .. }
                        | &IdentPart { .. }
                        | &CharPart { .. }
                        | &TagPart { .. } => {
                            rule.member_idxs.insert(name, i);
                            rule.idx_members.insert(i, name);
                            Some(name)
                        }
                    };
                    rule.parts.push(AstRulePart {
                        part_key: key,
                        member_key,
                        optional
                    });
                }
            }
        }
    }
}
impl<'a, 'd> VisitAst<'a, 'd> for BuildParsers<'a, 'd> {
    fn visit_ast_many(&mut self, node: &'d AstMany) {
        for item in &node.ast_items {
            use lang_data::AstRule::*;
            use ast::AstItem::*;
            match item {
                &AstDef(ast::AstDef{ref ident, ref token_list}) => {
                    let name = ident.unwrap_or(node.ident);
                    self.add_tokens_to_rule(true, node.ident, name, token_list);
                },
                &AstRef(ast::AstRef{ref ident}) => {
                    self.data.ast_data.get_mut(node.ident).unwrap().rules.push(RefRule(ident));
                }
            }
        }
    }

    fn visit_list(&mut self, node: &'d List) {
        for item in &node.items {
            use ast::AstItem::*;
            use lang_data::AstRule::*;
            match &item.ast_item {
                &AstDef(ast::AstDef{ref ident, ref token_list}) => {
                    let name = ident.unwrap_or(node.ident);
                    self.add_tokens_to_rule(false, name, name, token_list);
                },
                &AstRef(ast::AstRef{ref ident}) => {
                    self.data.list_data.get_mut(node.ident).unwrap().rules.push(
                        ListRule::new(Some(""), RefRule(ident))
                    );
                }
            }
        }
    }

    fn visit_ast_single(&mut self, node: &'d AstSingle) {
        let mut rule = AstPartsRule::new(node.ident);
        rule.ast_type = node.ident;
        self.add_tokens_to_rule(true, &node.ident, &node.ident, &node.token_list);
    }
}