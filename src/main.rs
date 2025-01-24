use std::{fs::File, io::{Read, Write}};
use modules::{
    lex::lex::Lexer, 
    parser::parser::Parser,
   // gen::gen::Generator
};

mod modules;

fn main() {
    let path = r"src/test.duxa";
    let file = File::open(path);
    if file.is_err() {panic!("Can't find file {}",path)}
    let mut buf = String::new();
    if file.unwrap().read_to_string(&mut buf).is_err(){
        panic!("Failed to read the file {}",path);
    }
    let lexer = Lexer(buf);
    let tokens = lexer.lexify();
    println!("{:?}",tokens);
    let mut parser = Parser(tokens);
    let program =  parser.parse_prog();
    //let mut generator = Generator::new(program);

    println!("{:?}",program);
   /*  let output_file = "./out.asm";
    let output_file =File::create(output_file);
    if output_file.is_err() { panic!("Failed to create output, {:?}",output_file)}
    let output_write_result = output_file.unwrap().write_all(generator.gen_prog().as_bytes());
    output_write_result.unwrap()*/
}
