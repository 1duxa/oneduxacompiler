mod gen {
    use crate::lex::lex::Token;

    // TODO: move to the generator
    fn compute_bin_expression(operator: Token, lhs: i64, rhs: i64) -> i64 {
        match operator {
            Token::PLUS => {}
            Token::SUB => {}
            Token::MUL => {}
            Token::DIV => {}
            _ => {
                panic!("Unknown operator")
            }
        }
        lhs + rhs
    }
}
