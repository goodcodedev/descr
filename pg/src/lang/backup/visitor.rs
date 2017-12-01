use super::ast::*;

pub trait Visitor<'a> {
    fn visit_comment(&mut self, node: &'a Comment) {
    }

    fn visit_random(&mut self, node: &'a Random) {
    }

    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.source_items {
            self.visit_source_items(item);
        }
    }

    fn visit_source_items(&mut self, node: &'a SourceItems) {
        match node {
            &SourceItems::RandomItem(ref inner) => self.visit_random(inner),
            &SourceItems::CommentItem(ref inner) => self.visit_comment(inner),
        }
    }

}