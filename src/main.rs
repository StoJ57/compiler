#![feature(trait_upcasting)]
#![feature(let_chains)]

use lazy_static::lazy_static;
use std::collections::VecDeque;

mod compiler;
mod lexer;
mod parser;

use compiler::asm::Register;
use parser::ast::Type;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::compiler::asm::Dialect;

const TEXT: &'static str = "
int main() {
    return 2 + 2;
}
";

lazy_static! {
    static ref REGISTERS: Vec<Register> = vec![
        Register {
            name: String::from("rax")
        },
        Register {
            name: String::from("rcx")
        },
        Register {
            name: String::from("rdx")
        }
    ];
}

pub fn main() {
    let c = lexer::lex(TEXT).unwrap();
    let parsed = parser::parse_program(VecDeque::from(c)).unwrap();
    parsed.pretty_print();
    let tac = compiler::tac::Program::from_ast(parsed);
    println!("{}", tac);
    let asm =
        compiler::asm::GCC::new(compiler::asm::RegisterSet::new(&REGISTERS)).compile_program(&tac);
    println!("{}", asm);
    let mut file = File::create("out/out.s").unwrap();
    file.write_all(asm.as_bytes()).unwrap();
    let message = Command::new("gcc")
        .arg("-masm=intel")
        .arg("out.s")
        .args(["-o", "out.exe"])
        .current_dir(std::env::current_dir().unwrap().join("out"))
        .output()
        .expect("failed to execute process"); //needs linking - wrong format
    println!("{}", String::from_utf8(message.stdout).unwrap());
    println!("{}", String::from_utf8(message.stderr).unwrap());
    let output = Command::new(r#"out/out.exe"#)
        .output()
        .expect("failed to execute process"); //needs linking - wrong format
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", output.status);
}
