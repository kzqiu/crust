use crate::lexer::{Token, TokenType};
use std::iter::Peekable;
use std::slice::Iter;

pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub blocks: Vec<BlockItem>,
    // pub params: Vec<(String, TokenType)>,
    // pub return_type: TokenType,
}

// Highest Precedence for Binary Operators
pub enum Factor {
    Expr(Box<Expression>),
    UnaryOp(TokenType, Box<Factor>),
    Number(i32),
    Identifier(String),
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
    pub additional: Vec<EqualityExpr>,
}

pub struct BitXOrExpr {
    pub bit_and_expr: BitAndExpr,
    pub additional: Vec<BitAndExpr>,
}

pub struct BitOrExpr {
    pub bit_xor_expr: BitXOrExpr,
    pub additional: Vec<BitXOrExpr>,
}

pub struct LogicalAndExpr {
    pub bit_or_expr: BitOrExpr,
    pub additional: Vec<BitOrExpr>,
}

// Lowest Precedence for Binary Operators
pub struct LogicalOrExpr {
    pub log_and_expr: LogicalAndExpr,
    pub additional: Vec<LogicalAndExpr>,
}

pub struct ConditionalExpr {
    pub log_or_expr: LogicalOrExpr,
    pub additional: Option<(Box<Expression>, Box<ConditionalExpr>)>,
}

pub enum Expression {
    Assign(String, Box<Expression>),
    Conditional(ConditionalExpr),
}

pub enum Statement {
    Return(Expression),
    Expr(Expression),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
}

pub struct Declaration {
    pub identifier: String,
    pub expr: Option<Expression>,
}

pub enum BlockItem {
    Statement(Statement),
    Declaration(Declaration),
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
        TokenType::Identifier => Factor::Identifier(next.text.to_string()),
        _ => {
            dbg!(next);
            panic!();
        }
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
                    let next_expr = parse_eq_expr(tokens);
                    expr.additional.push(next_expr);
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
                    let next_expr = parse_bit_and_expr(tokens);
                    expr.additional.push(next_expr);
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
                    let next_expr = parse_bit_xor_expr(tokens);
                    expr.additional.push(next_expr);
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
                    let next_expr = parse_bit_or_expr(tokens);
                    expr.additional.push(next_expr);
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

fn parse_log_or_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> LogicalOrExpr {
    let log_and_expr = parse_log_and_expr(tokens);

    let mut expr = LogicalOrExpr {
        log_and_expr,
        additional: Vec::new(),
    };

    loop {
        if let Some(next) = tokens.peek() {
            match &next.token_type {
                TokenType::Or => {
                    let next_expr = parse_log_and_expr(tokens);
                    expr.additional.push(next_expr);
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

fn parse_conditional_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> ConditionalExpr {
    ConditionalExpr {
        log_or_expr: parse_log_or_expr(tokens),
        additional: match tokens.peek().unwrap().token_type {
            TokenType::QuestionMark => {
                tokens.next();
                let expr = parse_expr(tokens);

                match tokens.next().unwrap().token_type {
                    TokenType::Colon => {}
                    _ => panic!(),
                }

                let cond_expr = parse_conditional_expr(tokens);

                Some((Box::new(expr), Box::new(cond_expr)))
            }
            _ => None,
        },
    }
}

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Expression {
    // let tk = tokens.next().unwrap();
    let mut iter_cpy = tokens.clone();
    let first_tk = iter_cpy.next().unwrap();
    // dbg!(first_tk);
    // dbg!(iter_cpy.peek().unwrap());
    match (first_tk.token_type, iter_cpy.peek().unwrap().token_type) {
        (TokenType::Identifier, TokenType::Assign) => {
            tokens.next();
            tokens.next();
            Expression::Assign(first_tk.text.to_string(), Box::new(parse_expr(tokens)))
        }
        _ => Expression::Conditional(parse_conditional_expr(tokens)),
    }
}

fn parse_statement(tokens: &mut Peekable<Iter<'_, Token>>) -> Statement {
    let tk = tokens.peek().unwrap();
    let statement;
    match tk.token_type {
        TokenType::Return => {
            tokens.next(); // remove return token
            match tokens.peek().unwrap().token_type {
                // Return case
                TokenType::Literal
                | TokenType::Minus
                | TokenType::BitComplement
                | TokenType::LogicalNeg
                | TokenType::Identifier
                | TokenType::LParen => statement = Statement::Return(parse_expr(tokens)),
                _ => {
                    dbg!(tokens.peek().unwrap());
                    panic!();
                }
            }
            match tokens.next().unwrap().token_type {
                TokenType::Semicolon => {}
                _ => panic!(),
            }
        }
        TokenType::Identifier => {
            // Expression case
            statement = Statement::Expr(parse_expr(tokens));
            match tokens.next().unwrap().token_type {
                TokenType::Semicolon => {}
                _ => panic!(),
            }
        }
        TokenType::If => {
            tokens.next();
            match tokens.next().unwrap().token_type {
                TokenType::LParen => {}
                _ => panic!(),
            }

            let expr = parse_expr(tokens);

            match tokens.next().unwrap().token_type {
                TokenType::RParen => {}
                _ => panic!(),
            }

            let inner_statement = parse_statement(tokens);

            let else_statement = match tokens.peek().unwrap().token_type {
                TokenType::Else => {
                    tokens.next();
                    Some(parse_statement(tokens))
                }
                _ => None,
            };

            statement = Statement::If(
                expr,
                Box::new(inner_statement),
                match else_statement {
                    Some(s) => Some(Box::new(s)),
                    None => None,
                },
            )
        }
        _ => {
            dbg!(tk);
            panic!();
        }
    }

    statement
}

fn parse_declaration(tokens: &mut Peekable<Iter<'_, Token>>) -> Declaration {
    tokens.next(); // get rid of type
    let identifier = tokens.next().unwrap().text.as_str();
    tokens.next(); // get rid of equals
    let decl = Declaration {
        identifier: String::from(identifier),
        expr: match tokens.peek().unwrap().token_type {
            TokenType::Semicolon => None,
            _ => Some(parse_expr(tokens)),
        },
    };

    match tokens.next().unwrap().token_type {
        TokenType::Semicolon => {}
        _ => panic!(),
    }

    decl
}

fn parse_block(tokens: &mut Peekable<Iter<'_, Token>>) -> BlockItem {
    match tokens.peek().unwrap().token_type {
        TokenType::Integer => BlockItem::Declaration(parse_declaration(tokens)),
        _ => BlockItem::Statement(parse_statement(tokens)),
    }
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

    let mut blocks = Vec::new();

    while let Some(tk) = tokens.peek() {
        match tk.token_type {
            TokenType::RBrace => break,
            _ => blocks.push(parse_block(tokens)),
        }
    }

    tokens.next();

    Function { name, blocks }
}

pub fn parse(tokens: &Vec<Token>) -> Program {
    let mut prog = Program {
        functions: Vec::new(),
    };

    prog.functions.push(parse_fn(&mut tokens.iter().peekable()));

    prog
}
