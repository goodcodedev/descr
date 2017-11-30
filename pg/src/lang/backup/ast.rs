#[derive(Debug)]
pub struct Comment<'a> {
    pub comment: &'a str,
}

#[derive(Debug)]
pub struct Random<'a> {
    pub string: &'a str,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub items: Vec<SourceItem<'a>>,
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    RandomItem(Random<'a>),
    CommentItem(Comment<'a>),
}

