use super::ast::*;

#[allow(unused_variables,dead_code)]
pub trait Visitor<'a> {
    fn visit_test(&mut self, node: &'a Test) {
    }

}