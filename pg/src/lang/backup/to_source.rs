use super::ast::*;

pub struct ToSource;
impl<'a> ToSource {
    pub fn to_source_test_item(mut s: String, node: &'a TestItem) -> String {
        s += " ";
        s += node.ident;
        s += " ";
        s += "val1";
        s += "val2";
        s
    }

    pub fn to_source_source(mut s: String, node: &'a Source) -> String {
        s += " ";
        for item in &node.items {
            s = Self::to_source_source_item(s, item);
        }
        s
    }

    pub fn to_source_source_item(s: String, node: &'a SourceItem) -> String {
        match node {
            &SourceItem::TestItemItem(ref inner) => Self::to_source_test_item(s, inner),
        }
    }

}