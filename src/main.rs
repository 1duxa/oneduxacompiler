use lex::lex::Lexer;

mod lex;
mod parser;

fn main() {
    let lexer = Lexer::new("make variable = 514;".into());
    println!("{:?}", lexer.lexify());
}
