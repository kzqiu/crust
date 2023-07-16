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

// Highest Precedence for Binary Operators
pub enum Factor {
    Expr(Box<Expression>),
    UnaryOp(TokenType, Box<Factor>),
    Number(i32),
}

pub struct Term {
    pub factor: Factor,
    pub additional: Vec<(TokenType, Factor)>,
}

pub struct AdditiveExpr {
    pub term: Term,
    pub additional: Vec<(TokenType, Term)>,
}

pub struct ShiftExpr {
    pub add_expr: AdditiveExpr,
    pub additional: Vec<(TokenType, AdditiveExpr)>,
}

pub struct RelationalExpr {
    pub shift_expr: ShiftExpr,
    pub additional: Vec<(TokenType, ShiftExpr)>,
}

pub struct EqualityExpr {
    pub rel_expr: RelationalExpr,
    pub additional: Vec<(TokenType, RelationalExpr)>,
}

pub struct BitAndExpr {
    pub eq_expr: EqualityExpr,
    pub additional: Vec<(TokenType, EqualityExpr)>,
}

pub struct BitXOrExpr {
    pub bit_and_expr: BitAndExpr,
    pub additional: Vec<(TokenType, BitAndExpr)>,
}

pub struct BitOrExpr {
    pub bit_xor_expr: BitXOrExpr,
    pub additional: Vec<(TokenType, BitXOrExpr)>,
}

pub struct LogicalAndExpr {
    pub bit_or_expr: BitOrExpr,
    pub additional: Vec<(TokenType, BitOrExpr)>,
}

// Lowest Precedence for Binary Operators
pub struct Expression {
    pub log_and_expr: LogicalAndExpr,
    pub additional: Vec<(TokenType, LogicalAndExpr)>,
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

fn parse_add_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> AdditiveExpr {
    let term = parse_term(tokens);

    let mut expr = AdditiveExpr {
        term,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::Addition | TokenType::Minus => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_term(tokens);
                    expr.additional.push((op, next_expr));
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

fn parse_shift_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> ShiftExpr {
    let add_expr = parse_add_expr(tokens);

    let mut expr = ShiftExpr {
        add_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::LBitShift | TokenType::RBitShift => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_add_expr(tokens);
                    expr.additional.push((op, next_expr));
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

fn parse_rel_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> RelationalExpr {
    let shift_expr = parse_shift_expr(tokens);

    let mut expr = RelationalExpr {
        shift_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::LessThan
                | TokenType::LessThanEqual
                | TokenType::GreaterThan
                | TokenType::GreaterThanEqual => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_shift_expr(tokens);
                    expr.additional.push((op, next_expr));
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

fn parse_eq_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> EqualityExpr {
    let rel_expr = parse_rel_expr(tokens);

    let mut expr = EqualityExpr {
        rel_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::Equal | TokenType::NotEqual => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_rel_expr(tokens);
                    expr.additional.push((op, next_expr));
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

fn parse_bit_and_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> BitAndExpr {
    let eq_expr = parse_eq_expr(tokens);

    let mut expr = BitAndExpr {
        eq_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::BitAnd => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_eq_expr(tokens);
                    expr.additional.push((op, next_expr));
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

fn parse_bit_xor_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> BitXOrExpr {
    let bit_and_expr = parse_bit_and_expr(tokens);

    let mut expr = BitXOrExpr {
        bit_and_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::BitXOr => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_bit_and_expr(tokens);
                    expr.additional.push((op, next_expr));
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

fn parse_bit_or_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> BitOrExpr {
    let bit_xor_expr = parse_bit_xor_expr(tokens);

    let mut expr = BitOrExpr {
        bit_xor_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::BitOr => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_bit_xor_expr(tokens);
                    expr.additional.push((op, next_expr));
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

fn parse_log_and_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> LogicalAndExpr {
    let bit_or_expr = parse_bit_or_expr(tokens);

    let mut expr = LogicalAndExpr {
        bit_or_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::And => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_bit_or_expr(tokens);
                    expr.additional.push((op, next_expr));
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

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Expression {
    let log_and_expr = parse_log_and_expr(tokens);

    let mut expr = Expression {
        log_and_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::Or => {
                    let op = tokens.next().unwrap().token_type;
                    let next_expr = parse_log_and_expr(tokens);
                    expr.additional.push((op, next_expr));
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
            | TokenType::LogicalNeg
            | TokenType::LParen => {}
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
