#[derive(Debug)]
pub struct AstSingle<'a> {
    pub ident: &'a str,
    pub tokens: Vec<Token<'a>>,
}

impl<'a> AstSingle<'a> {
    pub fn new(ident: &'a str, tokens: Vec<Token<'a>>) -> AstSingle<'a> {
        AstSingle {
            ident,
            tokens
        }
    }

    pub fn as_source_item(self) -> SourceItem<'a> {
        SourceItem::AstSingleItem(self)
    }
}

#[derive(Debug)]
pub struct KeyToken<'a> {
    pub key: &'a str,
}

impl<'a> KeyToken<'a> {
    pub fn new(key: &'a str) -> KeyToken<'a> {
        KeyToken {
            key
        }
    }

    pub fn as_token_type(self) -> TokenType<'a> {
        TokenType::KeyTokenItem(self)
    }
}

#[derive(Debug)]
pub struct NamedToken<'a> {
    pub token_type: TokenType<'a>,
    pub name: &'a str,
    pub not: bool,
    pub optional: bool,
}

impl<'a> NamedToken<'a> {
    pub fn new(name: &'a str, not: bool, token_type: TokenType<'a>, optional: bool) -> NamedToken<'a> {
        NamedToken {
            name,
            not,
            token_type,
            optional
        }
    }

    pub fn as_token(self) -> Token<'a> {
        Token::NamedTokenItem(self)
    }
}

#[derive(Debug)]
pub struct Quoted<'a> {
    pub string: &'a str,
}

impl<'a> Quoted<'a> {
    pub fn new(string: &'a str) -> Quoted<'a> {
        Quoted {
            string
        }
    }

    pub fn as_token_type(self) -> TokenType<'a> {
        TokenType::QuotedItem(self)
    }
}

#[derive(Debug)]
pub struct SimpleToken<'a> {
    pub token_type: TokenType<'a>,
    pub not: bool,
    pub optional: bool,
}

impl<'a> SimpleToken<'a> {
    pub fn new(not: bool, token_type: TokenType<'a>, optional: bool) -> SimpleToken<'a> {
        SimpleToken {
            not,
            token_type,
            optional
        }
    }

    pub fn as_token(self) -> Token<'a> {
        Token::SimpleTokenItem(self)
    }
}

#[derive(Debug)]
pub struct Source<'a> {
    pub items: Vec<SourceItem<'a>>,
}

impl<'a> Source<'a> {
    pub fn new(items: Vec<SourceItem<'a>>) -> Source<'a> {
        Source {
            items
        }
    }
}

#[derive(Debug)]
pub struct TokenGroup<'a> {
    pub not: bool,
    pub optional: bool,
    pub token_list: Vec<Token<'a>>,
}

impl<'a> TokenGroup<'a> {
    pub fn new(not: bool, token_list: Vec<Token<'a>>, optional: bool) -> TokenGroup<'a> {
        TokenGroup {
            not,
            token_list,
            optional
        }
    }

    pub fn as_token(self) -> Token<'a> {
        Token::TokenGroupItem(self)
    }
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    AstSingleItem(AstSingle<'a>),
}

impl<'a> SourceItem<'a> {
    pub fn ast_single(ident: &'a str, tokens: Vec<Token<'a>>) -> SourceItem<'a> {
        SourceItem::AstSingleItem(AstSingle::new(ident, tokens))
    }
}

#[derive(Debug)]
pub enum Token<'a> {
    NamedTokenItem(NamedToken<'a>),
    SimpleTokenItem(SimpleToken<'a>),
    TokenGroupItem(TokenGroup<'a>),
}

impl<'a> Token<'a> {
    pub fn named_token(name: &'a str, not: bool, token_type: TokenType<'a>, optional: bool) -> Token<'a> {
        Token::NamedTokenItem(NamedToken::new(name, not, token_type, optional))
    }

    pub fn simple_token(not: bool, token_type: TokenType<'a>, optional: bool) -> Token<'a> {
        Token::SimpleTokenItem(SimpleToken::new(not, token_type, optional))
    }

    pub fn token_group(not: bool, token_list: Vec<Token<'a>>, optional: bool) -> Token<'a> {
        Token::TokenGroupItem(TokenGroup::new(not, token_list, optional))
    }
}

#[derive(Debug)]
pub enum TokenType<'a> {
    KeyTokenItem(KeyToken<'a>),
    QuotedItem(Quoted<'a>),
}

impl<'a> TokenType<'a> {
    pub fn key_token(key: &'a str) -> TokenType<'a> {
        TokenType::KeyTokenItem(KeyToken::new(key))
    }

    pub fn quoted(string: &'a str) -> TokenType<'a> {
        TokenType::QuotedItem(Quoted::new(string))
    }
}

