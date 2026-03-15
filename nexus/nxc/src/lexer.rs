use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    itr: Peekable<Chars<'a>>,
    cur: usize,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let itr = input.chars().peekable();
        Self {
            itr,
            cur: 0,
            current: None,
        }
    }

    fn consume(&mut self) {
        let next = self.itr.next();
        if let Some(c) = next {
            self.cur += c.len_utf8();
            self.current = Some(c);
        } else {
            self.current = None;
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.consume();
        self.current
    }
}
