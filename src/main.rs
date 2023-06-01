#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    OpenBrace(char),
    CloseBrace(char),
    OpenParen(char),
    CloseParen(char),
    OpenBrack(char),
    CloseBrack(char),
    Quote(char), // can denote either open or closed
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
                '"' => return Ok(Token::Quote('"')),
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

    fn next_token(&mut self) -> Result<Token, &'static str> {
        // simple case: match current token
        let token: Token;
        match self.ch {
            '{'  => token = Token::OpenBrace('{'),
            '}'  => token = Token::CloseBrace('}'),
            '('  => token = Token::OpenParen('('),
            ')'  => token = Token::CloseParen(')'),
            '['  => token = Token::OpenBrack('['),
            ']'  => token = Token::CloseBrack(']'),
            '"'  => token = Token::Quote('"'),
            ':'  => token = Token::Colon(':'),
            ','  => token = Token::Comma(','),
            '\0' => token = Token::Eof,
            _    => return Err("Unrecognized token")
        }
        self.read_char();
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_next_token() {
        let input = String::from("{\"\":\"\",\"\":{},\"\":[]}");
        let expected = vec![
            crate::Token::OpenBrace('{'),
            crate::Token::Quote('"'),
            crate::Token::Quote('"'),
            crate::Token::Colon(':'),
            crate::Token::Quote('"'),
            crate::Token::Quote('"'),
            crate::Token::Comma(','),
            crate::Token::Quote('"'),
            crate::Token::Quote('"'),
            crate::Token::Colon(':'),
            crate::Token::OpenBrace('{'),
            crate::Token::CloseBrace('}'),
            crate::Token::Comma(','),
            crate::Token::Quote('"'),
            crate::Token::Quote('"'),
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
}

fn main() {
    println!("Hello, world!");
}
