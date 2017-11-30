use lang_data::data::*;
use lang_data::rule::*;
use descr_lang::gen::ast::*;
use descr_lang::gen::visitor::Visitor;

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
                              token_list: &Vec<Token<'d>>) {
        use lang_data::rule::AstRule::*;
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
            use self::Token::*;
            use lang_data::typed_part::TypedPart::*;
            match token {
                &SimpleTokenItem(SimpleToken{ref token_type, optional, not}) => {
                    match token_type {
                        &TokenType::KeyTokenItem(KeyToken { key }) => {
                            let part = self.data.typed_parts.get(key).unwrap();
                            // Key index on parts considered
                            // members
                            let member_key = match part {
                                &AstPart { .. }
                                | &ListPart { .. }
                                | &IntPart { .. }
                                | &StringPart { .. }
                                | &IdentPart { .. } => {
                                    rule.member_idxs.insert(key, i);
                                    rule.idx_members.insert(i, key);
                                    Some(key)
                                },
                                _ => None
                            };
                            rule.parts.push(AstRulePart {
                                token: AstRuleToken::Key(key),
                                member_key,
                                optional,
                                not
                            });
                        },
                        &TokenType::QuotedItem(Quoted { string }) => {
                            // Tag rule, not considered member
                            rule.parts.push(AstRulePart {
                                token: AstRuleToken::Tag(string),
                                member_key: None,
                                optional,
                                not
                            });
                        }
                    }
                },
                &NamedTokenItem(NamedToken{ref token_type, name, optional, not}) => {
                    match token_type {
                        &TokenType::KeyTokenItem(KeyToken { key }) => {
                            let part = self.data.typed_parts.get(key).unwrap();
                            // Key index by name
                            // This includes more types
                            // as a way to set members
                            let member_key = match part {
                                &AstPart { .. }
                                | &ListPart { .. }
                                | &IntPart { .. }
                                | &IdentPart { .. }
                                | &CharPart { .. }
                                | &FnPart { .. }
                                | &StringPart { .. }
                                | &TagPart { .. } => {
                                    rule.member_idxs.insert(name, i);
                                    rule.idx_members.insert(i, name);
                                    Some(name)
                                }
                            };
                            rule.parts.push(AstRulePart {
                                token: AstRuleToken::Key(key),
                                member_key,
                                optional,
                                not
                            });
                        },
                        &TokenType::QuotedItem(Quoted { string }) => {
                            // Tag rule, not considered member
                            rule.parts.push(AstRulePart {
                                token: AstRuleToken::Tag(string),
                                member_key: Some(name),
                                optional,
                                not
                            });
                        }
                    }
                }
            }
        }
    }
}
impl<'a, 'd> Visitor<'d> for BuildParsers<'a, 'd> {
    fn visit_ast_many(&mut self, node: &'d AstMany) {
        for item in &node.items {
            use lang_data::rule::AstRule::*;
            use self::AstItem::*;
            match item {
                &AstDefItem(AstDef{ref ident, ref tokens}) => {
                    let name = ident.unwrap_or(node.ident);
                    self.add_tokens_to_rule(true, node.ident, name, tokens);
                },
                &AstRefItem(AstRef{ref ident}) => {
                    self.data.ast_data.get_mut(node.ident).unwrap().rules.push(RefRule(ident));
                }
            }
        }
    }

    fn visit_list_many(&mut self, node: &'d ListMany) {
        for item in &node.items {
            use self::AstItem::*;
            use lang_data::rule::AstRule::*;
            match &item.ast_item {
                &AstDefItem(AstDef{ref ident, ref tokens}) => {
                    let name = ident.unwrap_or(node.ident);
                    self.add_tokens_to_rule(false, node.ident, name, tokens);
                },
                &AstRefItem(AstRef{ref ident}) => {
                    self.data.list_data.get_mut(node.ident).unwrap().rules.push(
                        ListRule::new(item.sep, RefRule(ident))
                    );
                }
            }
        }
    }

    fn visit_list_single(&mut self, node: &'d ListSingle) {
        let list_data = self.data.list_data.get_mut(node.ident).unwrap();
        use lang_data::rule::AstRule::*;
        list_data.rules.push(
            ListRule::new(None, RefRule(node.reference))
        );
    }

    fn visit_ast_single(&mut self, node: &'d AstSingle) {
        //let mut rule = AstPartsRule::new(node.ident);
        //rule.ast_type = node.ident;
        self.add_tokens_to_rule(true, &node.ident, &node.ident, &node.tokens);
    }
}