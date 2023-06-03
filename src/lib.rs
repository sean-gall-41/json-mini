#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    OpenBrace(char),
    CloseBrace(char),
    OpenParen(char),
    CloseParen(char),
    OpenBrack(char),
    CloseBrack(char),
    Colon(char),
    Comma(char),
    StringLiteral(String),
    NumericLiteral(String), // we'll parse later
    Eof,
}

#[derive(Debug, Default)]
pub struct Lexer {
    pub input: String, // what if the json file is massive, like over a few MB?
    pub pos: usize,
    pub read_pos: usize,
    pub ch: char
}

impl Lexer {
    pub fn from(s: String) -> Self {
        let mut lex = Self {
            input: s,
            ..Default::default()
        };
        lex.read_char();
        return lex;
    }

    pub fn read_char(&mut self) {
        if self.read_pos >= self.input.len() { self.ch = '\0'; }
        else { self.ch = self.input.chars().nth(self.read_pos).unwrap_or('\0'); }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_pos >= self.input.len() { '\0' }
        else { self.input.chars().nth(self.read_pos).unwrap_or('\0') }
    }

    pub fn next_numeric_literal(&mut self) -> Result<Token, String> {
        let mut literal = String::from(self.ch);
        loop {
            let next = self.peek_char();
            if !next.is_digit(10) { break; }
            literal.push(next);
            self.read_char();
        }
        Ok(Token::NumericLiteral(literal))
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        // simple case: match current token
        let token: Token;
        match self.ch {
            '{'  => token = Token::OpenBrace('{'),
            '}'  => token = Token::CloseBrace('}'),
            '('  => token = Token::OpenParen('('),
            ')'  => token = Token::CloseParen(')'),
            '['  => token = Token::OpenBrack('['),
            ']'  => token = Token::CloseBrack(']'),
            ':'  => token = Token::Colon(':'),
            ','  => token = Token::Comma(','),
            '\0' => token = Token::Eof,
            '"' => {
                let mut literal = String::from("");
                loop {
                    self.read_char();
                    if self.ch == '"' { break; }
                    literal.push(self.ch);
                }
                token = Token::StringLiteral(literal);
            },
            '0'..='9' => {
                token = self.next_numeric_literal().unwrap_or(Token::Eof);
            },
            '-' => {
                if !self.peek_char().is_digit(10) {
                    return Err(format!("Invalid token '-' found at position {}", self.read_pos));
                }
                token = self.next_numeric_literal().unwrap_or(Token::Eof);
            },
            _    => return Err(String::from("Unrecognized token"))
        }
        self.read_char();
        Ok(token)
    }
}



#[cfg(test)]
mod tests {
    use super::{Token, Lexer};
    #[test]
    fn test_next_token() {
        let input = String::from(r#"{"field_1":89,"field_2":{},"field_3":[]}"#);
        let expected = vec![
            Token::OpenBrace('{'),
            Token::StringLiteral(String::from("field_1")),
            Token::Colon(':'),
            Token::NumericLiteral(String::from("89")),
            Token::Comma(','),
            Token::StringLiteral(String::from("field_2")),
            Token::Colon(':'),
            Token::OpenBrace('{'),
            Token::CloseBrace('}'),
            Token::Comma(','),
            Token::StringLiteral(String::from("field_3")),
            Token::Colon(':'),
            Token::OpenBrack('['),
            Token::CloseBrack(']'),
            Token::CloseBrace('}')
        ];
        let mut lex = Lexer::from(input);
        for expected_token in expected.iter() {
            match lex.next_token() {
                Err(_) => break,
                Ok(token) => {
                    assert_eq!(token, *expected_token);
                }
            }
        }
    }

    #[test]
    fn test_next_token_neg_sign_invalid() {
        let input = String::from(r#"{"field":-a}"#);
        let expected = vec![
            Token::OpenBrace('{'),
            Token::StringLiteral(String::from("field")),
            Token::Colon(':')
        ];
        let mut lex = Lexer::from(input);
        for expected_token in expected.iter() {
            match lex.next_token() {
                Err(err_str) => {
                    assert_eq!(err_str, String::from("Invalid token '-' found at position 9"));
                },
                Ok(token) => {
                    assert_eq!(token, *expected_token);
                }
            }
        }
    }

    #[test]
    fn test_next_token_neg_sign_valid() {
        let input = String::from(r#"{"field":-314159}"#);
        let expected = vec![
            Token::OpenBrace('{'),
            Token::StringLiteral(String::from("field")),
            Token::Colon(':'),
            Token::NumericLiteral(String::from("-314159")),
        ];
        let mut lex = Lexer::from(input);
        for expected_token in expected.iter() {
            match lex.next_token() {
                Err(_) => break,
                Ok(token) => {
                    assert_eq!(token, *expected_token);
                }
            }
        }
    }
}

