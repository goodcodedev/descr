use super::ast::*;

pub trait Visitor<'a> {
    fn visit_ast_name(&mut self, node: &'a AstName) {
    }

    fn visit_container(&mut self, node: &'a Container) {
        self.visit_ast_name(&node.ast_name);
    }

}