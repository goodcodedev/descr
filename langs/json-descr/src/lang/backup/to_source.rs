use super::ast::*;

pub struct ToSource;
impl<'a> ToSource {
    pub fn to_source_js_object(mut s: String, node: &'a JsObject) -> String {
        s += " ";
        s.push('{');
        s += " ";
        let len = node.items.len();
        for (i, item) in node.items.iter().enumerate() {
            s = Self::to_source_object_pair(s, item);
            if i < len - 1 {         s.push(',');
 }
        }
        s += " ";
        s.push('}');
        s
    }

    pub fn to_source_int(mut s: String, node: &'a Int) -> String {
        s += " ";
        s += &node.int.to_string();
        s
    }

    pub fn to_source_string_val(mut s: String, node: &'a StringVal) -> String {
        s += " ";
        s += node.string;
        s
    }

    pub fn to_source_object_pair(mut s: String, node: &'a ObjectPair) -> String {
        s += " ";
        s += node.key;
        s += " ";
        s.push(':');
        s += " ";
        s = Self::to_source_js_val(s, &node.js_val);
        s
    }

    pub fn to_source_array_val(mut s: String, node: &'a ArrayVal) -> String {
        s += " ";
        s.push('[');
        s += " ";
        let len = node.array_vals.len();
        for (i, item) in node.array_vals.iter().enumerate() {
            s = Self::to_source_js_val(s, item);
            if i < len - 1 {         s.push(',');
 }
        }
        s += " ";
        s.push(']');
        s
    }

    pub fn to_source_js_val(s: String, node: &'a JsVal) -> String {
        match node {
            &JsVal::IntItem(ref inner) => Self::to_source_int(s, inner),
            &JsVal::StringValItem(ref inner) => Self::to_source_string_val(s, inner),
            &JsVal::ArrayValItem(ref inner) => Self::to_source_array_val(s, inner),
            &JsVal::JsObjectItem(ref inner) => Self::to_source_js_object(s, inner),
        }
    }

}