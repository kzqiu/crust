use crate::lexer::TokenType;
use crate::parser::*;
use std::collections::HashMap;

struct StackInfo {
    counter: u32,
    stack_index: i32,
    var_map: HashMap<String, i32>,
}

pub fn generate_factor(text: &mut String, factor: &Factor, stack_info: &mut StackInfo) {
    // <factor> ::= "(" <exp> ")" | <unary_op> <factor> | <int>
    match &factor {
        Factor::Expr(boxed_expr) => {
            generate_expr(text, boxed_expr, stack_info);
        }
        Factor::UnaryOp(op, boxed_factor) => {
            generate_factor(text, boxed_factor, stack_info);
            match op {
                TokenType::Minus => text.push_str("neg %eax\n"),
                TokenType::BitComplement => text.push_str("not %eax\n"),
                TokenType::LogicalNeg => text.push_str("cmpl $0, %eax\nmovl $0, %eax\nsete %al\n"),
                _ => {
                    dbg!(op);
                    panic!();
                }
            }
        }
        Factor::Number(val) => {
            text.push_str(format!("movl ${}, %eax\n", val).as_str());
        }
        Factor::Identifier(name) => {
            let offset = stack_info.var_map.get(name).unwrap();
            text.push_str(format!("movl {}(%ebp), %eax\n", offset).as_str());
        }
    }
}

