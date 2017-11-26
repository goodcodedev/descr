use lang_data::*;

pub struct CodegenVisitor<'a, 'd> {
    pub data: &'a LangData<'d>
}
impl<'a, 'd> CodegenVisitor<'a, 'd> {
    pub fn new(data: &'a LangData<'d>) {
        CodegenVisitor { data }
    }

}