#[allow(unused)]
pub mod parser {
    use crate::modules::lex::lex::Token;
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct StatementVariable {
        pub ident: String,
        pub expr: Expression,
    }
    #[derive(Debug)]
    pub struct IfStatement {
        expression: Expression,
        if_scope: Option<Vec<Statement>>,
        else_if_scope: Option<Box<IfStatement>>,
        else_scope: Option<Vec<Statement>>,   
    }
    #[derive(Debug)]
    pub enum Statement {
        MAKE(StatementVariable),
        ASSIGN(StatementVariable),
        IF(IfStatement),
        SCOPE(Option<Vec<Statement>>)
    }
    #[derive(Debug)]
    pub struct BinExpr {
        pub lhs: Box<Expression>,
        pub rhs: Box<Expression>,
        pub operator: Token,
    }

    #[derive(Debug)]
    pub enum Expression {
        BINEXPR(BinExpr),
        TERM(Term),
        STRING(String),
    }
    #[derive(Debug)]
    pub enum Term {
        INTLIT(i64),
        IDENT(String),
        PAREN(Box<Expression>),
        STRING(String)
    }
    pub struct Parser(pub VecDeque<Token>);
    impl Parser {
        fn consume_ident(&mut self) -> Option<String> {
            if let Some(ident) = self.0.pop_front() {
                match ident {
                    Token::IDENT(name) => return Some(name.to_string()),
                    _ => None,
                }
            } else {
                panic!("Expected ident")
            }
        }
        fn consume_int_lit(&mut self) -> Option<i64> {
            if let Some(num) = self.0.pop_front() {
                match num {
                    Token::NUM(number) => return Some(number),
                    _ => None,
                }
            } else {
                panic!("Expected ident")
            }
        }
        fn consume_discard(&mut self, expected: Token) {
            if let Some(t) = self.0.pop_front() {
                if t != expected {
                    panic!("Expected token: {:?}", expected)
                }
            } else {
                panic!("Expected token: {:?}", expected)
            }
        }
        fn peek_token(&self,idx:usize) -> Option<&Token> {
            self.0.get(idx)
        }
        fn parse_term(&mut self) -> Term {
            if let Some(curr_token) = self.0.pop_front() {
                match curr_token {
                    Token::NUM(number) => return Term::INTLIT(number),
                    Token::IDENT(ident) => return Term::IDENT(ident),
                    Token::OPAREN => {
                        let expr = self.parse_expression(0);
                        self.consume_discard(Token::CPAREN);
                        return Term::PAREN(Box::new(expr));
                    },
                    Token::QUOT => {
                        let str_token = self.consume_ident();
                        if str_token.is_none() {
                            panic!("Expected string");
                        }
                        self.consume_discard(Token::QUOT);
                        return Term::STRING(str_token.unwrap());
                    }
                    _ => unimplemented!("term"),
                }
            } else {
                panic!("Bad term")
            }
        }
        fn parse_expression(&mut self, min_prec: i8) -> Expression {
            let mut lhs = match self.parse_term() {
                Term::INTLIT(n) => Expression::TERM(Term::INTLIT(n)),
                Term::IDENT(name) => Expression::TERM(Term::IDENT(name)),
                Term::PAREN(expr) => *expr,
                Term::STRING(str) =>return Expression::STRING(str),
            };
            while let Some(operator) = self.0.front() {
                let prec: Option<i8>;
                prec = bin_prec(operator);
                if (prec.is_none() || prec.unwrap() < min_prec) {
                    break;
                }
                let operator = self.0.pop_front().unwrap();
                let next_min_prec = prec.unwrap() + 1;
                let rhs = self.parse_expression(next_min_prec);

                lhs = Expression::BINEXPR(BinExpr {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    operator,
                });
            }
            lhs
        }

        fn parse_make_statement(&mut self) -> StatementVariable {
            self.consume_discard(Token::MAKE);
            if let Some(name) = self.consume_ident() {
                self.consume_discard(Token::EQ);

                let stat = StatementVariable {
                    expr: self.parse_expression(0),
                    ident: name,
                };
                self.consume_discard(Token::SEMI);
                stat
            } else {
                panic!("Expected ident");
            }
        }
        fn parse_assign_statement(&mut self) -> StatementVariable {

            self.consume_discard(Token::EQ);
            StatementVariable {
                expr: self.parse_expression(0),
                ident:self.consume_ident().unwrap(),
            }
        }
        fn parse_if_statement(&mut self) -> IfStatement {
            self.consume_discard(Token::IF);
            let expr = self.parse_expression(0); // this should if .. {}
            let scope = self.parse_scope();
            let mut if_statment = IfStatement { expression : expr, if_scope: scope,else_scope:None,else_if_scope:None};
            
            if self.peek_token(0).is_some() && *self.peek_token(0).unwrap() == Token::ELSE && self.peek_token(1).is_some() && *self.peek_token(1).unwrap() == Token::IF{
                self.consume_discard(Token::ELSE);
                self.consume_discard(Token::IF);
                if_statment.else_if_scope = Some(Box::new(self.parse_if_statement()));
            } 
            if self.peek_token(0).is_some() && *self.peek_token(0).unwrap() == Token::ELSE{
                self.consume_discard(Token::ELSE);
                if_statment.else_scope = self.parse_scope();
            }
            if_statment
        }
        fn parse_scope(&mut self) -> Option<Vec<Statement>> {
            self.consume_discard(Token::OCPAREN); 
            let mut statements: Vec<Statement> = Vec::new();
            
            while let Some(curr_token) = self.peek_token(0) {
                if *curr_token == Token::CCPAREN {
                    self.consume_discard(Token::CCPAREN);
                    break;
                }
                match curr_token {
                    Token::MAKE => statements.push(Statement::MAKE(self.parse_make_statement())),
                    Token::IDENT(_) => {
                        statements.push(Statement::ASSIGN(self.parse_assign_statement()))
                    }
                    Token::IF => statements.push(Statement::IF(self.parse_if_statement())),
                    Token::OCPAREN => statements.push(Statement::SCOPE(self.parse_scope())),
                    _ => panic!("Unknown statement in scope"),
                }
            }
        
            if statements.is_empty() {
                None
            } else {
                Some(statements)
            }
        }
        
        pub fn parse_prog(&mut self) -> Vec<Statement> {
            let mut statments: Vec<Statement> = Vec::new();
            while self.0.len() > 0 {
                if let Some(token) = self.peek_token(0) {
                    match token {
                        Token::MAKE => statments.push(Statement::MAKE(self.parse_make_statement())),
                        Token::IDENT(_) => {
                            statments.push(Statement::ASSIGN(self.parse_assign_statement()))
                        }
                        Token::IF => {
                            statments.push(Statement::IF(self.parse_if_statement()))
                        }
                        Token::OCPAREN => {
                            statments.push(Statement::SCOPE(self.parse_scope()))

                        }
                        _ => panic!("Unknown statement"),
                    }
                }
            }
            return statments;
        }
    }
    fn bin_prec(operator: &Token) -> Option<i8> {
        match operator {
            Token::PLUS => Some(1),
            Token::SUB => Some(1),
            Token::MUL => Some(2),
            Token::DIV => Some(2),
            _ => None,
        }
    }
}
