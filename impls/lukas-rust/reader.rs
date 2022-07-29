use crate::types::MalType;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Reader {
    pub tokens: Vec<String>,
    pub cursor: usize,
}

fn next(reader: &Reader) -> (Option<String>, Reader) {
    let token = peek(&reader);
    let next_reader = Reader {
        tokens: reader.tokens.clone(),
        cursor: reader.cursor + 1,
    };

    (token, next_reader)
}

fn peek(reader: &Reader) -> Option<String> {
    reader.tokens.get(reader.cursor).map(|x| x.to_owned())
}

pub fn read_str(str: &str) -> Reader {
    let reg =
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap();

    let tokens: Vec<String> = reg
        .find_iter(str)
        .map(|x| x.as_str().replace(",", " ").trim().to_string())
        .collect();

    Reader { tokens, cursor: 0 }
}

pub fn read_form(reader: &Reader) -> (MalType, Reader) {
    let (token, next_reader) = next(reader);

    match token {
        Some(token) => match token.as_str() {
            "(" => read_list(&next_reader),
            _ => read_atom(reader),
        },
        None => panic!("no next token"),
    }
}

fn read_list(initial_reader: &Reader) -> (MalType, Reader) {
    let mut acc: Vec<MalType> = Vec::new();
    let mut loop_reader: Reader = initial_reader.clone();

    loop {
        let (token, next_reader) = next(&loop_reader);

        match token {
            Some(token) => match token.as_str() {
                ")" => break (MalType::MalList(acc), next_reader),
                _ => {
                    let (form, reader_after_form_read) = read_form(&loop_reader);
                    acc.push(form);
                    loop_reader = reader_after_form_read;
                }
            },
            None => panic!("No more tokens to loop trough."),
        }
    }
}

fn read_atom(reader: &Reader) -> (MalType, Reader) {
    let (token, next_reader) = next(reader);

    match token {
        Some(token) => (MalType::MalAtom(token), next_reader),
        None => panic!("no atom?"),
    }
}
