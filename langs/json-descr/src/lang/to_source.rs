use super::ast::*;

pub struct ToSource;
#[allow(unused_variables,dead_code)]
impl<'a> ToSource {
    pub fn array_val(mut s: String, node: &'a ArrayVal) -> String {
        s += " ";
        s.push('[');
        s += " ";
        let len = node.items.len();
        for (i, item) in node.items.iter().enumerate() {
            s = Self::js_val(s, item);
            if i < len - 1 {         s.push(',');
 }
        }
        s += " ";
        s.push(']');
        s
    }

    pub fn string_val(mut s: String, node: &'a StringVal) -> String {
        s += " ";
        s += "\"";
        s += node.string.as_str();
        s += "\"";
        s
    }

    pub fn js_object(mut s: String, node: &'a JsObject) -> String {
        s += " ";
        s.push('{');
        s += " ";
        let len = node.items.len();
        for (i, item) in node.items.iter().enumerate() {
            s = Self::object_pair(s, item);
            if i < len - 1 {         s.push(',');
 }
        }
        s += " ";
        s.push('}');
        s
    }

    pub fn int(mut s: String, node: &'a Int) -> String {
        s += " ";
        s += &node.int.to_string();
        s
    }

    pub fn object_pair(mut s: String, node: &'a ObjectPair) -> String {
        s += " ";
        s += "\"";
        s += node.key.as_str();
        s += "\"";
        s += " ";
        s.push(':');
        s += " ";
        s = Self::js_val(s, &node.val);
        s
    }

    pub fn js_val(s: String, node: &'a JsVal) -> String {
        match node {
            &JsVal::IntItem(ref inner) => Self::int(s, inner),
            &JsVal::StringValItem(ref inner) => Self::string_val(s, inner),
            &JsVal::ArrayValItem(ref inner) => Self::array_val(s, inner),
            &JsVal::JsObjectItem(ref inner) => Self::js_object(s, inner),
        }
    }

}