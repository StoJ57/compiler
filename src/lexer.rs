use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    PARENTH,
    SEMICOLON,
    IDENT,
    INT,
    OPER,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    text: String,
}
impl Token {
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
}

#[derive(Debug)]
pub struct LexError {
    text: String,
}
impl LexError {
    fn new<T: ToString>(msg: T) -> Self {
        Self {
            text: msg.to_string(),
        }
    }
}
impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.text)
    }
}
impl std::error::Error for LexError {
    fn description(&self) -> &str {
        &self.text
    }
}

macro_rules! error {
    ($($args: tt)*) => {
        Err(LexError::new(format!($($args)*)))
    }
}

const WHITESPACE: [char; 3] = ['\n', '\t', ' '];

const TOKENS: [(&str, TokenType); 8] = [
    ("\\{", TokenType::PARENTH),
    ("\\}", TokenType::PARENTH),
    ("\\(", TokenType::PARENTH),
    ("\\)", TokenType::PARENTH),
    (";", TokenType::SEMICOLON),
    (r"[a-zA-Z]\w*", TokenType::IDENT),
    ("[0-9]+", TokenType::INT),
    (
        r"[-~!+*/]|(&&)|(\|\|)|(==)|(!=)|(<)|(<=)|(>)|(>=)|(=)",
        TokenType::OPER,
    ),
];

pub fn get_tokens() -> Vec<(regex::Regex, TokenType)> {
    let mut tokens = vec![];
    for token in &TOKENS {
        tokens.push((Regex::new(token.0).unwrap(), token.1));
    }
    tokens
}

pub fn lex(text_in: &str) -> Result<Vec<Token>, LexError> {
    let mut text: String = text_in.to_string().clone();
    let mut out = vec![];
    while text.len() > 0 {
        if WHITESPACE.contains(&text.chars().nth(0).unwrap()) {
            text = text[1..text.len()].to_string();
            continue;
        }
        let mut found = false;
        for (regex, token_type) in get_tokens() {
            let opt_match = regex.find(&text);
            match opt_match {
                Some(reg_match) => {
                    if reg_match.start() == 0 {
                        out.push(Token {
                            text: reg_match.as_str().to_string(),
                            token_type: token_type,
                        });
                        text = text[reg_match.end()..text.len()].to_string();
                        found = true;
                        break;
                    }
                }
                None => (),
            }
        }
        if !found {
            return error!("Unknown token at '{}'", text);
        }
    }
    Ok(out)
}
