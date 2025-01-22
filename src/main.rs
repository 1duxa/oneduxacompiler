use std::{fs::File, io::Read};

use gen::gen::Generator;
use lex::lex::Lexer;
use parser::parser::Parser;

mod gen;
mod lex;
mod parser;

fn main() {
    let path = r"C:\Users\User\Desktop\code\langs\rust\oneduxalang\src\test.duxa";
    let file = File::open(path);
    if file.is_err() {panic!("Can't find file {}",path)}
    let mut buf = String::new();
    if file.unwrap().read_to_string(&mut buf).is_err(){
        panic!("Failed to read the file {}",path);
    }
    let lexer = Lexer(buf);
    let tokens = lexer.lexify();
    let mut parser = Parser(tokens);
    let program =  parser.parse_prog();
    let mut generator = Generator::new(program);
    println!("{}",generator.gen_prog())
}
