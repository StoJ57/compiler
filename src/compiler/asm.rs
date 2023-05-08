use std::collections::HashMap;

use super::tac;

#[derive(Debug, Clone)]
pub struct Register {
    pub name: String,
}

#[derive(Debug)]
pub struct RegisterSet {
    registers: Vec<Register>,
    available: Vec<usize>,
    map: HashMap<usize, usize>,
}

impl RegisterSet {
    pub fn new(registers: &Vec<Register>) -> Self {
        Self {
            available: (0..registers.len()).collect(),
            registers: registers.clone(),
            map: HashMap::new(),
        }
    }
    pub fn add_register(&mut self, id: usize) -> Option<Register> {
        let index = self.available.pop()?;
        self.map.insert(id, index);
        Some(self.registers[index].clone())
    }
    pub fn free_register(&mut self, i: usize) {
        self.available.push(*self.map.get(&i).unwrap());
        self.map.remove(&i);
    }
    pub fn get_register(&mut self, id: usize) -> Option<Register> {
        match self.map.get(&id) {
            Some(i) => Some((self.registers.get(*i)?).clone()),
            None => self.add_register(id),
        }
    }
}

pub trait Dialect {
    fn compile_address(&mut self, addr: &tac::Address) -> String;
    fn compile_line(&mut self, line: &tac::Line) -> String;
    fn compile_function(&mut self, name: &str, lines: &Vec<tac::Line>) -> String;
    fn compile_program(&mut self, program: &tac::Program) -> String;
}

pub struct GCC {
    register_set: RegisterSet,
}
impl GCC {
    pub fn new(register_set: RegisterSet) -> Self {
        Self { register_set }
    }
}
impl Dialect for GCC {
    fn compile_address(&mut self, addr: &tac::Address) -> String {
        match addr {
            tac::Address::Constant(value) => {
                format!("{}", value)
            }
            tac::Address::Variable(id) => {
                format!("{:?}", self.register_set.get_register(*id as usize))
            }
        }
    }

    fn compile_line(&mut self, line: &tac::Line) -> String {
        match line {
            tac::Line::Add(a, b) => {
                format!("add {}, {}\n", a, b)
            }
            tac::Line::Subtract(a, b) => {
                format!("sub {}, {}\n", a, b)
            }
            tac::Line::Return(a) => {
                format!("mov {}, %rax\nret\n", a)
            }
            tac::Line::Move(source, dest) => {
                format!("mov {}, {}\n", dest, source)
            }
        }
    }

    fn compile_function(&mut self, name: &str, lines: &Vec<tac::Line>) -> String {
        let mut out = format!(".global {name}\n{name}:\n");
        for line in lines {
            out += &self.compile_line(line);
        }
        out
    }

    fn compile_program(&mut self, program: &tac::Program) -> String {
        let mut out = String::new();
        for (name, func) in program.functions.iter() {
            out += &self.compile_function(name, &func.body);
        }
        out
    }
}
