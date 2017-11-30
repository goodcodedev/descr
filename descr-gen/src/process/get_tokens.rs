use lang_data::data::*;
use descr_lang::gen::ast::*;
use descr_lang::gen::visitor::Visitor;

pub struct GetTokens<'a, 'd: 'a> {
    data: &'a mut LangData<'d>
}

impl<'a, 'd> GetTokens<'a, 'd> {
    pub fn new(data: &'a mut LangData<'d>) -> GetTokens<'a, 'd> {
        GetTokens {
            data
        }
    }
}

impl<'a, 'd> Visitor<'d> for GetTokens<'a, 'd> {

    fn visit_key_token(&mut self, node: &'d KeyToken) {
        self.data.resolve_typed_part(node.key);
    }

    fn visit_list_single(&mut self, node: &'d ListSingle) {
        self.data.resolve_typed_part(node.sep);
    }

    fn visit_list_many(&mut self, node: &'d ListMany) {
        match node.sep {
            Some(sep) => self.data.resolve_typed_part(sep),
            None => {}
        };
        for item in &node.items {
            self.visit_list_item(item);
        }
    }

}