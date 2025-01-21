pub mod lex {
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub enum Token {
        MAKE,
        IDENT(String),
        NUM(i64),
        EQ,
        SEMI,
    }
    ///
    ///  make variable = 5;
    ///
    ///
    ///
    pub struct Lexer(String);

    impl Lexer {
        pub fn new(code: String) -> Self {
            Self(code)
        }
        #[allow(unstable_features)]
        pub fn lexify(&self) -> Vec<Token> {
            let mut tokens: VecDeque<Token> = VecDeque::new();
            let mut symbols = self.0.chars().collect::<VecDeque<char>>();

            while symbols.len() > 0 {
                let curr_symbol = symbols.pop_front().unwrap();
                match curr_symbol {
                    '0'..='9' => {
                        let mut curr_num = curr_symbol.to_string().parse::<i64>().unwrap();

                        while !symbols.front().is_none() && symbols.front().unwrap().is_numeric() {
                            let next_num = symbols.pop_front();
                            if !next_num.unwrap().is_numeric() {
                                symbols.push_front(next_num.unwrap());
                                break;
                            } else {
                                curr_num = curr_num * 10
                                    + next_num.unwrap().to_string().parse::<i64>().unwrap();
                            }
                        }
                        tokens.push_back(Token::NUM(curr_num));
                    }
                    'a'..='z' | 'A'..='Z' => {
                        let mut curr_ident: Vec<char> = Vec::new();
                        curr_ident.push(curr_symbol);
                        while !symbols.front().is_none()
                            && Self::is_in_a_to_z(symbols.front().unwrap())
                        {
                            let next_char = symbols.pop_front();
                            curr_ident.push(next_char.unwrap());
                        }
                        tokens.push_back(Token::IDENT(curr_ident.iter().collect::<String>()));
                    }
                    ';' => {
                        tokens.push_back(Token::SEMI);
                    }
                    '=' => {
                        tokens.push_back(Token::EQ);
                    }
                    ' ' => continue,
                    _ => panic!("Dont know who you are"),
                }
            }
            tokens.into()
        }
        fn is_in_a_to_z(c: &char) -> bool {
            ('a'..='z').contains(c) || ('A'..='Z').contains(c)
        }
    }
}
