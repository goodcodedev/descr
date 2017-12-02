#[derive(Debug)]
pub struct BgColor {
    pub color: Color,
}

#[derive(Debug)]
pub struct Say<'a> {
    pub string: &'a str,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub enum Statement<'a> {
    SayItem(Say<'a>),
    BgColorItem(BgColor),
}

