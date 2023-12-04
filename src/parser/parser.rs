use crate::lex::token::{Token, TokenType};
use crate::Lexer;
use std::{any::Any, collections::HashMap};

pub struct Interpreter {
    lexer: Lexer,                             // Lexer, to lex while parsing
    variables: HashMap<String, Box<dyn Any>>, // Variable assignment, this handles all vars
}

impl Interpreter {
    pub fn new(source_code: String) -> Self {
        Interpreter {
            lexer: Lexer::new(source_code),
            variables: HashMap::new(),
        }
    }
    pub fn parse(&mut self) {
        loop {
            let token = self.lexer.get_next_token();
            let val: TokenType;
        }
    }
    fn expression() {}
}
