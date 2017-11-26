use ast::*;

pub trait VisitAst<'a, 'b> {
    fn visit_source(&mut self, node: &'b Source) {
        for node in &node.nodes {
            match node {
                &SourceNode::AstSingle(ref ast_single) => {
                    self.visit_ast_single(ast_single);
                }
                &SourceNode::AstMany(ref ast_many) => self.visit_ast_many(ast_many),
                &SourceNode::List(ref list) => self.visit_list(list)
            }
        }
    }

    fn visit_ast_single(&mut self, node: &'b AstSingle) {
        println!("{}, {:?}", node.ident, node.token_list);
        for item in &node.token_list {
            self.visit_token_node(item);
        }
    }

    fn visit_ast_many(&mut self, node: &'b AstMany) {
        for item in &node.ast_items {
            match item {
                &AstItem::AstDef(ref ast_def) => self.visit_ast_def(ast_def),
                &AstItem::AstRef(ref ast_ref) => self.visit_ast_ref(ast_ref),
            }
        }
    }

    fn visit_ast_ref(&mut self, node: &AstRef) {
        println!("{}", node.ident);
    }

    fn visit_token_node(&mut self, item: &'b TokenNode) {
        match item {
            &TokenNode::TokenKey(ref token_key) => self.visit_token_key(token_key),
            &TokenNode::TokenNamedKey(ref inner) => self.visit_token_named_key(inner)
        }
    }

    fn visit_ast_def(&mut self, node: &'b AstDef) {
        println!("{:?} {:?}", node.ident, node.token_list);
        for item in &node.token_list {
            self.visit_token_node(item);
        }
    }
    
    fn visit_token_named_key(&mut self, node: &'b TokenNamedKey) {
        println!("Token named key {}", node.key);
    }

    fn visit_token_key(&mut self, node: &'b TokenKey) {
        println!("Tokenkey {}", node.ident);
    }

    fn visit_list(&mut self, node: &'b List) {
        println!("List sep {:?}", node.sep);
        for item in &node.items {
            println!("Item sep {:?}", item.sep);
            match item.ast_item {
                AstItem::AstDef(ref ast_def) => self.visit_ast_def(ast_def),
                AstItem::AstRef(ref ast_ref) => self.visit_ast_ref(ast_ref),
            }
        }
    }
}

pub struct Visitor;
impl Visitor {
    pub fn new() -> Visitor {
        Visitor {}
    }
}
impl<'a, 'b> VisitAst<'a, 'b> for Visitor {}