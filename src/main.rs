use gen::gen::Generator;
use lex::lex::Lexer;
use parser::parser::Parser;

mod gen;
mod lex;
mod parser;

fn main() {
    let lexer = Lexer(
    "
    make int variable = 514 - 32;
    make int second = 8*3;
    make string gg = \"s\"
    
    ".into());
    let tokens = lexer.lexify();
    let mut parser = Parser(tokens);
    let program =  parser.parse_prog();
    let mut generator = Generator::new(program);
    println!("{}",generator.gen_prog())
}
