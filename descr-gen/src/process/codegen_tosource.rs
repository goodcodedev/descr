use lang_data::data::*;

pub struct CodegenToSource<'a, 'd: 'a> {
    data: &'a LangData<'d>
}
impl<'a, 'd> CodegenToSource<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) -> CodegenToSource<'a, 'd> {
        CodegenToSource { data }
    }

    pub fn gen(&self) -> String {
        let mut s = String::with_capacity(
            self.data.ast_data.len() * 100
            + self.data.list_data.len() * 100
        );
    }
}