pub fn generate_term(text: &mut String, term: &Term, stack_info: &mut StackInfo) {
    // <term> ::= <factor> { ("*" | "/") <factor> }
    generate_factor(text, &term.factor, stack_info);

    for (op, factor) in term.additional.iter() {
        text.push_str("push %rax\n");
        generate_factor(text, factor, stack_info);

        match op {
            TokenType::Multiplication => text.push_str("pop %rcx\nimul %ecx, %eax\n"),
            TokenType::Division => {
                // e1 in eax, e2 in ecx, then sign extend (cdq) eax into [edx:eax]
                // stores quotient in eax, remainder in edx
                text.push_str("movl %eax, %ecx\npop %rax\ncdq\nidivl %ecx\n");
            }
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_add_expr(text: &mut String, add_expr: &AdditiveExpr, stack_info: &mut StackInfo) {
    // <add-expr> ::= <term> { ("+" | "-") <term> }
    generate_term(text, &add_expr.term, stack_info);

    for (op, expr) in add_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_term(text, expr, stack_info);

        match op {
            TokenType::Addition => text.push_str("pop %rcx\naddl %ecx, %eax\n"),
            TokenType::Minus => text.push_str("movl %eax, %ecx\npop %rax\nsubl %ecx, %eax\n"),
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_shift_expr(text: &mut String, shift_expr: &ShiftExpr, stack_info: &mut StackInfo) {
    // <shift-expr> ::= <add-expr> { ("<<" | ">>") <add-expr> }
    generate_add_expr(text, &shift_expr.add_expr, stack_info);

    for (op, expr) in shift_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_add_expr(text, expr, stack_info);
        text.push_str("movl %eax, %ecx\npop %rax\n");

        match op {
            TokenType::LBitShift => text.push_str("sal %ecx, %eax\n"),
            TokenType::RBitShift => text.push_str("sar %ecx, %eax\n"),
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_rel_expr(text: &mut String, rel_expr: &RelationalExpr, stack_info: &mut StackInfo) {
    // <rel-expr> ::= <shift-expr> { ("<" | ">" | "<=" | ">=") <shift-expr> }
    generate_shift_expr(text, &rel_expr.shift_expr, stack_info);

    for (op, expr) in rel_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_shift_expr(text, expr, stack_info);
        text.push_str("pop %rcx\ncmpl %eax, %ecx\nmovl $0, %eax\n");

        match op {
            TokenType::LessThan => text.push_str("setl %al\n"),
            TokenType::LessThanEqual => text.push_str("setle %al\n"),
            TokenType::GreaterThan => text.push_str("setg %al\n"),
            TokenType::GreaterThanEqual => text.push_str("setge %al\n"),
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_eq_expr(text: &mut String, eq_expr: &EqualityExpr, stack_info: &mut StackInfo) {
    // <eq-expr> ::= <rel-expr> { ("!=" | "==") <rel-expr> }
    generate_rel_expr(text, &eq_expr.rel_expr, stack_info);

    for (op, expr) in eq_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_rel_expr(text, expr, stack_info);
        text.push_str("pop %rcx\ncmpl %eax, %ecx\nmovl $0, %eax\n");

        match op {
            TokenType::Equal => text.push_str("sete %al\n"),
            TokenType::NotEqual => text.push_str("setne %al\n"),
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_bit_and_expr(
    text: &mut String,
    bit_and_expr: &BitAndExpr,
    stack_info: &mut StackInfo,
) {
    // <bit-and-expr> ::= <eq-expr> { "&" <eq-expr> }
    generate_eq_expr(text, &bit_and_expr.eq_expr, stack_info);

    for (op, expr) in bit_and_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_eq_expr(text, expr, stack_info);
        text.push_str("pop %rcx\n");

        match op {
            TokenType::BitAnd => text.push_str("and %ecx, %eax\n"),
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_bit_xor_expr(
    text: &mut String,
    bit_xor_expr: &BitXOrExpr,
    stack_info: &mut StackInfo,
) {
    // <bit-xor-expr> ::= <bit-and-expr> { "^" <bit-and-expr> }
    generate_bit_and_expr(text, &bit_xor_expr.bit_and_expr, stack_info);

    for (op, expr) in bit_xor_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_bit_and_expr(text, expr, stack_info);
        text.push_str("pop %rcx\n");

        match op {
            TokenType::BitXOr => text.push_str("xor %ecx, %eax\n"),
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_bit_or_expr(
    text: &mut String,
    bit_or_expr: &BitOrExpr,
    stack_info: &mut StackInfo,
) {
    // <bit-or-expr> ::= <bit-xor-expr> { "|" <bit-xor-expr> }
    generate_bit_xor_expr(text, &bit_or_expr.bit_xor_expr, stack_info);

    for (op, expr) in bit_or_expr.additional.iter() {
        text.push_str("push %rax\n");
        generate_bit_xor_expr(text, expr, stack_info);
        text.push_str("pop %rcx\n");

        match op {
            TokenType::BitOr => text.push_str("or %ecx, %eax\n"),
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_log_and_expr(
    text: &mut String,
    log_and_expr: &LogicalAndExpr,
    stack_info: &mut StackInfo,
) {
    // <log-and-expr> ::= <bit-or-expr> { "&&" <bit-or-expr> }
    generate_bit_or_expr(text, &log_and_expr.bit_or_expr, stack_info);

    for (op, expr) in log_and_expr.additional.iter() {
        match op {
            TokenType::And => {
                let c = stack_info.counter;
                text.push_str(
                    format!(
                        "cmpl $0, %eax\njne _clause{}\njmp _end{}\n_clause{}:\n",
                        c, c, c
                    )
                    .as_str(),
                );
                generate_bit_or_expr(text, expr, stack_info);
                text.push_str(
                    format!("cmpl $0, %eax\nmovl $0, %eax\nsetne %al\n_end{}:\n", c).as_str(),
                );
                stack_info.counter += 1;
            }
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_log_or_expr(
    text: &mut String,
    log_or_expr: &LogicalOrExpr,
    stack_info: &mut StackInfo,
) {
    // <logical-or-expr> ::= <logical-and-expr> { "||" <logical-and-expr> }
    generate_log_and_expr(text, &log_or_expr.log_and_expr, stack_info);

    for (op, expr) in log_or_expr.additional.iter() {
        match op {
            TokenType::Or => {
                let c = stack_info.counter;
                text.push_str(
                    format!(
                        "cmpl $0, %eax\nje _clause{}\nmovl $1, %eax\njmp _end{}\n_clause{}:\n",
                        c, c, c
                    )
                    .as_str(),
                );
                generate_log_and_expr(text, expr, stack_info);
                text.push_str(
                    format!("cmpl $0, %eax\nmovl $0, %eax\nsetne %al\n_end{}:\n", c).as_str(),
                );
                stack_info.counter += 1;
            }
            _ => {
                dbg!(op);
                panic!();
            }
        }
    }
}

pub fn generate_expr(text: &mut String, expr: &Expression, stack_info: &mut StackInfo) {
    match expr {
        Expression::Assign(name, inner_expr) => {
            generate_expr(text, inner_expr, stack_info);
            let offset = stack_info.var_map.get(name).unwrap();
            text.push_str(format!("movl %eax, {}(%ebp)", offset).as_str());
        }
        Expression::Else(log_or_expr) => {
            generate_log_or_expr(text, log_or_expr, stack_info);
        }
    }
}

pub fn generate_statement(text: &mut String, statement: &Statement, stack_info: &mut StackInfo) {
    // <function> :: "int" <id> "(" ")" "{" { <statement> } "}"
    match statement {
        Statement::Declare(name, opt_expr) => {
            if let Some(_) = stack_info.var_map.get(name) {
                panic!("Variable \"{}\" already declared in this scope.", name);
            }

            if let Some(inner_expr) = opt_expr {
                generate_expr(text, inner_expr, stack_info);
            } else {
                // set to 0
                text.push_str("movl $0, %eax\n");
            }

            text.push_str("pushl %rax\n");
            stack_info
                .var_map
                .insert(name.to_string(), stack_info.stack_index);
            stack_info.stack_index -= 8;
        }
        Statement::Expr(expr) => {
            generate_expr(text, expr, stack_info);
        }
        Statement::Return(expr) => {
            generate_expr(text, expr, stack_info);
            text.push_str("ret\n\n");
        }
    }
    text.push_str("movl %ebp, %esp\npop %rbp\nret\n\n");
}

pub fn generate(prog: Program) -> String {
    // <program> ::= <function>
    let mut text = String::from(".globl main\n\n");
    let mut stack_info = StackInfo {
        var_map: HashMap::new(),
        stack_index: 0,
        counter: 0,
    };

    for func in prog.functions.iter() {
        text.push_str(format!("{}:\npush %rbp\nmovl %esp, %ebp\n", func.name).as_str());

        let mut has_ret: bool = false;

        for statement in func.statements.iter() {
            if let Statement::Return(_) = statement {
                has_ret = true;
            }

            generate_statement(&mut text, statement, &mut stack_info);
        }

        if !has_ret {
            text.push_str("movl $0, %eax\nret\n")
        }
    }

    text
}

pub fn write_asm(path: &str, text: &str) {
    std::fs::write(format!("{}.s", path), text).unwrap();
}
