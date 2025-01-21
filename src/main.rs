use lex::lex::Lexer;
use parser::parser::Parser;

mod lex;
mod parser;

fn main() {
    let lexer = Lexer("make variable = 514;".into());
    let mut parser = Parser(lexer.lexify());
    parser.parse_prog();
}
