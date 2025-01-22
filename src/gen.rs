#[allow(unused)]
pub mod gen {
    use std::collections::HashSet;

    use crate::{
        lex::lex::Token,
        parser::parser::{BinExpr, Expression, Statement, StatementVariable, Term},
    };

    pub struct Generator {
        program: Vec<Statement>,
        assembly: String,
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
            Self { program, assembly: "".into(), stack_location: 0, variables: HashSet::new() }
        }
        fn gen_bin_expr(&mut self, expr: &BinExpr)  {
            self.gen_expr(expr.rhs.as_ref());
            self.gen_expr(expr.lhs.as_ref());
            self.pop("rax");
            self.pop("rbx");
            self.assembly += format!("{}{}",Self::operator_assembly(&expr.operator), "\n").as_str();
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
                        let offset = format!("QWORD [rsp + {}{}" , (self.stack_location - var.unwrap().location - 1) * 8 , "]\n");
                        self.push(&offset);
                    }
                }
                Term::INTLIT(int) => {
                    self.assembly+= format!("     mov rax, {}{}", int , "\n").as_str();
                    self.push("rax");
                }
                Term::PAREN(expr) => {
                    self.gen_expr(expr.as_ref());
                }
            }
        }
        fn gen_expr(&mut self, expr: &Expression) {
            match expr {
                Expression::BINEXPR(bin_expr) => self.gen_bin_expr(bin_expr),
                Expression::TERM(term) => self.gen_term(term),
            }
        }
        fn gen_statement(&mut self, statement: &Statement)  {
            match statement {
                Statement::MAKE(stat) => {self.gen_statement_make(stat);}
                Statement::ASSIGN(stat) => {}
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
            self.assembly += "global _start\n_start:\n";
            let statements: Vec<_> = self.program.drain(..).collect();

            for statement in statements {
                self.gen_statement(&statement); 
            }
            self.assembly += "     mov rax, 60\n";
            self.assembly += "     mov rdi, 0\n";
            self.assembly += "     syscall\n";
            self.assembly.clone()
        }
        fn operator_assembly(operator:&Token) -> String {
            match operator {
                    Token::PLUS =>"     add rax, rbx".into(),
                    Token::SUB => "     sub rax, rbx".into(),
                    Token::MUL => "     mul rbx".into(),
                    Token::DIV => "     div rbx".into(),
                    _ => panic!("Bad operator, {:#?}",operator),
                }            
        }
        fn push(&mut self, reg: &str) {
            self.assembly += format!("     push {}{}", reg, "\n").as_str();
            self.stack_location += 1;
        }
        fn pop(&mut self, reg: &str) {
            self.assembly += format!("     pop {}{}", reg, "\n").as_str();
            self.stack_location -= 1;
        }
    }
}
