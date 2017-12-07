#[derive(Debug)]
pub struct Plus<'a> {
    pub op1: Expr<'a>,
    pub op2: Expr<'a>,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub exprs: Vec<Expr<'a>>,
}

#[derive(Debug)]
pub struct VarName<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub enum Expr<'a> {
    VarNameItem(VarName<'a>),
    PlusItem(Box<Plus<'a>>),
}

