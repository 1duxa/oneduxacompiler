#[allow(unused)]
pub mod gen {
    use std::collections::HashSet;
    use crate::{
        modules::lex::lex::Token,
        modules::parser::parser::{BinExpr, Expression, Statement, StatementVariable, Term},
    };
    enum AssemblyStatments {
        SysCallWrite,

        
    }
    pub struct Generator {
        program: Vec<Statement>,
        start_section: String,
        data_section: String, // Will store strings there
        stack_location: i64,
        variables: HashSet<Variable>,
    }
    
    #[derive(PartialEq, Eq,Hash)]
    pub struct Variable {
        name: String,
        location: i64,
    }
    impl Generator {
        pub fn new(program:Vec<Statement>) -> Self {
            Self { program, start_section: "".into(), data_section: "".into(), stack_location: 1, variables: HashSet::new() }
        }
        fn add_string_to_data_section(&mut self, value: &str) -> String {
            // Generate a unique label for the string
            let label = format!("str_{}", self.data_section.len());
            self.data_section += &format!("{} db '{}', 0\n", label, value);
            label
        }
        fn gen_bin_expr(&mut self, expr: &BinExpr)  {
            self.gen_expr(expr.rhs.as_ref());
            self.gen_expr(expr.lhs.as_ref());
            self.pop("rax");
            self.pop("rbx");
            self.start_section += format!("{}{}",Self::operator_start_section(&expr.operator), "\n").as_str();
            self.push("rax");
        }
        fn gen_make_string_expr(&mut self, value: &String) {
            let label = self.add_string_to_data_section(value);
            self.start_section += format!("     lea rax, [{}]{}", label, "\n").as_str();
            self.push("rax");
        }
        fn gen_term(&mut self, term: &Term)  {
            match term {
                Term::IDENT(ident) => {
                    let var = self.variables.iter().find(|var| ident == &var.name);
                    if var.is_none() {
                        panic!("Variable not found {}",ident);
                    }
                    else {
                        let offset = format!("QWORD [rsp + {}{}" , (self.stack_location - var.unwrap().location) * 8 , "]\n");
                        self.push(&offset);
                    }
                }
                Term::INTLIT(int) => {
                    self.start_section+= format!("     mov rax, {}{}", int , "\n").as_str();
                    self.push("rax");
                }
                Term::PAREN(expr) => {
                    self.gen_expr(expr.as_ref());
                }
                Term::STRING(str) => {
                    let label = self.add_string_to_data_section(str);
                    self.start_section += format!("     lea rax, [{}]{}", label, "\n").as_str();
                    self.push("rax");
                }
            }
        }
        fn gen_expr(&mut self, expr: &Expression) {
            match expr {
                Expression::BINEXPR(bin_expr) => self.gen_bin_expr(bin_expr),
                Expression::TERM(term) => self.gen_term(term),
                Expression::STRING(str) => self.gen_make_string_expr(str),
            }
        }
        fn gen_statement(&mut self, statement: &Statement)  {
            match statement {
                Statement::MAKE(stat) => {self.gen_statement_make(stat);}
                Statement::ASSIGN(stat) => {unimplemented!()},
                Statement::IF(stat) => {unimplemented!()},
            }
        }
        fn gen_statement_make(&mut self, statement_var: &StatementVariable) {
            let var = self.variables.iter().find(|var| statement_var.ident == var.name);
            if var.is_some() {
                panic!("Variable already exists {}",statement_var.ident);
            }
            else {
                self.gen_expr(&statement_var.expr);
                self.variables.insert(Variable{name:statement_var.ident.clone(),location:self.stack_location});
            }
        }
        pub fn gen_prog(&mut self) -> String {
            self.start_section += "global _start\n_start:\n";
            self.data_section += "section .data\n";
            let statements: Vec<_> = self.program.drain(..).collect();

            for statement in statements {
                self.gen_statement(&statement); 
            }
            self.start_section += "     mov rax, 60\n";
            self.start_section += "     mov rdi, 0\n";
            self.start_section += "     syscall\n";
            format!("{}\n{}",self.data_section,self.start_section)
        }
        fn operator_start_section(operator:&Token) -> String {
            match operator {
                    Token::PLUS =>"     add rax, rbx".into(),
                    Token::SUB => "     sub rax, rbx".into(),
                    Token::MUL => "     mul rbx".into(),
                    Token::DIV => "     div rbx".into(),
                    _ => panic!("Bad operator, {:#?}",operator),
                }            
        }
        fn push(&mut self, reg: &str) {
            self.start_section += format!("     push {}{}", reg, "\n").as_str();
            self.stack_location += 1;
        }
        fn pop(&mut self, reg: &str) {
            self.start_section += format!("     pop {}{}", reg, "\n").as_str();
            self.stack_location -= 1;
        }
    }
}
