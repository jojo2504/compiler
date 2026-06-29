use crate::token::{Keyword, Token};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let mut advance = true;
        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'+' => Token::PLUS,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b'-' => Token::MINUS,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NOTEQ
                } else {
                    Token::BANG
                }
            }
            b'*' => Token::ASTERISK,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'/' => Token::SLASH,
            0 => Token::EOF,
            _ => {
                advance = !advance;
                if self.ch.is_ascii_alphabetic() || self.ch == b'_' {
                    Keyword::read_keyword(&self.read_identifier())
                } else if self.ch.is_ascii_digit() {
                    Token::INT(self.read_number())
                } else {
                    Token::ILLEGAL((self.ch as char).to_string())
                }
            }
        };

        if advance {
            self.read_char();
        }
        token
    }

    pub fn read_char(&mut self) {
        if self.read_position as usize >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position] as u8;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        return self.input[position..self.position].iter().collect();
    }

    pub fn skip_whitespace(&mut self) {
        while matches!(self.ch as char, ' ' | '\t' | '\n' | '\r') {
            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return self.input[position..self.position].iter().collect();
    }

    pub fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position] as u8
        }
    }
}
