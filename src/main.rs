use std::{env, fs::File, io::{Read, Write}};
use modules::{
    lex::lex::Lexer, 
    parser::parser::Parser, to_dot::Dot,
    gen::gen::Generator
};

mod modules;

fn main() {
    // Args
    let args:Vec<String> = env::args().collect();
    let path = args.get(0);
    let to_dot_arg = args.iter().find(|arg| *arg=="--dot");
    if path.is_none() {
        panic!("Please provide PATH argument, cargo run ./src/test.test");
    }
    let path = path.unwrap();
    let file = File::open(path);
    if file.is_err() {panic!("Can't find file {}",path)}
    let mut buf = String::new();
    if file.unwrap().read_to_string(&mut buf).is_err(){
        panic!("Failed to read the file {}",path);
    }
    // Program
    let lexer = Lexer(buf);
    let tokens = lexer.lexify();
    println!("{:?}",tokens);
    let mut parser = Parser(tokens);
    let program =  parser.parse_prog();

    if to_dot_arg.is_some() {
        let dot = Dot(&program);
        let mut dot_file =File::create("ast.dot").unwrap();
        let _ =dot_file.write_all(dot.to_dot().as_bytes());
    }

    let mut generator = Generator::new(program);
    let output_file = "./out.asm";
    let output_file =File::create(output_file);
    if output_file.is_err() { panic!("Failed to create output, {:?}",output_file)}
    let output_write_result = output_file.unwrap().write_all(generator.gen_prog().as_bytes());
    output_write_result.unwrap()
}
