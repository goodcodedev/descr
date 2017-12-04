use super::ast::*;

pub trait Visitor<'a> {
    fn visit_ast_def(&mut self, node: &'a AstDef) {
        for item in &node.tokens {
            self.visit_token(item);
        }
    }

    fn visit_ast_many(&mut self, node: &'a AstMany) {
        for item in &node.items {
            self.visit_ast_item(item);
        }
    }

    fn visit_ast_ref(&mut self, node: &'a AstRef) {
    }

    fn visit_ast_single(&mut self, node: &'a AstSingle) {
        for item in &node.tokens {
            self.visit_token(item);
        }
    }

    fn visit_comment(&mut self, node: &'a Comment) {
    }

    fn visit_func_token(&mut self, node: &'a FuncToken) {
        for item in &node.fn_args {
            self.visit_func_arg(item);
        }
    }

    fn visit_key_token(&mut self, node: &'a KeyToken) {
    }

    fn visit_list_item(&mut self, node: &'a ListItem) {
        self.visit_ast_item(&node.ast_item);
    }

    fn visit_list_many(&mut self, node: &'a ListMany) {
        for item in &node.items {
            self.visit_list_item(item);
        }
    }

    fn visit_list_single(&mut self, node: &'a ListSingle) {
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

    fn visit_ast_item(&mut self, node: &'a AstItem) {
        match node {
            &AstItem::AstDefItem(ref inner) => self.visit_ast_def(inner),
            &AstItem::AstRefItem(ref inner) => self.visit_ast_ref(inner),
        }
    }

    fn visit_func_arg(&mut self, node: &'a FuncArg) {
        match node {
            &FuncArg::QuotedItem(ref inner) => self.visit_quoted(inner),
        }
    }

    fn visit_list(&mut self, node: &'a List) {
        match node {
            &List::ListSingleItem(ref inner) => self.visit_list_single(inner),
            &List::ListManyItem(ref inner) => self.visit_list_many(inner),
        }
    }

    fn visit_source_item(&mut self, node: &'a SourceItem) {
        match node {
            &SourceItem::AstSingleItem(ref inner) => self.visit_ast_single(inner),
            &SourceItem::AstManyItem(ref inner) => self.visit_ast_many(inner),
            &SourceItem::ListItem(ref inner) => self.visit_list(inner),
            &SourceItem::CommentItem(ref inner) => self.visit_comment(inner),
        }
    }

    fn visit_token(&mut self, node: &'a Token) {
        match node {
            &Token::NamedTokenItem(ref inner) => self.visit_named_token(inner),
            &Token::SimpleTokenItem(ref inner) => self.visit_simple_token(inner),
        }
    }

    fn visit_token_type(&mut self, node: &'a TokenType) {
        match node {
            &TokenType::FuncTokenItem(ref inner) => self.visit_func_token(inner),
            &TokenType::KeyTokenItem(ref inner) => self.visit_key_token(inner),
            &TokenType::QuotedItem(ref inner) => self.visit_quoted(inner),
        }
    }

}