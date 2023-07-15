use crate::lexer::TokenType;
use crate::parser::*;

pub fn generate_factor(text: &mut String, factor: &Factor) {
    // <factor> ::= "(" <exp> ")" | <unary_op> <factor> | <int>
    match &factor {
        Factor::Expr(boxed_expr) => {
            generate_expr(text, boxed_expr);
        }
        Factor::UnaryOp(op, boxed_factor) => {
            generate_factor(text, boxed_factor);
            match op {
                TokenType::Minus => text.push_str("neg %eax\n"),
                TokenType::BitComplement => text.push_str("not %eax\n"),
                TokenType::LogicalNeg => text.push_str("cmpl $0, %eax\nmovl $0, %eax\nsete %al\n"),
                _ => panic!(),
            }
        }
        Factor::Number(val) => {
            text.push_str(format!("movl ${}, %eax\n", val).as_str());
        }
    }
}

pub fn generate_term(text: &mut String, term: &Term) {
    // <term> ::= <factor> { ("*" | "/") <factor> }
    generate_factor(text, &term.factor);

    for (op, factor) in term.additional.iter() {
        text.push_str("push %eax\n");
        generate_factor(text, factor);

        match op {
            TokenType::Multiplication => text.push_str("pop %ecx\nimul %ecx, %eax\n"),
            TokenType::Division => {
                // e1 in eax, e2 in ecx, then sign extend.
                // stores quotient in eax, remainder in edx
                text.push_str("mov %ecx, %eax\npop %eax\ncdq\nidivl %ecx\n");
            }
            _ => panic!(),
        }
    }
}

pub fn generate_expr(text: &mut String, expr: &Expression) {
    // <exp> ::= <term> { ("+" | "-" <term> }
    generate_term(text, &expr.term);

    for (op, term) in expr.additional.iter() {
        text.push_str("push %eax\n");
        generate_term(text, term);
        text.push_str("pop %ecx\n");

        match op {
            TokenType::Addition => text.push_str("addl %ecx, $eax\n"),
            TokenType::Minus => text.push_str("subl %eax %ecx\n"),
            _ => panic!(),
        }
    }
}

pub fn generate_statement(text: &mut String, statement: &Statement) {
    // <function> :: "int" <id> "(" ")" "{" <statement> "}"
    generate_expr(text, &statement.expr);

    text.push_str("ret\n\n");
}

pub fn generate(prog: Program) -> String {
    // <program> ::= <function>
    let mut text = String::from(".globl main\n\n");

    for func in prog.functions.iter() {
        text.push_str(format!("{}:\n", func.name).as_str());

        for statement in func.statements.iter() {
            generate_statement(&mut text, statement);
        }
    }

    text
}

pub fn write_asm(path: &str, text: &str) {
    std::fs::write(format!("{}.s", path), text).unwrap();
}
