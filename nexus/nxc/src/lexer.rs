use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Fn,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Punct {
    LParen, // (
    RParen, // )
    LBrace, // {
    RBrace, // }
    Colon,  // :
    Comma,  // ,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Ident(String),
    Punct(Punct),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    input: &'a str,
    itr: Peekable<Chars<'a>>,
    cur: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            itr: input.chars().peekable(),
            cur: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.itr.next()?;
        self.cur += c.len_utf8();
        Some(c)
    }

    fn peek(&mut self) -> Option<char> {
        self.itr.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    fn read_ident(&mut self, first: char, start: usize) -> Token {
        let mut s = String::from(first);
        while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
            s.push(self.advance().unwrap());
        }
        let end = self.cur;
        let kind = match s.as_str() {
            "fn" => TokenKind::Keyword(Keyword::Fn),
            _ => TokenKind::Ident(s),
        };
        Token {
            kind,
            span: Span { start, end },
        }
    }

    fn single_punct(c: char) -> Option<Punct> {
        match c {
            '(' => Some(Punct::LParen),
            ')' => Some(Punct::RParen),
            '{' => Some(Punct::LBrace),
            '}' => Some(Punct::RBrace),
            ':' => Some(Punct::Colon),
            ',' => Some(Punct::Comma),
            _ => None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let start = self.cur;

        let Some(c) = self.advance() else {
            return Token {
                kind: TokenKind::Eof,
                span: Span { start, end: start },
            };
        };

        if c.is_alphabetic() || c == '_' {
            return self.read_ident(c, start);
        }

        if let Some(p) = Self::single_punct(c) {
            return Token {
                kind: TokenKind::Punct(p),
                span: Span {
                    start,
                    end: self.cur,
                },
            };
        }

        self.next_token()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        Lexer::new(src)
            .tokenize()
            .into_iter()
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn test_fn_no_args_no_ret() {
        let k = kinds("fn foo() bar {}");
        assert_eq!(
            k,
            vec![
                TokenKind::Keyword(Keyword::Fn),
                TokenKind::Ident("foo".into()),
                TokenKind::Punct(Punct::LParen),
                TokenKind::Punct(Punct::RParen),
                TokenKind::Ident("bar".into()),
                TokenKind::Punct(Punct::LBrace),
                TokenKind::Punct(Punct::RBrace),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_fn_multi_args() {
        let k = kinds("fn add(a: i32, b: i32) i32 {}");
        assert_eq!(
            k,
            vec![
                TokenKind::Keyword(Keyword::Fn),
                TokenKind::Ident("add".into()),
                TokenKind::Punct(Punct::LParen),
                TokenKind::Ident("a".into()),
                TokenKind::Punct(Punct::Colon),
                TokenKind::Ident("i32".into()),
                TokenKind::Punct(Punct::Comma),
                TokenKind::Ident("b".into()),
                TokenKind::Punct(Punct::Colon),
                TokenKind::Ident("i32".into()),
                TokenKind::Punct(Punct::RParen),
                TokenKind::Ident("i32".into()),
                TokenKind::Punct(Punct::LBrace),
                TokenKind::Punct(Punct::RBrace),
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_span() {
        let mut lex = Lexer::new("fn f()");
        let t = lex.next_token();
        assert_eq!(t.span, Span { start: 0, end: 2 });
    }
}
