use lang_data::data::*;
use visit_ast::*;
use lang_data::*;
use ast::*;

pub struct RegisterKeys<'a, 'd: 'a> {
    data: &'a mut LangData<'d>
}

impl<'a, 'd> RegisterKeys<'a, 'd> {
    pub fn new(data: &'a mut LangData<'d>) -> RegisterKeys<'a, 'd> {
        RegisterKeys {
            data
        }
    }
}
impl<'a, 'd> VisitAst<'a, 'd> for RegisterKeys<'a, 'd> {

    fn visit_ast_single(&mut self, node: &'d AstSingle) {
        self.data.ast_data.insert(
            node.ident, 
            AstData::new(node.ident, node.ident)
        );
    }

    fn visit_ast_many(&mut self, node: &'d AstMany) {
        self.data.ast_data.insert(
            node.ident,
            AstData::new(node.ident, node.ident)
        );
    }

    fn visit_list(&mut self, node: &'d List) {
        self.data.list_data.insert(
            node.ident,
            ListData::new(node.ident, node.sep)
        );
    }
}