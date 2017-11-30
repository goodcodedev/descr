use super::ast::*;

pub trait Visitor<'a> {
    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.some_list {
            self.visit_something(item);
        }
    }

    fn visit_str2_item(&mut self, node: &'a Str2Item) {
    }

    fn visit_str_item(&mut self, node: &'a StrItem) {
    }

    fn visit_something(&mut self, node: &'a Something) {
        match node {
            &Something::StrItemItem(ref inner) => self.visit_str_item(inner),
            &Something::Str2ItemItem(ref inner) => self.visit_str2_item(inner),
        }
    }

}