#[derive(Debug)]
pub struct Source<'a> {
    pub items: Vec<SourceItem<'a>>,
}

#[derive(Debug)]
pub struct TestItem<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    TestItemItem(TestItem<'a>),
}

