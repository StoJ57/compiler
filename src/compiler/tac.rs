use std::collections::HashMap;

use super::super::parser::ast;

pub enum Address {
    Constant(u64),
    Variable(u64)
}

pub enum Line {
    Add(Address, Address), // A += B
    Subtract(Address, Address), // A -= B
    Return(Address), //return A
    Move(Address, Address)
}

pub struct Scope {
    var_counter: u64,
    jump_counter: u64
}
impl Scope {
    pub fn new() -> Self {
        Self {
            var_counter: 0,
            jump_counter: 0
        }
    }
    pub fn var_label(&mut self) -> u64{
        if self.var_counter != 0 {
            self.var_counter += 1;
        }
        self.var_counter
    }
    pub fn jump_label(&mut self) -> u64 {
        if self.jump_counter != 0 {
            self.jump_counter += 1;
        }
        self.jump_counter
    }
}

fn compile_expression(expression: Box<dyn ast::Expression>, scope: &mut Scope, target: u64, strict_target: bool) -> (Vec<Line>, Address){
    //Target is Address::Variable where the result of the expression is put
    //Strict target - can the returned target be different to the argument given (e.g. Address::Constant)

    expression.compile_tac(scope, target, strict_target)
}

struct Function {
    body: Vec<Line>,
    scope: Scope,
}
impl Function {
    pub fn from_ast(ast: Vec<ast::Statement>) -> Self{
        let mut func = Self {
            body: vec![],
            scope: Scope::new(),
        };
        for statement in ast {
            match statement {
                ast::Statement::DECLARE => todo!(),
                ast::Statement::EXPRESSION => todo!(),
                ast::Statement::RETURN(expression) => {
                    let var = func.scope.var_label();
                    let (mut lines, var) = compile_expression(expression, &mut func.scope, var, false);
                    func.body.append(&mut lines);
                    func.body.push(Line::Return(var));
                }
            }
        }
        func
    }
}

pub struct Program {
    functions: HashMap<String, Function>
}
impl Program {
    pub fn from_ast(ast: ast::Program) -> Self{
        let mut program = Self {
            functions: HashMap::new()
        };
        for func in ast.functions {
            program.functions.insert(
                func.name,
                Function::from_ast(func.body)
            );
        }
        program
    }
}