use crate::lexer::TokenType;
use crate::parser::*;

pub fn generate_factor(text: &mut String, factor: &Factor, counter: &mut u32) {
    // <factor> ::= "(" <exp> ")" | <unary_op> <factor> | <int>
    match &factor {
        Factor::Expr(boxed_expr) => {
            generate_expr(text, boxed_expr, counter);
        }
        Factor::UnaryOp(op, boxed_factor) => {
            generate_factor(text, boxed_factor, counter);
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

pub fn generate_term(text: &mut String, term: &Term, counter: &mut u32) {
    // <term> ::= <factor> { ("*" | "/") <factor> }
    generate_factor(text, &term.factor, counter);

    for (op, factor) in term.additional.iter() {
        text.push_str("push %rax\n");
        generate_factor(text, factor, counter);

        match op {
            TokenType::Multiplication => text.push_str("pop %rcx\nimul %ecx, %eax\n"),
            TokenType::Division => {
                // e1 in eax, e2 in ecx, then sign extend (cdq) eax into [edx:eax]
                // stores quotient in eax, remainder in edx
                text.push_str("movl %eax, %ecx\npop %rax\ncdq\nidivl %ecx\n");
            }
            _ => panic!(),
        }
    }
}

pub fn generate_add_expr(text: &mut String, add_expr: &AdditiveExpr, counter: &mut u32) {
    // <add-expr> ::= <term> { ("+" | "-") <term> }
    generate_term(text, &add_expr.term, counter);

    for (op, expr) in add_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_term(text, expr, counter);

        match op {
            TokenType::Addition => text.push_str("pop %rcx\naddl %ecx, %eax\n"),
            TokenType::Minus => text.push_str("movl %eax, %ecx\npop %rax\nsubl %ecx, %eax\n"),
            _ => panic!(),
        }
    }
}

pub fn generate_shift_expr(text: &mut String, shift_expr: &ShiftExpr, counter: &mut u32) {
    // <shift-expr> ::= <add-expr> { ("<<" | ">>") <add-expr> }
    generate_add_expr(text, &shift_expr.add_expr, counter);

    for (op, expr) in shift_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_add_expr(text, expr, counter);
        text.push_str("movl %eax, %ecx\npop %rax\n");

        match op {
            TokenType::LBitShift => text.push_str("sal %ecx, %eax\n"),
            TokenType::RBitShift => text.push_str("sar %ecx, %eax\n"),
            _ => panic!(),
        }
    }
}

pub fn generate_rel_expr(text: &mut String, rel_expr: &RelationalExpr, counter: &mut u32) {
    // <rel-expr> ::= <shift-expr> { ("<" | ">" | "<=" | ">=") <shift-expr> }
    generate_shift_expr(text, &rel_expr.shift_expr, counter);

    for (op, expr) in rel_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_shift_expr(text, expr, counter);
        text.push_str("pop %rcx\ncmpl %eax, %ecx\nmovl $0, %eax\n");

        match op {
            TokenType::LessThan => text.push_str("setl %al\n"),
            TokenType::LessThanEqual => text.push_str("setle %al\n"),
            TokenType::GreaterThan => text.push_str("setg %al\n"),
            TokenType::GreaterThanEqual => text.push_str("setge %al\n"),
            _ => panic!(),
        }
    }
}

pub fn generate_eq_expr(text: &mut String, eq_expr: &EqualityExpr, counter: &mut u32) {
    // <eq-expr> ::= <rel-expr> { ("!=" | "==") <rel-expr> }
    generate_rel_expr(text, &eq_expr.rel_expr, counter);

    for (op, expr) in eq_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_rel_expr(text, expr, counter);
        text.push_str("pop %rcx\ncmpl %eax, %ecx\nmovl $0, %eax\n");

        match op {
            TokenType::Equal => text.push_str("sete %al\n"),
            TokenType::NotEqual => text.push_str("setne %al\n"),
            _ => panic!(),
        }
    }
}

