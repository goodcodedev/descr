use super::ast::*;

pub trait Visitor<'a> {
    fn visit_hello(&mut self, node: &'a Hello) {
    }

    fn visit_say(&mut self, node: &'a Say) {
    }

    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.items {
            self.visit_source_item(item);
        }
    }

    fn visit_source_item(&mut self, node: &'a SourceItem) {
        match node {
            &SourceItem::SayItem(ref inner) => self.visit_say(inner),
            &SourceItem::HelloItem(ref inner) => self.visit_hello(inner),
        }
    }

}