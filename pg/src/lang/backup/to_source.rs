use super::ast::*;

pub struct ToSource;
impl<'a> ToSource {
    pub fn to_source_plus(mut s: String, node: &'a Plus) -> String {
        s += " ";
        s = Self::to_source_expr(s, &node.op1);
        s += " ";
        s += "+";
        s += " ";
        s = Self::to_source_expr(s, &node.op2);
        s
    }

    pub fn to_source_var_name(mut s: String, node: &'a VarName) -> String {
        s += " ";
        s += node.ident;
        s
    }

    pub fn to_source_expr(s: String, node: &'a Expr) -> String {
        match node {
            &Expr::VarNameItem(ref inner) => Self::to_source_var_name(s, inner),
            &Expr::PlusItem(ref inner) => Self::to_source_plus(s, inner),
        }
    }

}