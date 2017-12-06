use super::ast::*;

pub trait Visitor<'a> {
    fn visit_enum_item(&mut self, node: &'a EnumItem) {
    }

    fn visit_gen_type(&mut self, node: &'a GenType) {
    }

    fn visit_generic(&mut self, node: &'a Generic) {
        for item in &node.generic_items {
            self.visit_generic_item(item);
        }
    }

    fn visit_life_time(&mut self, node: &'a LifeTime) {
    }

    fn visit_rs_enum(&mut self, node: &'a RsEnum) {
        for item in &node.enum_items {
            self.visit_enum_item(item);
        }
    }

    fn visit_rs_struct(&mut self, node: &'a RsStruct) {
        match node.generic {
            Some(ref inner) => self.visit_generic(inner),
            None => {}
        }
        for item in &node.struct_members {
            self.visit_struct_member(item);
        }
    }

    fn visit_source(&mut self, node: &'a Source) {
        for item in &node.source_items {
            self.visit_source_item(item);
        }
    }

    fn visit_struct_member(&mut self, node: &'a StructMember) {
        self.visit_tpe_spes(&node.tpe_spes);
    }

    fn visit_tpe_spes(&mut self, node: &'a TpeSpes) {
        match node.generic_item {
            Some(ref inner) => self.visit_generic_item(inner),
            None => {}
        }
        self.visit_tpe(&node.tpe);
    }

    fn visit_generic_item(&mut self, node: &'a GenericItem) {
        match node {
            &GenericItem::LifeTimeItem(ref inner) => self.visit_life_time(inner),
            &GenericItem::GenTypeItem(ref inner) => self.visit_gen_type(inner),
        }
    }

    fn visit_source_item(&mut self, node: &'a SourceItem) {
        match node {
            &SourceItem::RsStructItem(ref inner) => self.visit_rs_struct(inner),
            &SourceItem::RsEnumItem(ref inner) => self.visit_rs_enum(inner),
        }
    }

    fn visit_tpe(&mut self, node: &'a Tpe) {
    }

}