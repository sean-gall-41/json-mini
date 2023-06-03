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

impl Token {
    fn from(s: String) -> Result<Self, &'static str> {
        if s.len() == 1 {
            // feels kinda weird ngl
            match s.chars().next().unwrap() {
                '{' => return Ok(Token::OpenBrace('{')),
                '}' => return Ok(Token::CloseBrace('}')),
                '(' => return Ok(Token::OpenParen('(')),
                ')' => return Ok(Token::CloseParen(')')),
                '[' => return Ok(Token::OpenBrack('[')),
                ']' => return Ok(Token::CloseBrack(']')),
                ':' => return Ok(Token::Colon(':')),
                ',' => return Ok(Token::Comma(',')),
                '\0' => return Ok(Token::Eof),
                _ => Err("Unrecognized token")
            }
        } else {
            if s.parse::<f64>().is_ok() {
                return Ok(Token::NumericLiteral(s));
            } else {
                return Ok(Token::StringLiteral(s));
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Lexer {
    input: String, // what if the json file is massive, like over a few MB?
    pos: usize,
    read_pos: usize,
    ch: char
}

impl Lexer {
    fn from(s: String) -> Self {
        let mut lex = Self {
            input: s,
            ..Default::default()
        };
        lex.read_char();
        return lex;
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() { self.ch = '\0'; }
        else { self.ch = self.input.chars().nth(self.read_pos).unwrap_or('\0'); }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&mut self) -> char {
        if self.read_pos >= self.input.len() { '\0' }
        else { self.input.chars().nth(self.read_pos).unwrap_or('\0') }
    }

    fn next_numeric_literal(&mut self) -> Result<Token, String> {
        let mut literal = String::from(self.ch);
        loop {
            let next = self.peek_char();
            if !next.is_digit(10) { break; }
            literal.push(next);
            self.read_char();
        }
        Ok(Token::NumericLiteral(literal))
    }

    fn next_token(&mut self) -> Result<Token, String> {
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
    #[test]
    fn test_next_token() {
        let input = String::from(r#"{"field_1":89,"field_2":{},"field_3":[]}"#);
        let expected = vec![
            crate::Token::OpenBrace('{'),
            crate::Token::StringLiteral(String::from("field_1")),
            crate::Token::Colon(':'),
            crate::Token::NumericLiteral(String::from("89")),
            crate::Token::Comma(','),
            crate::Token::StringLiteral(String::from("field_2")),
            crate::Token::Colon(':'),
            crate::Token::OpenBrace('{'),
            crate::Token::CloseBrace('}'),
            crate::Token::Comma(','),
            crate::Token::StringLiteral(String::from("field_3")),
            crate::Token::Colon(':'),
            crate::Token::OpenBrack('['),
            crate::Token::CloseBrack(']'),
            crate::Token::CloseBrace('}')
        ];
        let mut lex = crate::Lexer::from(input);
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
            crate::Token::OpenBrace('{'),
            crate::Token::StringLiteral(String::from("field")),
            crate::Token::Colon(':')
        ];
        let mut lex = crate::Lexer::from(input);
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
            crate::Token::OpenBrace('{'),
            crate::Token::StringLiteral(String::from("field")),
            crate::Token::Colon(':'),
            crate::Token::NumericLiteral(String::from("-314159")),
        ];
        let mut lex = crate::Lexer::from(input);
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

fn main() {
    println!("Hello, world!");
}

