use super::ast::*;

pub trait Visitor<'a> {
    fn visit_comment(&mut self, node: &'a Comment) {
    }

    fn visit_random(&mut self, node: &'a Random) {
    }

    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.items {
            self.visit_source_item(item);
        }
    }

    fn visit_source_item(&mut self, node: &'a SourceItem) {
        match node {
            &SourceItem::RandomItem(ref inner) => self.visit_random(inner),
            &SourceItem::CommentItem(ref inner) => self.visit_comment(inner),
        }
    }

}