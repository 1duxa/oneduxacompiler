pub mod lex {
    use std::collections::VecDeque;
    #[derive(Debug, PartialEq)]
    #[allow(unused)]
    pub enum Token {
        IDENT(String),
        NUM(i64),
        MAKE,    // make
        INT,     // int type
        STR,     // string type
        BOOL,    // bool type
        EQ,      // ;
        SEMI,    // ;
        PLUS,    // +
        MUL,     // *
        DIV,     // /
        SUB,     // -
        OPAREN,  // (
        CPAREN,  // )
        OCPAREN, // {
        CCPAREN, // }
        OSPAREN, // [
        CSPAREN, // ]
        QUOT,    // "
        IF,
        ELSE,
    }

    pub struct Lexer(pub String);

    impl Lexer {
        #[allow(unstable_features)]
        pub fn lexify(&self) -> VecDeque<Token> {
            use std::collections::VecDeque;

            let mut tokens: VecDeque<Token> = VecDeque::new();
            let mut symbols = self.0.chars().collect::<VecDeque<char>>();
            let initial_len = symbols.len();
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
                            && Self::is_a_to_z_and_num(symbols.front().unwrap())
                        {
                            let next_char = symbols.pop_front();
                            curr_ident.push(next_char.unwrap());
                        }
                        let ident = curr_ident.iter().collect::<String>();
                        match ident.as_str() {
                            "make" => tokens.push_back(Token::MAKE),
                            "int" => tokens.push_back(Token::INT),
                            "str" => tokens.push_back(Token::STR),
                            "bool" => tokens.push_back(Token::BOOL),
                            "if" => tokens.push_back(Token::IF),
                            "else" => tokens.push_back(Token::ELSE),
                            _ => tokens.push_back(Token::IDENT(ident.to_string())),
                        }
                    }
                    ';' => tokens.push_back(Token::SEMI),
                    '=' => tokens.push_back(Token::EQ),
                    '+' => tokens.push_back(Token::PLUS),
                    '-' => tokens.push_back(Token::SUB),
                    '*' => tokens.push_back(Token::MUL),
                    '/' => tokens.push_back(Token::DIV),
                    '(' => tokens.push_back(Token::OPAREN),
                    ')' => tokens.push_back(Token::CPAREN),
                    '{' => tokens.push_back(Token::OCPAREN),
                    '}' => tokens.push_back(Token::CCPAREN),
                    '[' => tokens.push_back(Token::OSPAREN),
                    ']' => tokens.push_back(Token::CSPAREN),
                    '"' => {
                        tokens.push_back(Token::QUOT);
                        let mut curr_ident: Vec<char> = Vec::new();
                        while !symbols.front().is_none()
                            && Self::is_a_to_z_and_num_and_whitespace(symbols.front().unwrap())
                        {
                            let next_char = symbols.pop_front();
                            curr_ident.push(next_char.unwrap());
                        }
                        let ident = curr_ident.iter().collect::<String>();
                        tokens.push_back(Token::IDENT(ident.to_string()));
                        let close_quote = symbols.pop_front().unwrap();
                        if close_quote != '"'{
                            panic!("expected close \"");
                        } else {
                            tokens.push_back(Token::QUOT);
                        }


                    },
                    ' ' | '\n' | '\r'=> continue,
                    _ => panic!(
                        "Dont know who you are, mister {}\nchar at: {}",
                        curr_symbol,
                        initial_len - symbols.len() - 1
                    ),
                }
            }
            tokens.into()
        }
        fn is_a_to_z_and_num(c: &char) -> bool {
            ('a'..='z').contains(c) || ('A'..='Z').contains(c) || ('0'..'9').contains(c)
        }
        fn is_a_to_z_and_num_and_whitespace(c: &char) -> bool {
            ('a'..='z').contains(c) || ('A'..='Z').contains(c) || ('0'..'9').contains(c) || *c == ' '
        }
    }
}
