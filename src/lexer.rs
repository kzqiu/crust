use fancy_regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicolon,
    Integer,
    Return,
    Identifier,
    Literal,
    Minus,
    BitComplement,
    LogicalNeg,
    Addition,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub start: u64,
    pub end: u64,
}

pub fn lex(file: &str) -> Vec<Token> {
    let patterns = [
        r"\{",               // LBRACE
        r"\}",               // RBRACE
        r"\(",               // LPAREN
        r"\)",               // RPAREN
        r";",                // SEMICOLON
        r"int(?=[\s(]*)",    // INTEGER
        r"return(?=[\s;]*)", // RETURN
        r"[a-zA-Z]+\w+",     // IDENTIFIER -> Try "([a-zA-Z])+\w*"
        r"[0-9]+",           // LITERAL
        r"-",                // Minus
        r"~",                // BIT_COMPLEMENT
        r"!",                // LOGICAL_NEG
        r"\+",
        r"\*",
        r"/",
    ];

    let mut token_starts: HashSet<u64> = HashSet::new();
    let mut tokens: Vec<Token> = Vec::new();

    for pattern in patterns.iter() {
        let re = Regex::new(&pattern).unwrap();
        let matches: Vec<fancy_regex::Match<'_>> = re
            .find_iter(file)
            .filter(|m| !token_starts.contains(&(m.as_ref().unwrap().start() as u64)))
            .map(|m| m.unwrap())
            .collect();

        for m in matches.iter() {
            let text = m.as_str();
            let s = m.start().clone();

            token_starts.insert(s as u64);

            let mut tk = Token {
                text: text.to_string(),
                token_type: match text {
                    "{" => TokenType::LBrace,
                    "}" => TokenType::RBrace,
                    "(" => TokenType::LParen,
                    ")" => TokenType::RParen,
                    ";" => TokenType::Semicolon,
                    "int" => TokenType::Integer,
                    "return" => TokenType::Return,
                    "-" => TokenType::Minus,
                    "~" => TokenType::BitComplement,
                    "!" => TokenType::LogicalNeg,
                    "+" => TokenType::Addition,
                    "*" => TokenType::Multiplication,
                    "/" => TokenType::Division,
                    _ => TokenType::Identifier,
                },
                start: m.start() as u64,
                end: m.end() as u64,
            };

            if text.chars().all(char::is_numeric) {
                tk.token_type = TokenType::Literal;
            }

            tokens.push(tk);
        }
    }

    tokens.sort_by(|a, b| a.start.cmp(&b.start));

    tokens
}
