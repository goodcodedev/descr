pub struct TokenKey {
    pub QUESTION: bool,
    pub ident: String,
 }

pub struct AstRef {
    pub ident: String,
 }

pub struct AstDef {
    pub RPAREN: bool,
    pub ident: String,
    pub LPAREN: bool,
    pub tokenList: tokenList,
 }

pub struct AstSingle {
    pub ident: String,
    pub LPAREN: bool,
    pub RPAREN: bool,
    pub tokenList: tokenList,
 }

pub struct TokenNamedKey {
    pub COLON: bool,
    pub QUESTION: bool,
    pub ident: String,
 }

pub struct AstMany {
    pub LBRACE: bool,
    pub RBRACE: bool,
    pub ident: String,
    pub astItems: astItems,
 }

pub enum Source {
    AstSingleItem(AstSingle),
    AstManyItem(AstMany),
    ListItem(List),
}

pub enum Token {
    TokenKeyItem(TokenKey),
    TokenNamedKeyItem(TokenNamedKey),
}

pub enum AstItem {
    AstDefItem(AstDef),
    AstRefItem(AstRef),
}

