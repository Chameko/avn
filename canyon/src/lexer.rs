use crate::token::{Token, TokenType, Literal, SYMBOLS, KEYWORDS};
use std::iter::Peekable;
use std::vec::IntoIter;
use std::{fs, io};

pub struct Lexer {
    raw_data: Peekable<IntoIter<char>>,
    line: usize,
    filename: String,
}

impl Lexer {
    pub fn from_file(path: &str) -> io::Result<Self> {
        Ok(Lexer {
            raw_data: fs::read_to_string(path)?.chars().collect::<Vec<_>>().into_iter().peekable(),
            line: 1,
            filename: String::from(path),
        })
    }

    pub fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
            line: 1,
            filename: String::from("text"),
        }
    }

    fn get_next_char_while(&mut self, raw_token: &mut String, cond: fn(char) -> bool) {
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    raw_token.push(*c);
                    self.raw_data.next();
                }
                _ => break
            }
        }
    }

    fn is_identifier(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let token: Result<Token, String>;

        let first_char: char;

        loop {
            match self.raw_data.next() {
                Some(c) if c == '\n' => {self.line = self.line + 1; continue}
                Some(c) if c.is_whitespace() => continue,
                Some(c) => {
                    first_char = c;
                    break;
                }
                None => return None
            }
        }

        if Self::is_identifier(first_char) && !first_char.is_numeric() {
            let mut name = first_char.to_string();
            self.get_next_char_while(&mut name, Self::is_identifier);

            if !KEYWORDS.contains(&&name[..]) {
                token = Ok(Token {
                    line: self.line,
                    token: TokenType::Identifier(name),
                    filename: self.filename.clone(),
                })
            } else {
                token = Ok(Token {
                    line: self.line,
                    token: TokenType::Keyword(name),
                    filename: self.filename.clone(),
                })
            };
        } else if first_char.is_numeric() {
            let mut value = first_char.to_string();
            self.get_next_char_while(&mut value, |c| c.is_numeric());
        
            // If the value returns Err during parse(), it probably means the integer is out of range.
            token = match value.parse() {
                Ok(i) => Ok(Token{ token: TokenType::Literal(Literal::Int(i)), line: self.line, filename: self.filename.clone()}),
                Err(_) => Err(format!("Integer literal {} is invalid", value)),
            }
        } else if first_char == '"' {
            let mut value = String::new();
            self.get_next_char_while(&mut value, |c| c != '"');
            
            // Our get_next_char_while function doesn't eat the token if it doesn't meet the condition, so we have to explicitly eat the ending `"`
            self.raw_data.next();

            token = Ok( Token{ token: TokenType::Literal(Literal::String(value)), line: self.line, filename: self.filename.clone() })
        } else {
            let mut raw = first_char.to_string();
            loop {
                if let Some(peek) = self.raw_data.peek() {
                    raw.push(*peek);
                } else {
                    // We reached the end of the program.
                    break;
                }
        
                if SYMBOLS.contains(&&raw[..]) {
                    self.raw_data.next();
                } else {
                    raw.pop();
                    break;
                }
            }
        
            token = match &raw[..] {
                // Ignore comments until newline
                s if s == "//" => {
                    self.get_next_char_while(&mut String::new(), |c| c != '\n');
                    // Once we reached the end of the comment, we still need to return a token. So simply call `next`. I now realize that if we have at least 128 lines of comments (Rust's recursion limit) our program will crash.
                    self.next()?
                }
                s if SYMBOLS.contains(&s) => Ok(Token{ token: TokenType::Symbol(raw), line: self.line, filename: self.filename.clone()}),
                _ => Err(format!("Unknown token: {}", raw)),
            }
        }

        Some(token)
    }
}