use crate::token::TokenType;

pub struct Lexer {}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {}
    }

    pub fn next_token(&mut self) -> TokenType {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::Lexer;

    #[test]
    fn next_token() {
        let input = r###"=+(){},;"###;
        let tokens = vec![
            TokenType::ASSIGN,
            TokenType::PLUS,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LBRACE,
            TokenType::COMMA,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ];

        let mut lexer = Lexer::new(String::from(input));
        for token in tokens {
            let next_token = lexer.next_token();
            assert_eq!(token, next_token);
        }
    }
}
