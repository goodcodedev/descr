use super::ast::*;

#[allow(unused_variables,dead_code)]
pub trait Visitor<'a> {
    fn visit_ast_single(&mut self, node: &'a AstSingle) {
        for item in &node.tokens {
            self.visit_token(item);
        }
    }

    fn visit_key_token(&mut self, node: &'a KeyToken) {
    }

    fn visit_named_token(&mut self, node: &'a NamedToken) {
        self.visit_token_type(&node.token_type);
    }

    fn visit_quoted(&mut self, node: &'a Quoted) {
    }

    fn visit_simple_token(&mut self, node: &'a SimpleToken) {
        self.visit_token_type(&node.token_type);
    }

    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.items {
            self.visit_source_item(item);
        }
    }

    fn visit_token_group(&mut self, node: &'a TokenGroup) {
        for item in &node.token_list {
            self.visit_token(item);
        }
    }

    fn visit_source_item(&mut self, node: &'a SourceItem) {
        match node {
            &SourceItem::AstSingleItem(ref inner) => self.visit_ast_single(inner),
        }
    }

    fn visit_token(&mut self, node: &'a Token) {
        match node {
            &Token::NamedTokenItem(ref inner) => self.visit_named_token(inner),
            &Token::SimpleTokenItem(ref inner) => self.visit_simple_token(inner),
            &Token::TokenGroupItem(ref inner) => self.visit_token_group(inner),
        }
    }

    fn visit_token_type(&mut self, node: &'a TokenType) {
        match node {
            &TokenType::KeyTokenItem(ref inner) => self.visit_key_token(inner),
            &TokenType::QuotedItem(ref inner) => self.visit_quoted(inner),
        }
    }

}