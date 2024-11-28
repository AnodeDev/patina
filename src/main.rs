use std::fs::File;
use std::io::prelude::*;

enum TokenKind {
    Invalid,
}

struct Token {
    kind: TokenKind,
    value: String,
}

struct Lexer<'a> {
    content: Vec<&'a str>,
    parens_balance: i8,
}

impl<'a> Lexer<'a> {
    fn new(content: Vec<&'a str>) -> Self {
        Lexer {
            content,
            parens_balance: 0,
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("")?;
    let mut content_str = String::new();
    file.read_to_string(&mut content_str)?;

    let content: Vec<&str> = content_str.split("/n").collect();

    let lexer = Lexer::new(content);

    Ok(())
}
