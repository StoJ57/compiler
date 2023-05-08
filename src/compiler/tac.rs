use std::collections::HashMap;

use super::super::parser::ast;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Address {
    Constant(u64),
    Variable(u64),
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::Constant(value) => write!(f, "${}", value),
            Address::Variable(id) => write!(f, "%{}", id),
        }
    }
}

#[derive(Debug)]
pub enum Line {
    Add(Address, Address),      // A += B
    Subtract(Address, Address), // A -= B
    Return(Address),            // Return A
    Move(Address, Address),     // Move A into B
}
impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Line::Add(a, b) => writeln!(f, "add {}, {}", a, b),
            Line::Subtract(a, b) => writeln!(f, "subtract {}, {}", a, b),
            Line::Return(a) => writeln!(f, "return {}", a),
            Line::Move(a, b) => writeln!(f, "move {}, {}", a, b),
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    var_counter: u64,
    jump_counter: u64,
}
impl Scope {
    pub fn new() -> Self {
        Self {
            var_counter: 0,
            jump_counter: 0,
        }
    }
    pub fn var_label(&mut self) -> u64 {
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

fn compile_expression(
    expression: Box<dyn ast::Expression>,
    scope: &mut Scope,
    target: u64,
    strict_target: bool,
) -> (Vec<Line>, Address) {
    //Target is Address::Variable where the result of the expression is put
    //Strict target - can the returned target be different to the argument given (e.g. Address::Constant)

    expression.compile_tac(scope, target, strict_target)
}

#[derive(Debug)]
pub struct Function {
    pub body: Vec<Line>,
    scope: Scope,
}
impl Function {
    pub fn from_ast(ast: Vec<ast::Statement>) -> Self {
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
                    let (mut lines, var) =
                        compile_expression(expression, &mut func.scope, var, false);
                    func.body.append(&mut lines);
                    func.body.push(Line::Return(var));
                }
            }
        }
        func
    }
}
impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.body.iter() {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Program {
    pub functions: HashMap<String, Function>,
}
impl Program {
    pub fn from_ast(ast: ast::Program) -> Self {
        let mut program = Self {
            functions: HashMap::new(),
        };
        for func in ast.functions {
            program
                .functions
                .insert(func.name, Function::from_ast(func.body));
        }
        program
    }
}
impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, func) in self.functions.iter() {
            writeln!(f, "Function {}:", name)?;
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}
