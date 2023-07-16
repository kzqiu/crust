# crust 🥧

Crust is a small C to x86-64 compiler, built using Rust. This project is meant for me to learn about compilers and the modules that they are composed of, so instead of using a lexer or parser generator, I try to build each component from scratch. Of course, this means that things will be done incorrectly and inefficiently but, with the theoreticals in mind, it should be possible to make something useable for simple programs.

I'm very much so still learning about compilers, C, assembly, and Rust, so this seemed like a great little project to just learn from. I got the idea from [Nora Sandler's amazing series of blog posts](https://norasandler.com/2017/11/29/Write-a-Compiler.html).

If you have any advice for this, please reach out!

## Roadmap
* [x] Basic lexing, parsing (into AST), and x86-64 assembly generation.
* [x] Unary Operators (~, !, -), not including prefix, postfix, or referencing.
* [x] Implement binary operators (numerical, binary, logical, relational) and proper order of operations.
* [x] Local int variables, assignment operators, basic stack frames.
* [ ] Compound assignment operators.
* [ ] Conditional flow.
* [ ] Compound statements (nested scopes).
* [ ] Loops (for, while, break and continue keywords).
* [ ] Additional functions.
* [ ] Global variables.

## Much later...
* [ ] Additional types (floats, arrays, pointers, and structs).
* [ ] Optimizations.
