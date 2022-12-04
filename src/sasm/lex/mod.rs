mod perform_lex;
use perform_lex::*;

use super::{AddrType, SasmErrors};

// Langauge syntax:
// IDN arg? ['idn]? [; | [; *]]?

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lexemes {
    OP(String),
    ARG(AddrType),
    LABEL(String),
    NEWLINE,
}

pub struct LexingBuffer(String, Vec<Lexemes>, u64);
impl LexingBuffer {
    fn new(str: String) -> Self {
        Self {
            0: str,
            1: Vec::new(),
            2: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn first(&self) -> char {
        self.0
            .chars()
            .collect::<Vec<char>>()
            .first()
            .unwrap_or(&' ')
            .clone()
    }

    fn tokens_mut(&mut self) -> &mut Vec<Lexemes> {
        &mut self.1
    }

    fn contents(&self) -> &String {
        &self.0
    }

    fn trim(&mut self, idx: usize) -> String {
        let retval = self.0[..idx].to_string();
        self.0 = self.0[idx..].to_string();
        retval
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

pub fn lex(contents: String) -> Result<Vec<Lexemes>, SasmErrors> {
    let mut lb = LexingBuffer::new(contents);

    while !lb.is_empty() {
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
    }

    Ok(lb.tokens_mut().to_owned())
}
