use crate::lexer::{Token, TokenType};

pub mod ast;
mod AST;

use std::collections::VecDeque;
use lazy_static::lazy_static;

lazy_static! {

    static ref OP_ORDER: Vec<Vec<&'static str>> = {
            vec![
            vec!["||"],
            vec!["&&"],
            vec!["!=", "=="],
            vec!["<", ">", "<=", ">="],
            vec!["+", "-"],
            vec!["*", "/"]
        ]
    };

    static ref OP_LEN: i64 = OP_ORDER.len() as i64;
}


#[derive(Debug)]
pub struct ParseError {
    text: String
}
impl ParseError {
    fn new<T: ToString>(msg: T) -> Self {
        Self {
            text: msg.to_string()
        }
    }
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.text)
    }
}
impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        &self.text
    }
}

macro_rules! error {
    ($($args: tt)*) => {
        Err(ParseError::new(format!($($args)*)))
    }
}


pub fn parse_program(mut tokens: VecDeque<Token>) -> Result<ast::Program, ParseError>{
    let mut functions : Vec<ast::Function>= vec![];
    while tokens.len() > 0 {
        if tokens[0].text() != "int"{
            return error!("First token must be int");
        }
        if tokens[1].token_type() != &TokenType::IDENT {
            return error!("Function name not supplied");
        }
        let fname = tokens[1].text().to_string();
        if fname != "main" {
            return error!("Function name must be main");
        }
        if tokens[2].text() != "(" {
            return error!("Must have '(' after argument");
        }
        if tokens[3].text() != ")" {
            return error!("Function cannot have arguments");
        }
        if tokens[4].text() != "{" {
            return error!("Function cannot have arguments");
        }
        tokens = tokens.split_off(5);

        let mut fbody = vec![];
        while tokens[0].text() != "}" {
            match parse_statement(&mut tokens) {
                Ok(statement) => {
                    fbody.push(statement)
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        let func = ast::Function {
            name: fname,
            body: fbody
        };
        functions.push(func);
        tokens.pop_front();
    }
    Ok(ast::Program {
        functions: functions
    })
}

pub fn parse_statement(tokens: &mut VecDeque<Token>) -> Result<ast::Statement, ParseError>{
    let mut statement: Option<ast::Statement> = None;
    
    if tokens[0].text() == "return" {
        tokens.pop_front();
        statement = Some(ast::Statement::RETURN(
            parse_exp(tokens)?
        ));
    } else {
        println!("{}", tokens[0].text());
    }
    
    if tokens[0].token_type() != &TokenType::SEMICOLON {
        return error!("Statement must end semicolon");
    }
    *tokens = tokens.split_off(1);
    match statement {
        Some(s) => Ok(s),
        None => error!("Something went wrong")
    }
}

pub fn parse_exp (tokens: &mut VecDeque<Token>) -> Result<Box<dyn ast::Expression>, ParseError>{
    if tokens.len() > 1 && tokens[1].text() == "=" { //assignment
        if tokens[0].token_type() != &TokenType::IDENT {
            return error!("");
        }
        let var_name = tokens.pop_front().unwrap().text().to_string();
        tokens.pop_front(); //pop "="
        let exp = parse_exp_nassign(tokens, 0)?;
        let ast = ast::Assign {
            var_name: var_name,
            expression: exp
        };
        Ok(Box::new(ast))
    }
    else { //operation
        parse_exp_nassign(tokens, 0)
    }
}

pub fn parse_exp_nassign(tokens: &mut VecDeque<Token>, depth: i64) -> Result<Box<dyn ast::Expression>, ParseError>{
    let mut term = match depth + 1 {
        _ if depth+1 < *OP_LEN => parse_exp_nassign(tokens, depth+1)?,
        _ => parse_factor(tokens)?
    };
    while tokens[0].token_type() == &TokenType::OPER{
        if !OP_ORDER[depth as usize].contains(&tokens[0].text()){
            break;
        }
        let oper = tokens.pop_front().unwrap();
        let next_term = parse_exp_nassign(tokens, depth)?;
        term = Box::new(ast::BinOp {
            value_a: term,
            value_b: next_term,
            oper: match ast::BinOpType::from_str(oper.text()){
                Ok(x) => x,
                Err(_) => {
                    return error!("Invalid binary operator: ");
                }
            }
        })
    }
    Ok(term)
}

pub fn parse_factor(tokens: &mut VecDeque<Token>) -> Result<Box<dyn ast::Factor>, ParseError>{
    let next_token = tokens.pop_front().unwrap();
    match *next_token.token_type() {
        TokenType::PARENTH => todo!(),
        TokenType::IDENT => todo!(),
        TokenType::INT => {
            Ok(Box::new(ast::Const {
                value: match next_token.text().parse(){
                    Ok(x) => x,
                    Err(_) => {
                        return error!("Invalid integer constant");
                    }
                }
            }))
        },
        TokenType::OPER => {
            Ok(Box::new(ast::UnOp {
                value: parse_factor(tokens)?,
                oper: match ast::UnOpType::from_str(next_token.text()){
                    Ok(x) => x,
                    Err(_) => {
                        return error!("Invalid unary operator: {}", next_token.text())
                    }
                }
            }))
        },
        _ => error!("Wrong token in factor")
    }
}
