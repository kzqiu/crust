use crate::parser::*;
// use std::fs::File;

pub fn generate(prog: Program) -> String {
    let mut text = String::from(".globl main\n\n");

    for func in prog.functions.iter() {
        text.push_str(format!("{}:\n", func.name).as_str());

        for statement in func.statements.iter() {
            text.push_str(format!("movl ${}, %eax\nret\n", statement.expr.val).as_str());
        }
    }

    text
}

pub fn write_asm(path: &str, text: &str) {
    std::fs::write(path, text).unwrap();
}
