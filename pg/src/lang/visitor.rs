use super::ast::*;

pub trait Visitor<'a> {
    fn visit_plus(&mut self, node: &'a Plus) {
        self.visit_expr(&node.op1);
        self.visit_expr(&node.op2);
    }

    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.exprs {
            self.visit_expr(item);
        }
    }

    fn visit_var_name(&mut self, node: &'a VarName) {
    }

    fn visit_expr(&mut self, node: &'a Expr) {
        match node {
            &Expr::VarNameItem(ref inner) => self.visit_var_name(inner),
            &Expr::PlusItem(ref inner) => self.visit_plus(inner),
        }
    }

}