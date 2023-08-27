use crate::{
    error::LoxError,
    token::{Object, Token, TokenType},
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.advance();
            match c {
                '(' => self.add_token(TokenType::LEFT_PAREN),
                ')' => self.add_token(TokenType::RIGHT_PAREN),
                '{' => self.add_token(TokenType::LEFT_BRACE),
                '}' => self.add_token(TokenType::RIGHT_BRACE),
                ',' => self.add_token(TokenType::COMMA),
                '.' => self.add_token(TokenType::DOT),
                '-' => self.add_token(TokenType::MINUS),
                '+' => self.add_token(TokenType::PLUS),
                ';' => self.add_token(TokenType::SEMICOLON),
                '*' => self.add_token(TokenType::STAR),
                '!' => {
                    let tty = if self.match_char('=') {
                        TokenType::BANG_EQUAL
                    } else {
                        TokenType::BANG
                    };
                    self.add_token(tty)
                }
                '=' => {
                    let tty = if self.match_char('=') {
                        TokenType::EQUAL_EQUAL
                    } else {
                        TokenType::EQUAL
                    };
                    self.add_token(tty)
                }
                '<' => {
                    let tty = if self.match_char('=') {
                        TokenType::LESS_EQUAL
                    } else {
                        TokenType::LESS
                    };
                    self.add_token(tty)
                }
                '>' => {
                    let tty = if self.match_char('=') {
                        TokenType::GREATER_EQUAL
                    } else {
                        TokenType::GREATER
                    };
                    self.add_token(tty)
                }
                '/' => {
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::SLASH)
                    }
                }
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                'o' if self.match_char('r') => self.add_token(TokenType::OR),
                n if self.is_digit(n) => {
                    self.number();
                }
                '"' => self.string()?,
                c if self.is_alpha(c) => self.identifier(),
                _ => {
                    return Err(LoxError::new(
                        self.line,
                        String::from("Unexpected character."),
                    ))
                }
            }
            self.start = self.current;
        }
        self.add_token(TokenType::EOF);
        Ok(&self.tokens)
    }

    fn identifier(&mut self) {
        {
            while self.is_alpha_number(self.peek()) {
                self.advance();
            }
        }
        self.add_token(TokenType::IDENTIFIER);
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let num: String = self.source[self.start..self.current].iter().collect();
        let num = num.parse::<f64>().unwrap();
        self.add_token_object(TokenType::NUMBER, Some(Object::Num(num)));
        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::new(
                self.line,
                String::from("Unterminated string."),
            ));
        }
        self.advance();
        let str: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::STRING, Some(Object::Str(str)));
        Ok(())
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' && c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alpha_number(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if *self.source.get(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            *self.source.get(self.current).unwrap()
        }
    }

    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }
    fn add_token(&mut self, tty: TokenType) {
        self.add_token_object(tty, None);
    }
    fn add_token_object(&mut self, tty: TokenType, literal: Option<Object>) {
        let str: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(tty, str, None, self.line))
    }
}
