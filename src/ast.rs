#[derive(Debug)]
pub struct Source<'a> {
    pub nodes: Vec<SourceNode<'a>>
}

#[derive(Debug)]
pub enum SourceNode<'a> {
    AstSingle(AstSingle<'a>),
    AstMany(AstMany<'a>),
    List(List<'a>),
}

#[derive(Debug)]
pub struct AstSingle<'a> {
    pub ident: &'a str,
    pub token_list: Vec<TokenNode<'a>>
}

impl<'a> AstSingle<'a> {
    pub fn new(ident: &'a str) -> AstSingle<'a> {
        AstSingle {
            ident,
            token_list: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct AstMany<'a> {
    pub ident: &'a str,
    pub ast_items: Vec<AstItem<'a>>
}

#[derive(Debug)]
pub struct List<'a> {
    pub ident: &'a str,
    pub sep: Option<&'a str>,
    pub items: Vec<ListItem<'a>>
}

#[derive(Debug)]
pub struct ListItem<'a> {
    pub ast_item: AstItem<'a>,
    pub sep: Option<&'a str>
}

#[derive(Debug)]
pub enum AstItem<'a> {
    AstDef(AstDef<'a>),
    AstRef(AstRef<'a>)
}

#[derive(Debug)]
pub struct AstDef<'a> {
    pub ident: Option<&'a str>,
    pub token_list: Vec<TokenNode<'a>>
}

#[derive(Debug)]
pub struct AstRef<'a> {
    pub ident: &'a str
}

#[derive(Debug)]
pub enum TokenNode<'a> {
    TokenKey(TokenKey<'a>),
    TokenNamedKey(TokenNamedKey<'a>)
}

#[derive(Debug)]
pub struct TokenKey<'a> {
    pub ident: &'a str,
    pub optional: bool
}

#[derive(Debug)]
pub struct TokenNamedKey<'a> {
    pub name: &'a str,
    pub key: &'a str,
    pub optional: bool
}