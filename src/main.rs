mod lex;
mod parser;
use lex::lex::Lexer;
use parser::parser::Interpreter;

fn main() {
    // Read an input
    let mut input = String::new();
    println!("Hello: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let mut interpreter = Interpreter::new(input);
    interpreter.parse();
}
