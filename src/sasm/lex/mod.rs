mod perform_lex;
use perform_lex::*;

use super::{AddrType, SasmErrors};

// Langauge syntax:
// IDN arg? ['idn]? [; | [; *]]?

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lexemes {
    Op(String),
    Arg(AddrType),
    Label(String),
    DecLabel(String),
    Newline,
}

pub struct LexingBuffer {
    pub contents: String,
    tokens: Vec<Lexemes>,
}

impl LexingBuffer {
    fn new(contents: String) -> Self {
        Self {
            contents,
            tokens: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    fn first(&self) -> char {
        self.contents
            .chars()
            .collect::<Vec<char>>()
            .first()
            .unwrap_or(&' ')
            .clone()
    }

    fn tokens_mut(&mut self) -> &mut Vec<Lexemes> {
        &mut self.tokens
    }

    fn trim(&mut self, idx: usize) -> String {
        let retval = self.contents[..idx].to_string();
        self.contents = self.contents[idx..].to_string();
        retval
    }

    fn clear(&mut self) {
        self.contents.clear();
    }
}

pub fn lex(contents: String) -> Result<Vec<Lexemes>, SasmErrors> {
    let mut lb = LexingBuffer::new(contents);
    let mut count = 0;
    let mut last_len = 0;

    while !lb.is_empty() {
        count += 1;
        if lb.tokens.len() != last_len {
            last_len = lb.tokens.len();
            count = 0;
        }

        if count >= 100 { // If 100 cycles have gone by without any token being lexed return error
            return Err(SasmErrors::LexemeNotRecognised);
        }

        clean_newline(&mut lb);
        clean_whitespace(&mut lb);
        clean_eof(&mut lb);
        clean_comments(&mut lb);

        if lex_op(&mut lb) {
            continue;
        }
        if lex_arg(&mut lb) {
            continue;
        }
        if lex_label(&mut lb) {
            continue;
        }
        if lex_dec_label(&mut lb) {
            continue;
        }
    }

    Ok(lb.tokens_mut().to_owned())
}
