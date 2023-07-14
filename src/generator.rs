use crate::lexer::TokenType;
use crate::parser::*;

pub fn generate_expr(text: &mut String, expr: &Expression) {
    if let Some(inner_val) = expr.val {
        text.push_str(format!("movl ${}, %eax\n", inner_val).as_str());
        return;
    }

    if let Some(inner_expr) = &expr.expr {
        generate_expr(text, inner_expr.as_ref());
    }

    match expr.unary_op.unwrap() {
        TokenType::Negation => text.push_str("neg %eax\n"),
        TokenType::BitComplement => text.push_str("not %eax\n"),
        TokenType::LogicalNeg => text.push_str("cmpl $0, %eax\nmovl $0, %eax\nsete %al\n"),
        _ => {}
    }
}

pub fn generate_statement(text: &mut String, statement: &Statement) {
    generate_expr(text, &statement.expr);

    text.push_str("ret\n\n");
}

pub fn generate(prog: Program) -> String {
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
