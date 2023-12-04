use super::token::{DFunction, Token};
use crate::lex::token::TokenType;

pub struct Lexer {
    text: String,               // Contains the value of actual Token
    pos: usize,                 // Contains the position of the actual Token
    line: usize,                // Contains the line of the actual Token
    column: usize,              // Contains the column of the actual Token
    current_char: Option<char>, // Contains the actual character
    read_tokens: Vec<Token>,    // Contains the tokens that have been read but not yet consumed
}

impl Lexer {
    pub fn new(text: String) -> Self {
        // Constructor
        let current_char = text.chars().next();
        Lexer {
            text,
            pos: 0,
            line: 1,
            column: 1,
            current_char,
            read_tokens: Vec::new(),
        }
    }

    /// Advances the `pos` pointer and sets the `current_char` field.
    fn advance(&mut self) {
        self.pos += 1;
        self.column += 1;
        self.current_char = self.text.chars().nth(self.pos);
    }

    /// Skips all whitespace characters until a non-whitespace character is found.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    /// Returns the next character in the input string without consuming it.
    pub fn peek(&self) -> Option<char> {
        self.text.chars().nth(self.pos + 1)
    }
    /// Function that trash one token
    pub fn unget_token(&mut self, token: Token) {
        self.read_tokens.insert(0, token);
    }

    pub fn get_next_token(&mut self) -> Token {
        if !self.read_tokens.is_empty() {
            return self.read_tokens.remove(0);
        }
        while let Some(c) = self.current_char {
            match c {
                c if c.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                c if c.is_digit(10) => return self.number(),
                c if c.is_alphabetic() => {
                    let token = self.keyword();
                    if token.token_type == TokenType::FunctionKeyword {
                        return Token::new(
                            TokenType::FunctionDeclaration,
                            token.value,
                            self.line,
                            self.column,
                        );
                    } else {
                        return token;
                    }
                }
                '"' => return self.string_literal(),
                '+' | '-' | '*' | '/' | '%' | '^' | '.' | '@' => {
                    self.advance();
                    return Token::new(TokenType::Operator, c.to_string(), self.line, self.column);
                }
                '(' | ')' | '{' | '}' | '[' | ']' | ':' | ',' => {
                    self.advance();
                    return Token::new(
                        TokenType::Punctuation,
                        c.to_string(),
                        self.line,
                        self.column,
                    );
                }
                ';' => {
                    self.advance();
                    return Token::new(TokenType::EOL, ";".to_string(), self.line, self.column);
                }
                '=' | '<' | '>' | '!' => {
                    let mut token_type = TokenType::Operator;
                    let mut value = c.to_string();
                    if let Some(next_char) = self.peek() {
                        if next_char == '=' {
                            self.advance();
                            value.push('=');
                            token_type = TokenType::ComparisonOperator;
                        }
                    }
                    self.advance();
                    return Token::new(token_type, value, self.line, self.column);
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                    continue;
                }
                _ => {
                    println!(
                        "Invalid character '{}' at line {} and column {}",
                        c, self.line, self.column
                    );
                    break;
                }
            }
        }
        Token::new(TokenType::EOF, "".to_string(), self.line, self.column)
    }

    fn number(&mut self) -> Token {
        let mut result = String::new();
        // Check the positive or negative sign
        if self.current_char == Some('-') {
            result.push(self.current_char.unwrap());
            self.advance();
        }
        // Recognize the whole part of the number
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        // Recognize the decimal part of the number (if it exists)
        if self.current_char == Some('.') {
            result.push(self.current_char.unwrap());
            self.advance();

            while let Some(c) = self.current_char {
                if c.is_digit(10) {
                    result.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }
        // Return the number token
        Token::new(TokenType::Number, result, self.line, self.column)
    }

    /// Parses a string literal token.
    fn string_literal(&mut self) -> Token {
        let mut result = String::new();
        self.advance();
        while let Some(c) = self.current_char {
            if c != '"' {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        if self.current_char.is_none() {
            println!(
                "Unterminated string literal at line {} and column {}",
                self.line, self.column
            );
        }
        self.advance(); // Consume the second '"' to advance to the next token
        Token::new(TokenType::StringLiteral, result, self.line, self.column)
    }
    fn keyword(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = match result.as_str() {
            "let" => TokenType::LetKeyword,
            "function" => TokenType::FunctionKeyword,
            "if" => TokenType::IfKeyword,
            "else" => TokenType::ElseKeyword,
            "in" => TokenType::InKeyword,
            "print" => TokenType::PrintKeyword,
            _ => TokenType::Identifier,
        };

        Token::new(token_type, result, self.line, self.column)
    }

    pub fn function_declaration(&mut self) -> DFunction {
        self.advance();
        self.skip_whitespace();

        if !self.current_char.unwrap().is_alphabetic() {
            println!(
                "Expected a function name at line {} and column {}",
                self.line, self.column
            );
        }

        let mut function_name = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() {
                function_name.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.skip_whitespace();

        if self.current_char.unwrap() != '(' {
            println!(
                "Expected '(' after function name at line {} and column {}",
                self.line, self.column
            );
        }

        let mut parameters = Vec::new();
        self.advance();
        while self.current_char.unwrap() != ')' {
            if !self.current_char.unwrap().is_alphabetic() {
                println!(
                    "Expected a parameter name at line {} and column {}",
                    self.line, self.column
                );
            }

            let mut parameter_name = String::new();
            while let Some(c) = self.current_char {
                if c.is_alphanumeric() {
                    parameter_name.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            parameters.push(parameter_name);
            self.skip_whitespace();

            if self.current_char.unwrap() == ',' {
                self.advance();
                self.skip_whitespace();
            }
        }
        self.advance();

        let next_token = self.get_next_token();
        if next_token.token_type != TokenType::FLinq || next_token.value != "=>" {
            println!(
                "Expected '=>' after function parameters at line {} and column {}",
                self.line, self.column
            );
        }

        let mut expression_tokens = Vec::new();
        let mut next_token = self.get_next_token();
        while next_token.token_type != TokenType::Semicolon
            && next_token.token_type != TokenType::EOF
        {
            expression_tokens.push(next_token);
            next_token = self.get_next_token();
        }

        DFunction::new(
            expression_tokens,
            function_name,
            parameters,
            TokenType::FunctionDeclaration,
        )
    }
}
