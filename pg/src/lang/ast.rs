#[derive(Debug)]
pub struct Source<'a> {
    pub some_list: Vec<Something<'a>>,
}

#[derive(Debug)]
pub struct Str2Item<'a> {
    pub string: &'a str,
}

#[derive(Debug)]
pub struct StrItem<'a> {
    pub string: &'a str,
}

#[derive(Debug)]
pub enum Something<'a> {
    StrItemItem(StrItem<'a>),
    Str2ItemItem(Str2Item<'a>),
}

