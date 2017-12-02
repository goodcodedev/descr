use super::ast::*;

pub trait Visitor<'a> {
    fn visit_bg_color(&mut self, node: &'a BgColor) {
        self.visit_color(&node.color);
    }

    fn visit_say(&mut self, node: &'a Say) {
    }

    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.statements {
            self.visit_statement(item);
        }
    }

    fn visit_color(&mut self, node: &'a Color) {
    }

    fn visit_statement(&mut self, node: &'a Statement) {
        match node {
            &Statement::SayItem(ref inner) => self.visit_say(inner),
            &Statement::BgColorItem(ref inner) => self.visit_bg_color(inner),
        }
    }

}