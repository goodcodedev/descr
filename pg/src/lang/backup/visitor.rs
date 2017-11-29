use super::ast::*;

pub trait Visitor<'a> {
    fn visit_first(&mut self, node: &'a First) {
        self.visit_second(&node.second);
    }

    fn visit_second(&mut self, node: &'a Second) {
    }

}