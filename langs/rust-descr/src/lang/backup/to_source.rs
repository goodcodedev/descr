use super::ast::*;

pub struct ToSource;
impl<'a> ToSource {
    pub fn to_source_source(mut s: String, node: &'a Source) -> String {
        s += " ";
        for item in &node.source_items {
            s = Self::to_source_source_item(s, item);
        }
        s
    }

    pub fn to_source_rs_enum(mut s: String, node: &'a RsEnum) -> String {
        s += " ";
        if node.public { s += "pub"; }
        s += " ";
        s += "enum";
        s += " ";
        s += node.ident;
        s += " ";
        s.push('{');
        s += " ";
        for item in &node.enum_items {
            s = Self::to_source_enum_item(s, item);
        }
        s += " ";
        s.push('}');
        s
    }

    pub fn to_source_struct_member(mut s: String, node: &'a StructMember) -> String {
        s += " ";
        if node.public { s += "pub"; }
        s += " ";
        s += node.ident;
        s += " ";
        s.push(':');
        s += " ";
        s = Self::to_source_tpe_spes(s, &node.tpe_spes);
        s
    }

    pub fn to_source_enum_item(mut s: String, node: &'a EnumItem) -> String {
        s += " ";
        s += node.ident;
        s
    }

    pub fn to_source_tpe_spes(mut s: String, node: &'a TpeSpes) -> String {
        s += " ";
        s = Self::to_source_tpe(s, &node.tpe);
        s += " ";
        if let Some(ref some_val) = node.generic_item {
            s = Self::to_source_generic_item(s, some_val);
        }
        s
    }

    pub fn to_source_life_time(mut s: String, node: &'a LifeTime) -> String {
        s += " ";
        s += "'";
        s += " ";
        s += node.ident;
        s
    }

    pub fn to_source_gen_type(mut s: String, node: &'a GenType) -> String {
        s += " ";
        s += node.ident;
        s
    }

    pub fn to_source_rs_struct(mut s: String, node: &'a RsStruct) -> String {
        s += " ";
        if node.public { s += "pub"; }
        s += " ";
        s += "struct";
        s += " ";
        s += node.ident;
        s += " ";
        if let Some(ref some_val) = node.generic {
            s = Self::to_source_generic(s, some_val);
        }
        s += " ";
        s.push('{');
        s += " ";
        for item in &node.struct_members {
            s = Self::to_source_struct_member(s, item);
        }
        s += " ";
        s.push('}');
        s
    }

    pub fn to_source_generic(mut s: String, node: &'a Generic) -> String {
        s += " ";
        s += "<";
        s += " ";
        for item in &node.generic_items {
            s = Self::to_source_generic_item(s, item);
        }
        s += " ";
        s += ">";
        s
    }

    pub fn to_source_generic_item(s: String, node: &'a GenericItem) -> String {
        match node {
            &GenericItem::LifeTimeItem(ref inner) => Self::to_source_life_time(s, inner),
            &GenericItem::GenTypeItem(ref inner) => Self::to_source_gen_type(s, inner),
        }
    }

    pub fn to_source_source_item(s: String, node: &'a SourceItem) -> String {
        match node {
            &SourceItem::RsStructItem(ref inner) => Self::to_source_rs_struct(s, inner),
            &SourceItem::RsEnumItem(ref inner) => Self::to_source_rs_enum(s, inner),
        }
    }

    pub fn to_source_tpe(mut s: String, node: &'a Tpe) -> String {
        match node {
            &Tpe::RsU32 => {
                s += " ";
        s += "u32";
            },
            &Tpe::RsI32 => {
                s += " ";
        s += "i32";
            },
            &Tpe::RsString => {
                s += " ";
        s += "String";
            },
            &Tpe::RsStr => {
                s += " ";
        s += "str";
            },
            &Tpe::Bool => {
                s += " ";
        s += "bool";
            },
        }
        s
    }

}