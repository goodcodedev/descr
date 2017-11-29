use lang_data::data::*;
use lang_data::*;
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

    fn visit_token_named_key(&mut self, node: &'d TokenNamedKey) {
        self.data.resolve_typed_part(node.key);
    }

    fn visit_token_key(&mut self, node: &'d TokenKey) {
        self.data.resolve_typed_part(node.ident);
    }

    fn visit_list_single(&mut self, node: &'d ListSingle) {
        self.data.resolve_typed_part(node.sep);
    }

    fn visit_list_many(&mut self, node: &'d ListMany) {
        match node.sep {
            Some(sep) => self.data.resolve_typed_part(sep),
            None => {}
        };
    }

}