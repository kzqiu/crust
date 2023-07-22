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
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Modulo,
    LBitShift,
    RBitShift,
    BitAnd,
    BitOr,
    BitXOr,
    Assign,
    AssignPlus,
    AssignMinus,
    AssignDivide,
    AssignMult,
    AssignMod,
    AssignLBitShift,
    AssignRBitShift,
    AssignBitAnd,
    AssignBitOr,
    AssignBitXOr,
    If,
    Else,
    Colon,
    QuestionMark,
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub start: u64,
    pub end: u64,
}

// assuming that intervals are inclusive!
fn in_interval_set(interval: (u64, u64), intervals: &Vec<(u64, u64)>) -> bool {
    for int in intervals.iter() {
        if interval.0 <= int.1 && interval.1 >= int.0 {
            return true;
        }
    }

    false
}

// fn merge_intervals(interval: (u64, u64), intervals: &mut Vec<(u64, u64)>) {
// three cases
// 1. no overlap, interval is just inserted
// 2. 1 overlap, one interval is merged with
// 3. 2 overlaps, two intervals and additional interval are merged into one
// for i in 0..intervals.len() {
//     if intervals[i]
// }
// }

pub fn lex(file: &str) -> Vec<Token> {
    // Somewhat inefficient, but it works well enough
    let patterns = [
        r"\{",
        r"\}",
        r"\(",
        r"\)",
        r";",
        r":",
        r"\?",
        r"int(?=[\s(]+)",
        r"return(?=[\s;]+)",
        r"if(?=[\s(]+)",
        r"else(?=[\s]+)",
        r"[a-zA-Z_]\w*",
        r"[0-9]+",
        r"==",
        r"<=",
        r">=",
        r"!=",
        r"\+=",
        r"-=",
        r"\*=",
        r"/=",
        r"%=",
        r"<<=",
        r">>=",
        r"&=",
        r"\|=",
        r"^=",
        r"~",
        r"!",
        r"\+",
        r"-",
        r"\*",
        r"/",
        r"%",
        r"<<",
        r">>",
        r"&&",
        r"\|\|",
        r"=",
        r"<",
        r">",
        r"&",
        r"\|",
        r"\^",
    ];

    let mut token_indices: HashSet<u64> = HashSet::new();
    let mut tokens: Vec<Token> = Vec::new();

    for pattern in patterns.iter() {
        let re = Regex::new(&pattern).unwrap();
        let matches: Vec<fancy_regex::Match<'_>> = re
            .find_iter(file)
            .filter(|m| {
                let tmp = m.as_ref().unwrap();
                for i in tmp.start()..tmp.end() {
                    if token_indices.contains(&(i as u64)) {
                        return false;
                    }
                }
                true
            })
            .map(|m| m.unwrap())
            .collect();

        for m in matches.iter() {
            let text = m.as_str();

            // TODO: Very inefficient, should just implement an intervals!
            for i in m.start()..m.end() {
                token_indices.insert(i as u64);
            }

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
                    "~" => TokenType::BitComplement,
                    "!" => TokenType::LogicalNeg,
                    "+" => TokenType::Addition,
                    "-" => TokenType::Minus,
                    "*" => TokenType::Multiplication,
                    "/" => TokenType::Division,
                    "%" => TokenType::Modulo,
                    "&&" => TokenType::And,
                    "||" => TokenType::Or,
                    "==" => TokenType::Equal,
                    "!=" => TokenType::NotEqual,
                    "<" => TokenType::LessThan,
                    "<=" => TokenType::LessThanEqual,
                    ">" => TokenType::GreaterThan,
                    ">=" => TokenType::GreaterThanEqual,
                    "&" => TokenType::BitAnd,
                    "|" => TokenType::BitOr,
                    "^" => TokenType::BitXOr,
                    "<<" => TokenType::LBitShift,
                    ">>" => TokenType::RBitShift,
                    "=" => TokenType::Assign,
                    "+=" => TokenType::AssignPlus,
                    "-=" => TokenType::AssignMinus,
                    "*=" => TokenType::AssignMult,
                    "/=" => TokenType::AssignDivide,
                    "%=" => TokenType::AssignMod,
                    "<<=" => TokenType::AssignLBitShift,
                    ">>=" => TokenType::AssignRBitShift,
                    "&=" => TokenType::AssignBitAnd,
                    "|=" => TokenType::AssignBitOr,
                    "^=" => TokenType::AssignBitXOr,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "?" => TokenType::QuestionMark,
                    ":" => TokenType::Colon,
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
