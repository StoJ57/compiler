#![feature(trait_upcasting)]

use std::collections::VecDeque;

mod lexer;
mod parser;
mod compiler;

use parser::ast::Type;

const TEXT: &'static str = "
int main(){
    return 1 + 2 - 3;
}
";


pub fn main(){
    let c = lexer::lex(TEXT).unwrap();
    let parsed = parser::parse_program(VecDeque::from(c)).unwrap();
    parsed.pretty_print();
    let tac = compiler::tac::Program::from_ast(parsed);
}