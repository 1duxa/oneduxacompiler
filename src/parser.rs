pub mod parser {
    use std::collections::VecDeque;

    use crate::lex::lex::Token;

    #[derive(Debug)]
    #[allow(unused)]
    pub struct Statement {
        ident: String,
        expr: i64,
    }
    #[allow(unused)]
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
        fn consume_discard(&mut self, expected: Token) {
            if let Some(t) = self.0.pop_front() {
                if t != expected {
                    panic!("consume discard fumbled")
                }
                println!("discarded")
            } else {
                panic!("No value provided");
            }
        }
        fn parse_statement(&mut self) -> Statement {
            if let Some(name) = self.consume_ident() {
                //let eq = self.0.pop_front();
                //if !eq.is_none() && eq.unwrap() == Token::EQ {
                self.consume_discard(Token::EQ);
                //self.consume_discard(Token::EQ);

                if let Some(expr) = self.0.pop_front() {
                    match expr {
                        Token::NUM(num) => {
                            return Statement {
                                expr: num,
                                ident: name,
                            };
                        }
                        _ => panic!("Expected expr"),
                    }
                } else {
                    panic!("Expected expr");
                }
                /* } else {
                    panic!("Expected '='");
                }*/
            } else {
                panic!("Expected ident");
            }
        }
        pub fn parse_prog(&mut self) {
            while self.0.len() > 0 {
                if let Some(token) = self.0.pop_front() {
                    if token == Token::MAKE {
                        println!("{:#?}", self.parse_statement());
                    }
                }
            }
        }
    }
}
