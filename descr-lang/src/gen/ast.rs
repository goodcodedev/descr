#[derive(Debug)]
pub struct AnnotArg<'a> {
    pub annot_arg_val: AnnotArgVal<'a>,
    pub key: &'a str,
}

#[derive(Debug)]
pub struct AnnotArgs<'a> {
    pub annot_arg_list: Vec<AnnotArg<'a>>,
}

#[derive(Debug)]
pub struct Annotation<'a> {
    pub annot_args: Option<AnnotArgs<'a>>,
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct AstDef<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ident: Option<&'a str>,
    pub tokens: Vec<Token<'a>>,
}

#[derive(Debug)]
pub struct AstMany<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ident: &'a str,
    pub items: Vec<AstItem<'a>>,
}

#[derive(Debug)]
pub struct AstRef<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct AstSingle<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ident: &'a str,
    pub tokens: Vec<Token<'a>>,
}

#[derive(Debug)]
pub struct Comment<'a> {
    pub comment: &'a str,
}

#[derive(Debug)]
pub struct FuncToken<'a> {
    pub fn_args: Vec<FuncArg<'a>>,
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct Ident<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct IntConst {
    pub int: u32,
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
    pub annots: Vec<Annotation<'a>>,
    pub ast_type: &'a str,
    pub ident: &'a str,
    pub items: Vec<ListItem<'a>>,
    pub sep: Option<&'a str>,
}

#[derive(Debug)]
pub struct ListSingle<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub ident: &'a str,
    pub reference: &'a str,
    pub sep: &'a str,
}

#[derive(Debug)]
pub struct NamedToken<'a> {
    pub token_type: TokenType<'a>,
    pub annots: Vec<Annotation<'a>>,
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
    pub annots: Vec<Annotation<'a>>,
    pub not: bool,
    pub optional: bool,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub items: Vec<SourceItem<'a>>,
}

#[derive(Debug)]
pub struct TokenGroup<'a> {
    pub annots: Vec<Annotation<'a>>,
    pub not: bool,
    pub optional: bool,
    pub token_list: Vec<Token<'a>>,
}

#[derive(Debug)]
pub enum AnnotArgVal<'a> {
    QuotedItem(Quoted<'a>),
    IdentItem(Ident<'a>),
    IntConstItem(IntConst),
}

#[derive(Debug)]
pub enum AstItem<'a> {
    AstDefItem(AstDef<'a>),
    AstRefItem(AstRef<'a>),
}

#[derive(Debug)]
pub enum FuncArg<'a> {
    QuotedItem(Quoted<'a>),
}

#[derive(Debug)]
pub enum List<'a> {
    ListSingleItem(ListSingle<'a>),
    ListManyItem(ListMany<'a>),
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    AstSingleItem(AstSingle<'a>),
    AstManyItem(AstMany<'a>),
    ListItem(List<'a>),
    CommentItem(Comment<'a>),
}

#[derive(Debug)]
pub enum Token<'a> {
    NamedTokenItem(NamedToken<'a>),
    SimpleTokenItem(SimpleToken<'a>),
    TokenGroupItem(TokenGroup<'a>),
}

#[derive(Debug)]
pub enum TokenType<'a> {
    FuncTokenItem(FuncToken<'a>),
    KeyTokenItem(KeyToken<'a>),
    QuotedItem(Quoted<'a>),
}

