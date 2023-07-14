use crate::lexer::{Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    // pub params: Vec<(String, TokenType)>,
    pub statements: Vec<Statement>,
    // pub return_type: TokenType,
}

pub enum Expression {
    UnaryOp(TokenType, Box<Expression>),
    BinaryOp(TokenType, Box<Expression>, Box<Expression>),
    Number(i32),
}

pub struct Statement {
    pub expr: Expression,
}

pub enum NodeType {
    Program,
    Function,
    Expression,
    Statement,
}

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Expression {
    let tk = tokens.next().unwrap();

    match tk.token_type {
        TokenType::Literal => Expression::Number(tk.text.parse::<i32>().unwrap()),
        TokenType::Minus | TokenType::BitComplement | TokenType::LogicalNeg => {
            Expression::UnaryOp(tk.token_type, Box::new(parse_expr(tokens)))
        }
        _ => panic!(),
    }
}

fn parse_statement(tokens: &mut Peekable<Iter<'_, Token>>) -> Statement {
    let tk = tokens.next().unwrap();
    match tk.token_type {
        TokenType::Return => match tokens.peek().unwrap().token_type {
            TokenType::Literal
            | TokenType::Minus
            | TokenType::BitComplement
            | TokenType::LogicalNeg => {}
            _ => panic!(),
        },
        _ => panic!(),
    }

    let expr = parse_expr(tokens);

    match tokens.next().unwrap().token_type {
        TokenType::Semicolon => {}
        _ => panic!(),
    }

    Statement { expr }
}

fn parse_fn(tokens: &mut Peekable<Iter<'_, Token>>) -> Function {
    let name;

    // Handle return type, function identifier, and left parenthesis.
    match tokens.next().unwrap().token_type {
        TokenType::Integer => {
            let tk = tokens.next().unwrap();
            match (tk.token_type, tokens.next().unwrap().token_type) {
                (TokenType::Identifier, TokenType::LParen) => {
                    name = tk.text.to_string();
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }

    // Handle function arguments, right parenthesis, and left brace.
    loop {
        let tk = tokens.next().unwrap();
        match tk.token_type {
            TokenType::RParen => match tokens.next().unwrap().token_type {
                TokenType::LBrace => break,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    let mut statements = Vec::new();

    while let Some(tk) = tokens.peek() {
        match tk.token_type {
            TokenType::RBrace => break,
            _ => statements.push(parse_statement(tokens)),
        }
    }

    tokens.next();

    Function { name, statements }
}

pub fn parse(tokens: &Vec<Token>) -> Program {
    let mut prog = Program {
        functions: Vec::new(),
    };

    prog.functions.push(parse_fn(&mut tokens.iter().peekable()));

    prog
}
