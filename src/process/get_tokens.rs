use lang_data::data::*;
use visit_ast::*;
use lang_data::*;
use ast::*;

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

impl<'a, 'd> VisitAst<'a, 'd> for GetTokens<'a, 'd> {

    fn visit_token_named_key(&mut self, node: &'d TokenNamedKey) {
        self.data.resolve_typed_part(node.key);
    }

    fn visit_token_key(&mut self, node: &'d TokenKey) {
        self.data.resolve_typed_part(node.ident);
    }

}