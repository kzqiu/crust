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

pub enum Factor {
    Expr(Box<Expression>),
    UnaryOp(TokenType, Box<Factor>),
    Number(i32),
}

pub struct Term {
    pub factor: Factor,
    pub additional: Vec<(TokenType, Factor)>,
}

pub struct Expression {
    pub term: Term,
    pub additional: Vec<(TokenType, Term)>,
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

fn parse_factor(tokens: &mut Peekable<Iter<'_, Token>>) -> Factor {
    let next = tokens.next().unwrap();
    match next.token_type {
        TokenType::LParen => {
            let expr = parse_expr(tokens);
            match tokens.next().unwrap().token_type {
                TokenType::RParen => {}
                _ => panic!(),
            }
            Factor::Expr(Box::new(expr))
        }
        TokenType::Minus | TokenType::BitComplement | TokenType::LogicalNeg => {
            let op = next.token_type;
            let factor = parse_factor(tokens);
            Factor::UnaryOp(op, Box::new(factor))
        }
        TokenType::Literal => Factor::Number(next.text.parse::<i32>().unwrap()),
        _ => panic!(),
    }
}

fn parse_term(tokens: &mut Peekable<Iter<'_, Token>>) -> Term {
    let factor = parse_factor(tokens);

    let mut term = Term {
        factor,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match next.token_type {
                TokenType::Multiplication | TokenType::Division => {
                    let op = tokens.next().unwrap().token_type;
                    let next_factor = parse_factor(tokens);
                    term.additional.push((op, next_factor));
                }
                _ => {
                    break;
                }
            }
        } else {
            panic!();
        }
    }

    term
}

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Expression {
    let term = parse_term(tokens);

    let mut expr = Expression {
        term,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::Minus | TokenType::Addition => {
                    let op = tokens.next().unwrap().token_type;
                    let next_term = parse_term(tokens);
                    expr.additional.push((op, next_term));
                }
                _ => {
                    break;
                }
            }
        } else {
            panic!();
        }
    }

    expr
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
