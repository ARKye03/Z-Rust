mod lex;
use lex::lex::Lexer;

fn main() {
    // Read an input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut x = Lexer::new(input);
    let y = x.get_next_token();
    println!("{}", y.value)
}
