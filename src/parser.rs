use crate::lexer::{Token, TokenType};

pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    // pub params: Vec<(String, TokenType)>,
    pub statements: Vec<Statement>,
    // pub return_type:
}

pub struct Expression {
    pub val: i32,
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

// pub fn tree(prog: &Program) {}

fn same_token_type(tokens: &Vec<Token>, index: &i64, t_type: TokenType) -> bool {
    tokens[*index as usize].token_type == t_type
}

fn parse_expr(tokens: &Vec<Token>, index: &mut i64) -> Expression {
    if !same_token_type(tokens, index, TokenType::Literal) {
        panic!("{index}: Requires literal value to parse expression.");
    }

    Expression {
        val: tokens[*index as usize].text.parse::<i32>().unwrap(),
    }
}

fn parse_statement(tokens: &Vec<Token>, index: &mut i64) -> Statement {
    if !same_token_type(tokens, index, TokenType::Return) {
        panic!("{index}: Return statement requires \"return\".");
    }

    *index += 1;

    if !same_token_type(tokens, index, TokenType::Literal) {
        panic!("{index}: Return statement requires literal value.");
    }

    let expr = parse_expr(tokens, index);

    *index += 1;

    if !same_token_type(tokens, index, TokenType::Semicolon) {
        panic!("{index}: Return statement requires semicolon.");
    }

    Statement { expr }
}

fn parse_fn(tokens: &Vec<Token>, index: &mut i64) -> Function {
    if !same_token_type(tokens, index, TokenType::Integer) {
        panic!("{index}: Function declaration requires valid type.");
    }

    *index += 1;

    if !same_token_type(tokens, index, TokenType::Identifier) {
        panic!("{index}: Function declaration requires valid identifier.");
    }

    let name = tokens[*index as usize].text.to_string();

    *index += 1;

    if !same_token_type(tokens, index, TokenType::LParen) {
        panic!("{index}: Function declaration requires starting parenthesis.");
    }

    *index += 1;

    // Iterate through parameters here

    if !same_token_type(tokens, index, TokenType::RParen) {
        panic!("{index}: Function declaration requires ending parenthesis.");
    }

    *index += 1;

    if !same_token_type(tokens, index, TokenType::LBrace) {
        panic!("{index}: Function declaration requires starting brace.");
    }

    *index += 1;

    let mut statements = Vec::new();

    while (*index as usize) < tokens.len() && !same_token_type(tokens, index, TokenType::RBrace) {
        statements.push(parse_statement(tokens, index));
        *index += 1;
    }

    if *index as usize == tokens.len() {
        panic!("{index}: Function declaration requires valid ending brace.");
    }

    *index += 1;

    Function { name, statements }
}

pub fn parse(tokens: &Vec<Token>) -> Program {
    let mut prog = Program {
        functions: Vec::new(),
    };

    let mut index: i64 = 0;

    prog.functions.push(parse_fn(tokens, &mut index));

    prog
}
