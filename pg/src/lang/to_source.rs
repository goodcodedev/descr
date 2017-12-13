use super::ast::*;

pub struct ToSource;
#[allow(unused_variables,dead_code)]
impl<'a> ToSource {
    pub fn test(mut s: String, node: &'a Test) -> String {
        s += " ";
        s += "First";
        s += " ";
        s += "Second";
        s
    }

}