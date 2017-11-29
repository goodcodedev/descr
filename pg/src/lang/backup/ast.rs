#[derive(Debug)]
pub struct First<'a> {
    pub second: Second<'a>,
}

#[derive(Debug)]
pub struct Second<'a> {
    pub ident: &'a str,
}

