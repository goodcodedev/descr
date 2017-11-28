pub trait Visitor<'a, 'b> {
    pub fn visit_ast_def(&mut self, node: &'b AstDef) {
        for item in &node.tokens {
            self.visit_token(item);
        }
    }

    pub fn visit_ast_many(&mut self, node: &'b AstMany) {
        for item in &node.items {
            self.visit_ast_item(item);
        }
    }

    pub fn visit_ast_ref(&mut self, node: &'b AstRef) {
    }

    pub fn visit_ast_single(&mut self, node: &'b AstSingle) {
        for item in &node.tokens {
            self.visit_token(item);
        }
    }

    pub fn visit_list_item(&mut self, node: &'b ListItem) {
        self.visit_ast_item(node.ast_item);
    }

    pub fn visit_list_many(&mut self, node: &'b ListMany) {
        for item in &node.items {
            self.visit_list_item(item);
        }
    }

    pub fn visit_list_single(&mut self, node: &'b ListSingle) {
    }

    pub fn visit_token_key(&mut self, node: &'b TokenKey) {
    }

    pub fn visit_token_named_key(&mut self, node: &'b TokenNamedKey) {
    }

    pub fn visit_ast_item(&mut self, node: &'b AstItem) {
        match node {
            &AstItem::AstDefItem(ref inner) => self.visit_ast_def(inner);
            &AstItem::AstRefItem(ref inner) => self.visit_ast_ref(inner);
        }
    }

    pub fn visit_list(&mut self, node: &'b List) {
        match node {
            &List::ListSingleItem(ref inner) => self.visit_list_single(inner);
            &List::ListManyItem(ref inner) => self.visit_list_many(inner);
        }
    }

    pub fn visit_source(&mut self, node: &'b Source) {
        match node {
            &Source::AstSingleItem(ref inner) => self.visit_ast_single(inner);
            &Source::AstManyItem(ref inner) => self.visit_ast_many(inner);
            &Source::ListItem(ref inner) => self.visit_list(inner);
        }
    }

    pub fn visit_token(&mut self, node: &'b Token) {
        match node {
            &Token::TokenKeyItem(ref inner) => self.visit_token_key(inner);
            &Token::TokenNamedKeyItem(ref inner) => self.visit_token_named_key(inner);
        }
    }

}