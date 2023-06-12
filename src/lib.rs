// TODO: add boolean token
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    WhiteSpace(char),
    OpenBrace(char),
    CloseBrace(char),
    OpenParen(char),
    CloseParen(char),
    OpenBrack(char),
    CloseBrack(char),
    Colon(char),
    Comma(char),
    StringLiteral(String),
    NumericLiteral(String),
    BoolLiteral(String),
    Eof,
}

impl Token {
    pub fn extract_value(self) -> String {
        match self {
            Token::WhiteSpace(val)  => String::from(val),
            Token::OpenBrace(val) => String::from(val),
            Token::CloseBrace(val) => String::from(val),
            Token::OpenParen(val) => String::from(val),
            Token::CloseParen(val) => String::from(val),
            Token::OpenBrack(val) => String::from(val),
            Token::CloseBrack(val) => String::from(val),
            Token::Colon(val) => String::from(val),
            Token::Comma(val) => String::from(val),
            Token::StringLiteral(val) => val,
            Token::NumericLiteral(val) => val,
            Token::BoolLiteral(val) => val,
            Token::Eof => String::from(""),
        }
    }
}

const IGNORE_WS: bool = true;
const NO_IGNORE_WS: bool = false;

#[derive(Debug, Default)]
pub struct JSONLexer {
    pub input: String, // what if the json file is massive, like over a few MB?
    pub lexed_input: Vec<Token>,
    pub pos: usize,
    pub read_pos: usize,
    pub ch: char,
    pub ignore_ws: bool,
}

impl JSONLexer {
    pub fn from(s: String, ignore_ws: bool) -> Self {
        let mut lex = Self { input: s,
            lexed_input: vec![],
            pos: Default::default(),
            read_pos: Default::default(),
            ch: Default::default(),
            ignore_ws: ignore_ws
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

    pub fn read_n_chars(&mut self, n: usize) {
        if self.read_pos + n >= self.input.len() {
            eprintln!("The Requested number of characters to read is beyond the end of the input buffer!");
            self.ch = '\0';
        }
        else {
            self.ch = self.input.chars().nth(self.read_pos+n).unwrap_or('\0');
            self.pos = self.read_pos; // this might be a bug, I'm not sure
            self.read_pos += n;
        }
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_pos >= self.input.len() { '\0' }
        else { self.input.chars().nth(self.read_pos).unwrap_or('\0') }
    }

    pub fn peek_n_chars(&mut self, n: usize) -> Result<&str, String> {
        if self.read_pos + n >= self.input.len() {
            Err(String::from("The Requested number of characters to peek is beyond the end of the input buffer!"))
        }
        else { Ok(&self.input[self.read_pos..(self.read_pos+n)]) }
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

    // TODO: test this function because dear-god it needs a refactor
    pub fn next_bool_literal(&mut self) -> Result<Token, String> {
        match self.peek_char() {
            't' => {
                let test_view: &str = self.peek_n_chars(4).unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    ""
                });
                if test_view == "true" {
                    self.read_n_chars(4);
                    Ok(Token::BoolLiteral(String::from("true")))
                } else {
                    Err(format!("Invalid token '{}' found at position {}", String::from(test_view), self.read_pos))
                }
            },
            'f' => {
                let test_view: &str = self.peek_n_chars(5).unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    ""
                });
                if test_view == "false" {
                    self.read_n_chars(5);
                    Ok(Token::BoolLiteral(String::from("false")))
                }
                else { Err(format!("Invalid token '{}' found at position {}", String::from(test_view), self.read_pos)) }
            }
            _ => { Err(String::from("Unrecognized token")) }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        // simple case: match current token
        let token: Token;
        match self.ch {
            '\t'|'\n'|'\r'|' ' => {
                if self.ignore_ws {
                    self.read_char();
                    return self.next_token();
                }
                token = Token::WhiteSpace(self.ch);
            }
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
            _ => {
                token = self.next_bool_literal().unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    Token::Eof
                })
            }
        }
        self.read_char();
        Ok(token)
    }

    pub fn lex(&mut self) -> Result<(), String> {
        loop {
            match self.next_token() {
                Err(e) => return Err(e),
                Ok(token) => {
                    if token == Token::Eof {
                        self.lexed_input.push(token);
                        break;
                    }
                    self.lexed_input.push(token);
                }
            }
        }
        Ok(())
    }

    pub fn tokens_to_string(&self) -> String {
        // do not like the clone here :-/
        self.lexed_input.clone().into_iter()
            .map(|token| token.extract_value())
            .collect::<Vec<String>>()
            .into_iter()
            .fold(String::from(""), |acc, tok| acc + &tok)
    }
}

