#![allow(dead_code)]
/*
C compiler built in Rust.
Copyright Kevin Qiu 2023
*/

extern crate getopts;
mod lexer;
mod parser;

use getopts::Options;
use std::env;
use std::fs;

static VERSION: &'static str = "0.1.0";

fn print_usage(prog: &str, opts: Options) {
    println!("crust {}", VERSION);
    println!("");
    println!("Usage:");
    println!("{} [OPTION]... [FILE]...", prog);
    println!("");
    println!("{}", opts.usage("C compiler in Rust"));
    println!("Source code: <https://github.com/kzqiu/crust>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optflag("h", "help", "display help and exit");
    opts.optflag("V", "version", "display current version");

    let matches = opts.parse(&args[1..]).unwrap();

    // Handling different flags
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        return;
    }

    if matches.opt_present("V") {
        println!("crust version: {}", VERSION);
        return;
    }

    let input = if matches.free.len() == 1 {
        matches.free[0].clone()
    } else {
        println!("Please specify a file name.");
        return;
    };

    if let Ok(file) = fs::read_to_string(input) {
        let tokens: Vec<lexer::Token> = lexer::lex(&file);

        // for (i, t) in tokens.iter().enumerate() {
        //     println!("{i}: {t}");
        // }

        let program: parser::Program = parser::parse(&tokens);

        // dbg!(tokens);
    } else {
        println!("Please input a valid path.");
        return;
    }
}
