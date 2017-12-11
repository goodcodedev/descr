use super::ast::*;

pub struct ToSource;
#[allow(unused_variables,dead_code)]
impl<'a> ToSource {
    pub fn token_group(mut s: String, node: &'a TokenGroup) -> String {
        s += " ";
        if node.not {
                s.push('!');
    }        s += " ";
        s.push('(');
        s += " ";
        let len = node.token_list.len();
        for (i, item) in node.token_list.iter().enumerate() {
            s = Self::token(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s.push(')');
        s += " ";
        if node.optional {
                s.push('?');
    }        s
    }

    pub fn ast_single(mut s: String, node: &'a AstSingle) -> String {
        s += " ";
        s += node.ident;
        s += " ";
        s.push('(');
        s += " ";
        let len = node.tokens.len();
        for (i, item) in node.tokens.iter().enumerate() {
            s = Self::token(s, item);
            if i < len - 1 {         s += " " }
        }
        s += " ";
        s.push(')');
        s
    }

    pub fn quoted(mut s: String, node: &'a Quoted) -> String {
        s += " ";
        s += "\"";
        s += node.string;
        s += "\"";
        s
    }

    pub fn simple_token(mut s: String, node: &'a SimpleToken) -> String {
        s += " ";
        if node.not {
                s.push('!');
    }        s += " ";
        s = Self::token_type(s, &node.token_type);
        s += " ";
        if node.optional {
                s.push('?');
    }        s
    }

    pub fn source(mut s: String, node: &'a Source) -> String {
        s += " ";
        let len = node.items.len();
        for (i, item) in node.items.iter().enumerate() {
            s = Self::source_item(s, item);
            if i < len - 1 {         s += " " }
        }
        s
    }

    pub fn key_token(mut s: String, node: &'a KeyToken) -> String {
        s += " ";
        s += node.key;
        s
    }

    pub fn named_token(mut s: String, node: &'a NamedToken) -> String {
        s += " ";
        s += node.name;
        s += " ";
        s.push(':');
        s += " ";
        if node.not {
                s.push('!');
    }        s += " ";
        s = Self::token_type(s, &node.token_type);
        s += " ";
        if node.optional {
                s.push('?');
    }        s
    }

    pub fn source_item(s: String, node: &'a SourceItem) -> String {
        match node {
            &SourceItem::AstSingleItem(ref inner) => Self::ast_single(s, inner),
        }
    }

    pub fn token(s: String, node: &'a Token) -> String {
        match node {
            &Token::NamedTokenItem(ref inner) => Self::named_token(s, inner),
            &Token::SimpleTokenItem(ref inner) => Self::simple_token(s, inner),
            &Token::TokenGroupItem(ref inner) => Self::token_group(s, inner),
        }
    }

    pub fn token_type(s: String, node: &'a TokenType) -> String {
        match node {
            &TokenType::KeyTokenItem(ref inner) => Self::key_token(s, inner),
            &TokenType::QuotedItem(ref inner) => Self::quoted(s, inner),
        }
    }

}