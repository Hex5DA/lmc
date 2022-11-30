use super::{Lexemes::*, LexingBuffer};

pub fn lex_idn(lb: &mut LexingBuffer) -> Option<String> {
    if !lb.first().is_ascii_alphabetic() {
        return None;
    }

    for (idx, ch) in lb.contents().chars().enumerate() {
        if !ch.is_ascii_alphabetic() {
            return Some(lb.trim(idx));
        }
    }

    None
}

pub fn lex_op(lb: &mut LexingBuffer) -> bool {
    if let Some(idn) = lex_idn(lb) {
        lb.tokens_mut().push(OP(idn));
        true
    } else {
        false
    }
}

pub fn lex_arg(lb: &mut LexingBuffer) -> bool {
    if !lb.first().is_ascii_digit() {
        return false;
    }

    for (idx, ch) in lb.contents().chars().enumerate() {
        if !ch.is_ascii_digit() {
            let num = lb.trim(idx);
            lb.tokens_mut().push(ARG(num.parse::<i64>().unwrap()));
            return true;
        }
    }

    false
}

pub fn lex_label(lb: &mut LexingBuffer) -> bool {
    if lb.first() != '\'' {
        return false;
    }

    if let Some(idn) = lex_idn(lb) {
        lb.tokens_mut().push(LABEL(idn));
        true
    } else {
        false
    }
}

pub fn clean_whitespace(lb: &mut LexingBuffer) -> bool {
    for (idx, ch) in lb.contents().chars().enumerate() {
        if !ch.is_whitespace() {
            let _ = lb.trim(idx);
            return true;
        }
    }

    false
}

pub fn clean_newline(lb: &mut LexingBuffer) -> bool {
    if lb.first() == '\n' {
        lb.trim(1);
        lb.tokens_mut().push(NEWLINE)
    }

    false
}

pub fn clean_eof(lb: &mut LexingBuffer) -> bool {
    if lb.contents() == "\x04" {
        lb.clear();
        return true;
    }
    false
}
