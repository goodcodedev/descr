use super::ast::*;

pub trait Visitor<'a> {
    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.items {
            self.visit_source_item(item);
        }
    }

    fn visit_test_item(&mut self, node: &'a TestItem) {
    }

    fn visit_source_item(&mut self, node: &'a SourceItem) {
        match node {
            &SourceItem::TestItemItem(ref inner) => self.visit_test_item(inner),
        }
    }

}