pub fn minify_json(in_json: String) -> Result<String, String> {
    let mut lexer = JSONLexer::from(in_json, IGNORE_WS);
    match lexer.lex() {
        Ok(_) => (),
        Err(err) => return Err(err)
    }
    Ok(lexer.tokens_to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;
    use super::{Token, JSONLexer};

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
        let mut lex = JSONLexer::from(input, IGNORE_WS);
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
        let mut lex = JSONLexer::from(input, IGNORE_WS);
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
        let mut lex = JSONLexer::from(input, IGNORE_WS);
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
    fn test_next_token_array() {
        let input = String::from(r#"
            {
                "an_array": [1, 2, 3, 4, 5]
            }
        "#);
        let expected = vec![
            Token::OpenBrace('{'),
            Token::StringLiteral(String::from("an_array")),
            Token::Colon(':'),
            Token::OpenBrack('['),
            Token::NumericLiteral(String::from("1")),
            Token::Comma(','),
            Token::NumericLiteral(String::from("2")),
            Token::Comma(','),
            Token::NumericLiteral(String::from("3")),
            Token::Comma(','),
            Token::NumericLiteral(String::from("4")),
            Token::Comma(','),
            Token::NumericLiteral(String::from("5")),
            Token::CloseBrack(']'),
            Token::CloseBrace('}')
        ];
        let mut lex = JSONLexer::from(input, IGNORE_WS);
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
    fn test_next_token_ignore_ws() {
        let input = String::from(r#"
            {
                "field_1": "value_1",
                "field_2": -69,
                "field_3": [],
                "field_4": {}
            }
        "#);
        let expected = vec![
            Token::OpenBrace('{'),
            Token::StringLiteral(String::from("field_1")),
            Token::Colon(':'),
            Token::StringLiteral(String::from("value_1")),
            Token::Comma(','),
            Token::StringLiteral(String::from("field_2")),
            Token::Colon(':'),
            Token::NumericLiteral(String::from("-69")),
            Token::Comma(','),
            Token::StringLiteral(String::from("field_3")),
            Token::Colon(':'),
            Token::OpenBrack('['),
            Token::CloseBrack(']'),
            Token::Comma(','),
            Token::StringLiteral(String::from("field_4")),
            Token::Colon(':'),
            Token::OpenBrace('{'),
            Token::CloseBrace('}'),
            Token::CloseBrace('}'),
        ];
        let mut lex = JSONLexer::from(input, IGNORE_WS);
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
    fn test_next_token_no_ignore_ws() {
        let input = String::from(
r#"
{
    "field_1": "value_1",
    "field_2": -69,
    "field_3": [],
    "field_4": {}
}
"#);
        let expected = vec![
            Token::WhiteSpace('\n'),
            Token::OpenBrace('{'),
            Token::WhiteSpace('\n'),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::StringLiteral(String::from("field_1")),
            Token::Colon(':'),
            Token::WhiteSpace(' '),
            Token::StringLiteral(String::from("value_1")),
            Token::Comma(','),
            Token::WhiteSpace('\n'),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::StringLiteral(String::from("field_2")),
            Token::Colon(':'),
            Token::WhiteSpace(' '),
            Token::NumericLiteral(String::from("-69")),
            Token::Comma(','),
            Token::WhiteSpace('\n'),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::StringLiteral(String::from("field_3")),
            Token::Colon(':'),
            Token::WhiteSpace(' '),
            Token::OpenBrack('['),
            Token::CloseBrack(']'),
            Token::Comma(','),
            Token::WhiteSpace('\n'),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::WhiteSpace(' '),
            Token::StringLiteral(String::from("field_4")),
            Token::Colon(':'),
            Token::WhiteSpace(' '),
            Token::OpenBrace('{'),
            Token::CloseBrace('}'),
            Token::WhiteSpace('\n'),
            Token::CloseBrace('}'),
            Token::WhiteSpace('\n'),
        ];
        let mut lex = JSONLexer::from(input, NO_IGNORE_WS);
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
    fn test_next_token_file() {
        let mut file = fs::File::open("test.json").expect("failed to open 'test.json'");
        let mut input = String::new();
        file.read_to_string(&mut input).expect("Failed to read the file 'test.json'");
        let expected = vec![
            Token::OpenBrace('{'),
            Token::StringLiteral(String::from("field_1")),
            Token::Colon(':'),
            Token::StringLiteral(String::from("value_1")),
            Token::Comma(','),
            Token::StringLiteral(String::from("field_2")),
            Token::Colon(':'),
            Token::NumericLiteral(String::from("5772156649")),
            Token::CloseBrace('}'),
            Token::Eof
        ];
        let mut lex = JSONLexer::from(input, IGNORE_WS);
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