pub fn generate_bit_and_expr(text: &mut String, bit_and_expr: &BitAndExpr, counter: &mut u32) {
    // <bit-and-expr> ::= <eq-expr> { "&" <eq-expr> }
    generate_eq_expr(text, &bit_and_expr.eq_expr, counter);

    for (op, expr) in bit_and_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_eq_expr(text, expr, counter);
        text.push_str("pop %rcx\n");

        match op {
            TokenType::BitAnd => text.push_str("and %ecx, %eax\n"),
            _ => panic!(),
        }
    }
}

pub fn generate_bit_xor_expr(text: &mut String, bit_xor_expr: &BitXOrExpr, counter: &mut u32) {
    // <bit-xor-expr> ::= <bit-and-expr> { "^" <bit-and-expr> }
    generate_bit_and_expr(text, &bit_xor_expr.bit_and_expr, counter);

    for (op, expr) in bit_xor_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_bit_and_expr(text, expr, counter);
        text.push_str("pop %rcx\n");

        match op {
            TokenType::BitXOr => text.push_str("xor %ecx, %eax\n"),
            _ => panic!(),
        }
    }
}

pub fn generate_bit_or_expr(text: &mut String, bit_or_expr: &BitOrExpr, counter: &mut u32) {
    // <bit-or-expr> ::= <bit-xor-expr> { "|" <bit-xor-expr> }
    generate_bit_xor_expr(text, &bit_or_expr.bit_xor_expr, counter);

    for (op, expr) in bit_or_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_bit_xor_expr(text, expr, counter);
        text.push_str("pop %rcx\n");

        match op {
            TokenType::BitOr => text.push_str("or %ecx, %eax\n"),
            _ => panic!(),
        }
    }
}

// TODO
pub fn generate_log_and_expr(text: &mut String, log_and_expr: &LogicalAndExpr, counter: &mut u32) {
    // <log-and-expr> ::= <bit-or-expr> { "&&" <bit-or-expr> }
    generate_bit_or_expr(text, &log_and_expr.bit_or_expr, counter);

    for (op, expr) in log_and_expr.additional.iter() {
        match op {
            TokenType::And => {
                text.push_str(
                    format!(
                        "cmpl $0, %eax\njne _clause{}\njmp _end{}\n_clause{}\n",
                        counter, counter, counter
                    )
                    .as_str(),
                );
                generate_bit_or_expr(text, expr, counter);
                text.push_str(
                    format!(
                        "cmpl $0, %eax\nmovl $0, %eax\nsetne %al\n_end{}:\n",
                        counter
                    )
                    .as_str(),
                );
                *counter += 1;
            }
            _ => panic!(),
        }
    }
}

pub fn generate_expr(text: &mut String, expr: &Expression, counter: &mut u32) {
    // <exp> ::= <logical-or-expr> { "||" <logical-or-expr> }
    generate_log_and_expr(text, &expr.log_and_expr, counter);

    for (op, expr) in expr.additional.iter() {
        match op {
            TokenType::Or => {
                text.push_str(
                    format!(
                        "cmpl $0, %eax\nje _clause{}\nmovl $1, %eax\njmp _end{}\n_clause{}:\n",
                        counter, counter, counter
                    )
                    .as_str(),
                );
                generate_log_and_expr(text, expr, counter);
                text.push_str(
                    format!(
                        "cmpl $0, %eax\nmovl $0, %eax\nsetne %al\n_end{}:\n",
                        counter
                    )
                    .as_str(),
                );
                *counter += 1;
            }
            _ => panic!(),
        }
    }
}

pub fn generate_statement(text: &mut String, statement: &Statement, counter: &mut u32) {
    // <function> :: "int" <id> "(" ")" "{" <statement> "}"
    generate_expr(text, &statement.expr, counter);
    text.push_str("ret\n\n");
}

pub fn generate(prog: Program) -> String {
    // <program> ::= <function>
    let mut text = String::from(".globl main\n\n");
    let mut counter: u32 = 0;

    for func in prog.functions.iter() {
        text.push_str(format!("{}:\n", func.name).as_str());

        for statement in func.statements.iter() {
            generate_statement(&mut text, statement, &mut counter);
        }
    }

    text
}

pub fn write_asm(path: &str, text: &str) {
    std::fs::write(format!("{}.s", path), text).unwrap();
}
