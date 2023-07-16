#![allow(dead_code)]
/*
C compiler built in Rust.
Copyright Kevin Qiu 2023
*/

extern crate getopts;
mod generator;
mod lexer;
mod parser;

use getopts::Options;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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
    let mut output_path = String::from("crust_out");

    opts.optflag("h", "help", "display help and exit");
    opts.optflag("V", "version", "display current version");
    opts.optopt(
        "o",
        "output",
        "output to specific path",
        output_path.as_str(),
    );

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

    // if matches.opt_present("o") {
    //     if let Some(out) = matches.opt_str("o") {
    //         output_path = out;
    //     } else {
    //         println!("Please specify an output path.");
    //         return;
    //     }
    // }

    let input = if matches.free.len() == 1 {
        matches.free[0].clone()
    } else {
        println!("Please specify a file name.");
        return;
    };

    // let stem = Path::new(&input).file_stem();
    // output_path = stem.unwrap().to_str().unwrap().to_string();

    if let Ok(file) = fs::read_to_string(input) {
        let tokens: Vec<lexer::Token> = lexer::lex(&file);
        // dbg!(tokens);
        let program: parser::Program = parser::parse(&tokens);
        let asm = generator::generate(program);

        generator::write_asm(output_path.as_str(), asm.as_str());

        let output = Command::new("zsh")
            .arg("-c")
            .arg(format!("gcc {}.s -o {}", output_path, output_path))
            .output()
            .unwrap();

        let output = String::from_utf8(output.stdout).unwrap();
        println!("{}", output);
    } else {
        println!("Please input a valid path.");
        return;
    }
}
