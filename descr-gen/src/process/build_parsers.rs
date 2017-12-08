use lang_data::data::*;
use lang_data::rule::*;
use lang_data::typed_part::*;
use lang_data::annotations::*;
use descr_lang::gen::ast::*;
use descr_lang::gen::visitor::Visitor;
use std::collections::HashMap;

pub struct BuildParsers<'a, 'd: 'a> {
    data: &'a mut LangData<'d>,
}
impl<'a, 'd: 'a> BuildParsers<'a, 'd> {
    pub fn new(data: &'a mut LangData<'d>) -> BuildParsers<'a, 'd> {
        BuildParsers { data }
    }

    pub fn process_token(token: &Token<'d>, typed_parts: &'a HashMap<&'d str, TypedPart<'d>>) -> AstRulePart<'d> {
        use self::Token::*;
        match token {
            &SimpleTokenItem(ref simple_token) => {
                let (token, member_key) = match &simple_token.token_type {
                    &TokenType::KeyTokenItem(KeyToken { key }) => {
                        let typed_part = typed_parts.get(key).unwrap();
                        (
                            AstRuleToken::Key(key),
                            if typed_part.is_auto_member() {
                                Some(key)
                            } else {
                                None
                            }
                        )
                    },
                    &TokenType::QuotedItem(Quoted { string }) => {
                        (AstRuleToken::Tag(string), None)
                    },
                    &TokenType::FuncTokenItem(ref func_token) => {
                        (AstRuleToken::parse_func_token(func_token), None)
                    }
                };
                AstRulePart {
                    token,
                    member_key,
                    optional: simple_token.optional,
                    not: simple_token.not,
                    annots: parse_annots(&simple_token.annots)
                }
            },
            &NamedTokenItem(ref named_token) => {
                let (token, member_key) = match &named_token.token_type {
                    &TokenType::KeyTokenItem(KeyToken { key }) => {
                        (AstRuleToken::Key(key), Some(named_token.name))
                    },
                    &TokenType::QuotedItem(Quoted { string }) => {
                        (AstRuleToken::Tag(string), Some(named_token.name))
                    },
                    &TokenType::FuncTokenItem(ref func_token) => {
                        (AstRuleToken::parse_func_token(func_token), Some(named_token.name))
                    }
                };
                AstRulePart {
                    token,
                    member_key,
                    optional: named_token.optional,
                    not: named_token.not,
                    annots: parse_annots(&named_token.annots)
                }
            },
            &TokenGroupItem(ref braced_token) => {
                AstRulePart {
                    token: AstRuleToken::Group(
                        braced_token.token_list
                            .iter()
                            .map(|token| {
                                Self::process_token(token, typed_parts)
                            }).collect::<Vec<_>>()
                    ),
                    member_key: None,
                    optional: braced_token.optional,
                    not: braced_token.not,
                    annots: parse_annots(&braced_token.annots)
                }
            }
        }
    }

    pub fn add_tokens_to_rule(
        &mut self,
        is_ast: bool,
        ident: &'d str,
        name: &'d str,
        token_list: &Vec<Token<'d>>,
        annots: AnnotList<'d>
    ) {
        use lang_data::rule::AstRule::*;
        let rule = {
            if is_ast {
                let ast_data = self.data.ast_data.get_mut(ident).unwrap();
                ast_data.rules.push(PartsRule(AstPartsRule::new(name, annots)));
                match ast_data.rules.last_mut().unwrap() {
                    &mut PartsRule(ref mut parts_rule) => parts_rule,
                    _ => panic!(),
                }
            } else {
                let list_data = self.data.list_data.get_mut(ident).unwrap();
                list_data.rules.push(ListRule::new(
                    Some(ident),
                    PartsRule(AstPartsRule::new(name, annots)),
                ));
                match &mut list_data.rules.last_mut().unwrap().ast_rule {
                    &mut PartsRule(ref mut parts_rule) => parts_rule,
                    _ => panic!(),
                }
            }
        };
        for token in token_list {
            rule.parts.push(Self::process_token(token, &self.data.typed_parts));
        }
    }
}
impl<'a, 'd> Visitor<'d> for BuildParsers<'a, 'd> {
    fn visit_ast_single(&mut self, node: &'d AstSingle) {
        //let mut rule = AstPartsRule::new(node.ident);
        //rule.ast_type = node.ident;
        self.add_tokens_to_rule(true, &node.ident, &node.ident, &node.tokens, parse_annots(&node.annots));
    }

    fn visit_ast_many(&mut self, node: &'d AstMany) {
        for item in &node.items {
            use lang_data::rule::AstRule::*;
            use self::AstItem::*;
            match item {
                &AstDefItem(AstDef {
                    ref ident,
                    ref tokens,
                    ref annots
                }) => {
                    let name = ident.unwrap_or(node.ident);
                    self.add_tokens_to_rule(true, node.ident, name, tokens, parse_annots(annots));
                }
                &AstRefItem(AstRef { ref ident }) => {
                    self.data
                        .ast_data
                        .get_mut(node.ident)
                        .unwrap()
                        .rules
                        .push(RefRule(ident));
                }
            }
        }
    }

    fn visit_list_many(&mut self, node: &'d ListMany) {
        for item in &node.items {
            use self::AstItem::*;
            use lang_data::rule::AstRule::*;
            match &item.ast_item {
                &AstDefItem(AstDef {
                    ref ident,
                    ref tokens,
                    ref annots
                }) => {
                    let name = ident.unwrap_or(node.ident);
                    self.add_tokens_to_rule(false, node.ident, name, tokens, parse_annots(annots));
                }
                &AstRefItem(AstRef { ref ident }) => {
                    self.data
                        .list_data
                        .get_mut(node.ident)
                        .unwrap()
                        .rules
                        .push(ListRule::new(item.sep, RefRule(ident)));
                }
            }
        }
    }

    fn visit_list_single(&mut self, node: &'d ListSingle) {
        let list_data = self.data.list_data.get_mut(node.ident).unwrap();
        use lang_data::rule::AstRule::*;
        list_data
            .rules
            .push(ListRule::new(None, RefRule(node.reference)));
    }

}
