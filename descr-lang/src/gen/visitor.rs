use super::ast::*;

pub trait Visitor<'a, 'b> {
    fn visit_ast_def(&mut self, node: &'b AstDef) {
        for item in &node.tokens {
            self.visit_token(item);
        }
    }

    fn visit_ast_many(&mut self, node: &'b AstMany) {
        for item in &node.items {
            self.visit_ast_item(item);
        }
    }

    fn visit_ast_ref(&mut self, node: &'b AstRef) {
    }

    fn visit_ast_single(&mut self, node: &'b AstSingle) {
        for item in &node.tokens {
            self.visit_token(item);
        }
    }

    fn visit_list_item(&mut self, node: &'b ListItem) {
        self.visit_ast_item(&node.ast_item);
    }

    fn visit_list_many(&mut self, node: &'b ListMany) {
        for item in &node.items {
            self.visit_list_item(item);
        }
    }

    fn visit_list_single(&mut self, node: &'b ListSingle) {
    }

    fn visit_source(&mut self, node: &'b Source) {
        for item in &node.items {
            self.visit_source_item(item);
        }
    }

    fn visit_token_key(&mut self, node: &'b TokenKey) {
    }

    fn visit_token_named_key(&mut self, node: &'b TokenNamedKey) {
    }

    fn visit_ast_item(&mut self, node: &'b AstItem) {
        match node {
            &AstItem::AstDefItem(ref inner) => self.visit_ast_def(inner),
            &AstItem::AstRefItem(ref inner) => self.visit_ast_ref(inner),
        }
    }

    fn visit_list(&mut self, node: &'b List) {
        match node {
            &List::ListSingleItem(ref inner) => self.visit_list_single(inner),
            &List::ListManyItem(ref inner) => self.visit_list_many(inner),
        }
    }

    fn visit_source_item(&mut self, node: &'b SourceItem) {
        match node {
            &SourceItem::AstSingleItem(ref inner) => self.visit_ast_single(inner),
            &SourceItem::AstManyItem(ref inner) => self.visit_ast_many(inner),
            &SourceItem::ListItem(ref inner) => self.visit_list(inner),
        }
    }

    fn visit_token(&mut self, node: &'b Token) {
        match node {
            &Token::TokenKeyItem(ref inner) => self.visit_token_key(inner),
            &Token::TokenNamedKeyItem(ref inner) => self.visit_token_named_key(inner),
        }
    }

}