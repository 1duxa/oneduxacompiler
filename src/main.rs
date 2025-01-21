use lex::lex::Lexer;
use parser::parser::Parser;

mod gen;
mod lex;
mod parser;

fn main() {
    let lexer = Lexer("make variable = 514 - 32;".into());
    let tokens = lexer.lexify();
    println!("{:?}", tokens);
    let mut parser = Parser(tokens);
    println!("{:?}", parser.parse_prog());
}
