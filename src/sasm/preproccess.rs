use std::collections::HashMap;

use super::{lex::Lexemes, AddrType, SasmErrors};

pub fn preprocess(lexemes: Vec<Lexemes>) -> Result<Vec<Lexemes>, SasmErrors> {
    let mut label_map: HashMap<String, u64> = HashMap::new();
    let mut instr_count = 0;

    for lexeme in lexemes.iter() {
        match lexeme {
            Lexemes::DecLabel(name) => {
                label_map.insert(name.to_owned(), instr_count);
                ()
            }
            Lexemes::Op(_) => instr_count += 1,
            _ => {}
        }
    }

    let mut new_lexemes = Vec::new();
    for lexeme in lexemes.iter() {
        new_lexemes.push(match lexeme {
            Lexemes::DecLabel(_) => continue,
            Lexemes::Label(name) => {
                let references = label_map
                    .get(name)
                    .ok_or(SasmErrors::UseOfUndeclaredLabel(name.to_owned()))?;
                Lexemes::Arg(*references as AddrType)
            }
            lexeme => lexeme.to_owned(),
        })
    }

    Ok(new_lexemes)
}
