use lang_data::data::*;
use lang_data::annotations::*;
use descr_lang::gen::ast::*;
use descr_lang::gen::visitor::Visitor;

pub struct RegisterKeys<'a, 'd: 'a> {
    data: &'a mut LangData<'d>,
}

impl<'a, 'd> RegisterKeys<'a, 'd> {
    pub fn new(data: &'a mut LangData<'d>) -> RegisterKeys<'a, 'd> {
        RegisterKeys { data }
    }
    pub fn check_start(&mut self, start_key: &'d str) {
        if self.data.start_key.is_none() {
            self.data.start_key = Some(start_key);
        }
    }
}
impl<'a, 'd> Visitor<'d> for RegisterKeys<'a, 'd> {
    fn visit_ast_single(&mut self, node: &'d AstSingle) {
        self.check_start(node.ident);
        self.data
            .ast_data
            .insert(node.ident, AstData::new(node.ident, node.ident, parse_annots(&node.annots)));
    }

    fn visit_ast_many(&mut self, node: &'d AstMany) {
        self.check_start(node.ident);
        self.data
            .ast_data
            .insert(node.ident, AstData::new(node.ident, node.ident, parse_annots(&node.annots)));
    }

    fn visit_list_single(&mut self, node: &'d ListSingle) {
        self.check_start(node.ident);
        self.data
            .list_data
            .insert(node.ident, ListData::new(node.ident, None, Some(node.sep), parse_annots(&node.annots)));
    }

    fn visit_list_many(&mut self, node: &'d ListMany) {
        self.check_start(node.ident);
        self.data.list_data.insert(
            node.ident,
            ListData::new(node.ident, Some(node.ast_type), node.sep, parse_annots(&node.annots)),
        );
    }
}
