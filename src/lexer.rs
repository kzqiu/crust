use regex::Regex;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    SEMICOLON,
    INTEGER,
    RETURN,
    IDENTIFIER,
    CONST,
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub start: u64,
    pub end: u64,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} => {}, Start: {}, End: {}",
            self.text,
            match self.token_type {
                TokenType::LBRACE => "LBRACE",
                TokenType::RBRACE => "RBRACE",
                TokenType::LPAREN => "LPAREN",
                TokenType::RPAREN => "RPAREN",
                TokenType::SEMICOLON => "SEMICOLON",
                TokenType::INTEGER => "INTEGER",
                TokenType::RETURN => "RETURN",
                TokenType::IDENTIFIER => "IDENTIFIER",
                TokenType::CONST => "CONST",
            },
            self.start,
            self.end
        )
    }
}

pub fn lex(file: &str) -> Vec<Token> {
    let patterns = [
        r"\{",          // LBRACE
        r"\}",          // RBRACE
        r"\(",          // LPAREN
        r"\)",          // RPAREN
        r";",           // SEMICOLON
        r"int",         // INTEGER
        r"return",      // RETURN
        r"[a-zA-Z]\w+", // IDENTIFIER
        r"[0-9]+",      // CONST
    ];

    let mut token_starts: HashSet<u64> = HashSet::new();
    let mut tokens: Vec<Token> = Vec::new();

    for pattern in patterns.iter() {
        let re = Regex::new(&pattern).unwrap();
        let matches: Vec<regex::Match<'_>> = re
            .find_iter(file)
            .filter(|m| !token_starts.contains(&(m.start() as u64)))
            .collect();

        for m in matches.iter() {
            let text = m.as_str();
            let s = m.start().clone();

            token_starts.insert(s as u64);

            let mut tk = Token {
                text: text.to_string(),
                token_type: match text {
                    "{" => TokenType::LBRACE,
                    "}" => TokenType::RBRACE,
                    "(" => TokenType::LPAREN,
                    ")" => TokenType::RPAREN,
                    ";" => TokenType::SEMICOLON,
                    "int" => TokenType::INTEGER,
                    "return" => TokenType::RETURN,
                    _ => TokenType::IDENTIFIER,
                },
                start: m.start() as u64,
                end: m.end() as u64,
            };

            if text.chars().all(char::is_numeric) {
                tk.token_type = TokenType::CONST;
            }

            tokens.push(tk);
        }
    }

    tokens.sort_by(|a, b| a.start.cmp(&b.start));

    tokens
}
