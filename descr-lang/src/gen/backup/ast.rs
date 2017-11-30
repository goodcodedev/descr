#[derive(Debug)]
pub struct AstDef<'a> {
    pub ident: Option<&'a str>,
    pub tokens: Vec<Token<'a>>,
}

#[derive(Debug)]
pub struct AstMany<'a> {
    pub ident: &'a str,
    pub items: Vec<AstItem<'a>>,
}

#[derive(Debug)]
pub struct AstRef<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct AstSingle<'a> {
    pub ident: &'a str,
    pub tokens: Vec<Token<'a>>,
}

#[derive(Debug)]
pub struct Comment {
}

#[derive(Debug)]
pub struct KeyToken<'a> {
    pub key: &'a str,
}

#[derive(Debug)]
pub struct ListItem<'a> {
    pub ast_item: AstItem<'a>,
    pub sep: Option<&'a str>,
}

#[derive(Debug)]
pub struct ListMany<'a> {
    pub ast_type: &'a str,
    pub ident: &'a str,
    pub items: Vec<ListItem<'a>>,
    pub sep: Option<&'a str>,
}

#[derive(Debug)]
pub struct ListSingle<'a> {
    pub ident: &'a str,
    pub reference: &'a str,
    pub sep: &'a str,
}

#[derive(Debug)]
pub struct NamedToken<'a> {
    pub token_type: TokenType<'a>,
    pub name: &'a str,
    pub not: bool,
    pub optional: bool,
}

#[derive(Debug)]
pub struct Quoted<'a> {
    pub string: &'a str,
}

#[derive(Debug)]
pub struct SimpleToken<'a> {
    pub token_type: TokenType<'a>,
    pub not: bool,
    pub optional: bool,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub items: Vec<SourceItem<'a>>,
}

#[derive(Debug)]
pub enum AstItem<'a> {
    AstDefItem(AstDef<'a>),
    AstRefItem(AstRef<'a>),
}

#[derive(Debug)]
pub enum List<'a> {
    ListSingleItem(ListSingle<'a>),
    ListManyItem(ListMany<'a>),
}

#[derive(Debug)]
pub enum Token<'a> {
    NamedTokenItem(NamedToken<'a>),
    SimpleTokenItem(SimpleToken<'a>),
}

#[derive(Debug)]
pub enum TokenType<'a> {
    KeyTokenItem(KeyToken<'a>),
    QuotedItem(Quoted<'a>),
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    AstSingleItem(AstSingle<'a>),
    AstManyItem(AstMany<'a>),
    ListItem(List<'a>),
    CommentItem(Comment),
}

