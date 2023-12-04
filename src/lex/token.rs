#[derive(PartialEq)]
pub enum TokenType {
    DFunction(Box<DFunction>),
    FLinq,
    ComparisonOperator,
    Number,
    StringLiteral,
    FunctionDeclaration,
    LetKeyword,
    IfKeyword,
    ElseKeyword,
    PrintKeyword,
    InKeyword,
    FunctionKeyword,
    Operator,
    Punctuation,
    Identifier,
    EOL,
    EOF,
    Semicolon,
    Separator,
}
#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub column: usize,
    pub line: usize,
}

impl Token {
    // Constructor
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            value,
            line,
            column,
        }
    }
}
#[derive(PartialEq)]
pub struct DFunction {
    pub expression: Vec<Token>,
    pub value: String,
    pub parameters: Vec<String>,
    pub token_type: TokenType,
}

impl DFunction {
    pub fn new(
        expression: Vec<Token>,
        value: String,
        parameters: Vec<String>,
        token_type: TokenType,
    ) -> Self {
        let mut expression = expression;
        expression.push(Token::new(TokenType::EOL, ";".to_string(), 0, 0));
        DFunction {
            expression,
            value,
            parameters,
            token_type,
        }
    }
}
