mod gen {
    use crate::{
        lex::lex::Token,
        parser::parser::{BinExpr, Expression, Statement, Term},
    };

    pub struct Generator(Vec<Statement>, String);

    impl Generator {
        fn gen_bin_expr(&self, expr: &BinExpr) -> String {
            self.gen_expr(expr.lhs.as_ref());
            self.gen_expr(expr.rhs.as_ref());
        }
        fn gen_term(&self, term: &Term) -> String {
            match term {
                Term::IDENT(ident) => {}
                Term::INTLIT(int) => {}
                Term::PAREN(expr) => {}
            }
        }
        fn gen_expr(&self, expr: &Expression) -> String {
            match expr {
                Expression::BINEXPR(bin_expr) => self.gen_bin_expr(bin_expr),
                Expression::TERM(term) => self.gen_term(term),
            }
        }
        fn gen_statment(&self, statement: &Statement) -> String {
            match statement {
                Statement::ASSIGN(stat) => {}
                Statement::MAKE(stat) => {}
            }

            "".into()
        }
        pub fn gen_prog(mut self) -> String {
            self.1 += "global _start\n_start:\n";
            for statement in &self.0 {
                self.1 += &self.gen_statment(&statement);
            }
            self.1 += "     mov rax, 60\n";
            self.1 += "     mov rdi, 0\n";
            self.1 += "     syscall\n";
            self.1
        }
    }
}
