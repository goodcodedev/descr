use super::ast::*;

#[allow(unused_variables,dead_code)]
pub trait Visitor<'a> {
    fn visit_array_val(&mut self, node: &'a ArrayVal) {
        for item in &node.items {
            self.visit_js_val(item);
        }
    }

    fn visit_int(&mut self, node: &'a Int) {
    }

    fn visit_js_object(&mut self, node: &'a JsObject) {
        for item in &node.items {
            self.visit_object_pair(item);
        }
    }

    fn visit_object_pair(&mut self, node: &'a ObjectPair) {
        self.visit_js_val(&node.val);
    }

    fn visit_string_val(&mut self, node: &'a StringVal) {
    }

    fn visit_js_val(&mut self, node: &'a JsVal) {
        match node {
            &JsVal::IntItem(ref inner) => self.visit_int(inner),
            &JsVal::StringValItem(ref inner) => self.visit_string_val(inner),
            &JsVal::ArrayValItem(ref inner) => self.visit_array_val(inner),
            &JsVal::JsObjectItem(ref inner) => self.visit_js_object(inner),
        }
    }

}