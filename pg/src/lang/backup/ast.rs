#[derive(Debug)]
pub struct Comment<'a> {
    pub comment: &'a str,
}

#[derive(Debug)]
pub struct Random {
    pub num: i32,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub source_items: SourceItems<'a>,
}

#[derive(Debug)]
pub enum SourceItems<'a> {
    RandomItem(Random),
    CommentItem(Comment<'a>),
}

