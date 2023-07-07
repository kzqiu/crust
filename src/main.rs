/*
C compiler built in Rust.
Copyright Kevin Qiu 2023
*/

extern crate getopts;

use getopts::Options;
use regex::{Regex, RegexSet};
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

fn lex(file: &str) -> Vec<regex::Match> {
    let patterns = [
        r"\{",          // LBRACKET
        r"\}",          // RBRACKET
        r"\(",          // LPAREN
        r"\)",          // RPAREN
        r";",           // SEMICOLON
        r"int",         // INTEGER
        r"return",      // RETURN
        r"[a-zA-Z]\w*", // SYMBOL
        r"[0-9]+",      // CONST
    ];

    let reg_set = RegexSet::new(patterns).unwrap();
    let regexes: Vec<_> = reg_set
        .patterns()
        .iter()
        .map(|pat| Regex::new(pat).unwrap())
        .collect();
    let mut matches: Vec<_> = reg_set
        .matches(file)
        .into_iter()
        .map(|index| &regexes[index])
        .map(|re| re.find(file).unwrap())
        .collect();

    matches.sort_by(|a, b| b.start().cmp(&a.start()));

    matches
}

fn parse() {}

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
        let matches = lex(&file);
    } else {
        println!("Please input a valid path.");
        return;
    }
}
