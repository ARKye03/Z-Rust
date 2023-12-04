use crate::lex::token::{Token, TokenType};
use crate::Lexer;
use std::{any::Any, collections::HashMap};

pub enum Value {
    Str(String),
    Float(f32),
}
pub struct Interpreter {
    lexer: Lexer,                             // Lexer, to lex while parsing
    variables: HashMap<String, Box<dyn Any>>, // Variable assignment, this handles all vars
}

impl Interpreter {
    /// Constructor of the interpreter
    ///
    /// # Arguments
    ///
    /// * `source_code` - The source code to be interpreted
    ///
    /// # Returns
    ///
    /// * The interpreter object
    pub fn new(source_code: String) -> Self {
        Interpreter {
            lexer: Lexer::new(source_code),
            variables: HashMap::new(),
        }
    }
    pub fn parse(&mut self) {
        let token = self.lexer.get_next_token();
        let mut val = self.expression();
        println!("{}", val.downcast::<String>().unwrap());
    }
    fn expression(&mut self) -> Box<dyn Any> {
        let mut left = self.term();

        loop {
            let token = self.lexer.get_next_token();

            let left_value = match left.downcast::<String>() {
                Ok(s) => Value::Str((*s).clone()),
                Err(_) => Value::Float(*left.downcast::<f32>().unwrap()),
            };

            if token.token_type == TokenType::Operator && (token.value == "+" || token.value == "-")
            {
                let right = self.term();
                left = self.binary_operation(left, token, right);
            } else {
                self.lexer.unget_token(token);
                return left;
            }
        }
    }

    fn term(&mut self) -> Box<dyn Any> {
        let mut left = self.power();

        loop {
            let token = self.lexer.get_next_token();

            if token.token_type != TokenType::Operator
                || (token.value != "*" && token.value != "/" && token.value != "%")
            {
                self.lexer.unget_token(token);
                return left;
            }
            let right = self.power();
            left = self.binary_operation(left, token, right);
        }
    }

    fn power(&mut self) -> Box<dyn Any> {
        let mut left = self.primary();

        loop {
            let token = self.lexer.get_next_token();

            if token.token_type != TokenType::Operator || token.value != "^" {
                self.lexer.unget_token(token);
                return left;
            }
            let right = self.primary();
            left = self.binary_operation(left, token, right);
        }
    }
    fn primary(&mut self) -> Box<dyn Any> {
        let mut token = self.lexer.get_next_token();

        // ... code to handle functions and constants ...

        match token.token_type {
            TokenType::Number => Box::new(token.value.parse::<f32>().unwrap()),
            TokenType::StringLiteral => Box::new(token.value.clone()),
            TokenType::Identifier => {
                if let Some(value) = self.variables.get(&token.value) {
                    Box::new(value.clone())
                } else {
                    println!(
                        "Undefined variable '{}' at line {} and column {}",
                        token.value, token.line, token.column
                    );
                    panic!(); // Don't ask me why I did this, I'm desperate.
                }
            }
            TokenType::Punctuation if token.value == "(" => {
                let expression_value = self.expression();
                self.variables.clear();
                let next_token = self.lexer.get_next_token();
                if next_token.token_type != TokenType::Punctuation || next_token.value != ")" {
                    println!(
                        "Expected ')' after expression at line {} and column {}",
                        next_token.line, next_token.column
                    );
                    Box::new(())
                } else {
                    expression_value
                }
            }
            // ... code to handle 'let' and 'if' keywords ...
            TokenType::Operator if token.value == "-" => {
                let mut is_negative = false;
                while token.token_type == TokenType::Operator && token.value == "-" {
                    is_negative = !is_negative;
                    token = self.lexer.get_next_token();
                }
                if token.token_type == TokenType::Number {
                    Box::new(
                        (if is_negative { -1.0 } else { 1.0 })
                            * token.value.parse::<f32>().unwrap(),
                    )
                } else if token.token_type == TokenType::Punctuation && token.value == "(" {
                    let expression_value = self.expression();
                    Box::new(
                        (if is_negative { -1.0 } else { 1.0 })
                            * *expression_value.downcast::<f32>().unwrap(),
                    )
                } else {
                    self.lexer.unget_token(token);
                    let expression_value = self.expression();
                    Box::new(
                        (if is_negative { -1.0 } else { 1.0 })
                            * *expression_value.downcast::<f32>().unwrap(),
                    )
                }
            }
            _ => {
                println!(
                    "Invalid expression at line {} and column {}",
                    token.line, token.column
                );
                Box::new(())
            }
        }
    }
    fn binary_operation(
        &self,
        left: Box<dyn Any>,
        operator_token: Token,
        right: Box<dyn Any>,
    ) -> Box<dyn Any> {
        if operator_token.token_type == TokenType::Operator {
            match operator_token.value.as_str() {
                "+" => {
                    if let (Ok(l), Ok(r)) = (left.downcast::<f32>(), right.downcast::<f32>()) {
                        return Box::new(*l + *r);
                    } else if let (Ok(ls), Ok(rs)) =
                        (left.downcast::<String>(), right.downcast::<String>())
                    {
                        return Box::new((*ls).clone() + &rs);
                    } else {
                        println!(
                            "Invalid operands for '+' operator at line {} and column {}",
                            operator_token.line, operator_token.column
                        );
                        return Box::new(());
                    }
                }
                "-" => {
                    if let (Ok(l), Ok(r)) = (left.downcast::<f32>(), right.downcast::<f32>()) {
                        return Box::new(*l - *r);
                    } else {
                        println!(
                            "Invalid operands for '-' operator at line {} and column {}",
                            operator_token.line, operator_token.column
                        );
                        return Box::new(());
                    }
                }
                // ... repeat for other operators ...
                _ => {
                    println!(
                        "Invalid operator '{}' at line {} and column {}",
                        operator_token.value, operator_token.line, operator_token.column
                    );
                    return Box::new(());
                }
            }
        } else {
            println!(
                "Expected operator at line {} and column {}",
                operator_token.line, operator_token.column
            );
            return Box::new(());
        }
    }

    fn concatenate_values(&self, left: Value, right: Value) -> Value {
        match (left, right) {
            (Value::Str(l), Value::Str(r)) => Value::Str(l + &r),
            (Value::Str(l), Value::Float(r)) => Value::Str(l + &r.to_string()),
            (Value::Float(l), Value::Str(r)) => Value::Str(l.to_string() + &r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
        }
    }
}
