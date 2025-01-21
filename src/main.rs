use lex::lex::Lexer;

mod lex;

fn main() {
    let lexer = Lexer::new("make variable = 514;".into());
    println!("{:?}", lexer.lexify());
    println!("Hello, world!");
}
