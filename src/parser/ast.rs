use super::super::compiler::tac;

pub trait Type {
    fn pretty_print_at(&self, indent: i64, del: &str);
    fn pretty_print(&self){
        self.pretty_print_at(0, "   ");
    }
}

pub trait Expression: Type {
    fn compile_tac(&self, scope: &mut tac::Scope, target: u64, strict_target: bool) -> (Vec<tac::Line>, tac::Address);
}
pub trait Factor: Expression {}

pub struct Const {
    pub value: u64
}
impl Factor for Const {}
impl Expression for Const {
    fn compile_tac(&self, scope: &mut tac::Scope, target: u64, strict_target: bool) -> (Vec<tac::Line>, tac::Address) {
        if strict_target {
            return (
                vec![tac::Line::Move(tac::Address::Constant(self.value), tac::Address::Variable(target))],
                tac::Address::Variable(target)
            );
        }
        (vec![], tac::Address::Constant(self.value))
    }
}
impl Type for Const {
    fn pretty_print_at(&self, indent: i64, del: &str) {
        let prefix = String::from(del.repeat(indent as usize));
        println!("{}", prefix+"Const "+&self.value.to_string());
    }
}

#[derive(Clone, Copy)]
pub enum UnOpType {
    Negate, // -
    Complement, // ~
    Not, // !
}
impl UnOpType {
    pub fn from_str(oper: &str) -> Result<Self, super::ParseError>{
        match oper {
            "-" => Ok(Self::Negate),
            "~" => Ok(Self::Complement),
            "!" => Ok(Self::Not),
            _ => Err(super::ParseError::new(&format!("Unknown operation: {}", oper)))
        }
    }
}
impl Into<&'static str> for UnOpType {
    fn into(self) -> &'static str {
        match self {
            UnOpType::Negate => "-",
            UnOpType::Complement => "~",
            UnOpType::Not => "!"
        }
    }
}

pub struct UnOp {
    pub value: Box<dyn Expression>,
    pub oper: UnOpType
}
impl Factor for UnOp {}
impl Expression for UnOp {
    fn compile_tac(&self, scope: &mut tac::Scope, target: u64, strict_target: bool) -> (Vec<tac::Line>, tac::Address) {
        todo!()
    }
}
impl Type for UnOp {
    fn pretty_print_at(&self, indent: i64, del: &str) {
        let prefix = String::from(del.repeat(indent as usize));
        println!("{}", prefix.clone()+"UnOp "+self.oper.into());
        println!("{}Value:", prefix.clone()+del);
        self.value.pretty_print_at(indent+2, del);
    }
}

#[derive(Clone, Copy)]
pub enum BinOpType {
    BitwiseOr, // ||
    BitWiseAnd, // &&
    NotEqual, // !=
    Equal, // ==
    LessThan, // <
    GreaterThan, // >
    LessThanOrEq, // <=
    GreaterThanOrEq, // >=
    Add, // +
    Subtract, // -
}
impl BinOpType {
    pub fn from_str(oper: &str) -> Result<Self, super::ParseError>{
        match oper {
            "||" => Ok(Self::BitwiseOr),
            "&&" => Ok(Self::BitWiseAnd),
            "!=" => Ok(Self::NotEqual),
            "==" => Ok(Self::Equal),
            "<" => Ok(Self::LessThan),
            ">" => Ok(Self::GreaterThan),
            "<=" => Ok(Self::LessThanOrEq),
            ">=" => Ok(Self::GreaterThanOrEq),
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            _ => Err(super::ParseError::new(&format!("Unknown operation: {}", oper)))
        }
    }
}
impl Into<&'static str> for BinOpType {
    fn into(self) -> &'static str {
        match self {
            BinOpType::BitwiseOr => "||",
            BinOpType::BitWiseAnd => "&&",
            BinOpType::NotEqual => "!=",
            BinOpType::Equal => "==",
            BinOpType::LessThan => "<",
            BinOpType::GreaterThan => ">",
            BinOpType::LessThanOrEq => "<=",
            BinOpType::GreaterThanOrEq => ">=",
            BinOpType::Add => "+",
            BinOpType::Subtract => "-",
        }
    }
}

pub struct BinOp {
    pub value_a: Box<dyn Expression>,
    pub value_b: Box<dyn Expression>,
    pub oper: BinOpType
}
impl Expression for BinOp {
    fn compile_tac(&self, scope: &mut tac::Scope, target: u64, strict_target: bool) -> (Vec<tac::Line>, tac::Address) {
        todo!()
    }
}
impl Type for BinOp {
    fn pretty_print_at(&self, indent: i64, del: &str) {
        let prefix = String::from(del.repeat(indent as usize));
        println!("{}BinOp '{}'", prefix.clone(), Into::<&str>::into(self.oper));
        println!("{}ValueA:", prefix.clone()+del);
        self.value_a.pretty_print_at(indent+2, del);
        println!("{}ValueB:", prefix.clone()+del);
        self.value_b.pretty_print_at(indent+2, del);
    }
}

pub struct Assign {
    pub var_name: String,
    pub expression: Box<dyn Expression>
}
impl Expression for Assign {
    fn compile_tac(&self, scope: &mut tac::Scope, target: u64, strict_target: bool) -> (Vec<tac::Line>, tac::Address) {
        todo!()
    }
}
impl Type for Assign {
    fn pretty_print_at(&self, indent: i64, del: &str) {
        let prefix = String::from(del.repeat(indent as usize));
        println!("{}", prefix.clone()+"Assign");
        println!("{}", prefix.clone()+del+"Var: "+&self.var_name);
        self.expression.pretty_print_at(indent+1, del);
    }
}

pub enum Statement {
    DECLARE,
    RETURN(Box<dyn Expression>),
    EXPRESSION
}
impl Type for Statement {
    fn pretty_print_at(&self, indent: i64, del: &str) {
        let prefix = String::from(del.repeat(indent as usize));
        match self {
            Self::RETURN(exp)=> {
                println!("{}", prefix + "RETURN Statement with value:");
                exp.pretty_print_at(indent+1, del);
            }
            _ => {
                println!("{}", prefix+"Other Statement")
            }
        }
        
    }
}
pub struct Function {
    pub name: String,
    pub body: Vec<Statement>
}
impl Type for Function {
    fn pretty_print_at(&self, indent: i64, del: &str) {
        let prefix = String::from(del.repeat(indent as usize));
        println!("{}", prefix.clone() + "Function");
        println!("{}", prefix.clone() + del + "Name: " + &self.name);
        println!("{}", prefix.clone() + del + "Body: ");
        for statement in &self.body {
            statement.pretty_print_at(indent+2, del);
        }
    }
}

pub struct Program {
    pub functions: Vec<Function>
}
impl Type for Program {
    fn pretty_print_at(&self, indent: i64, del: &str) {
        let prefix = String::from(del.repeat(indent as usize));
        println!("{}", prefix.clone() + "Program");
        println!("{}", prefix.clone() + del + "Functions: ");
        for func in &self.functions {
            func.pretty_print_at(indent+2, del);
        }
    }
}



