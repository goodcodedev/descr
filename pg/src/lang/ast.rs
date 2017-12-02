#[derive(Debug)]
pub struct AstName<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct Container<'a> {
    pub ast_name: AstName<'a>,
    pub ident: &'a str,
}

