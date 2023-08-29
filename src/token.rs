use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,  // (
    RIGHT_PAREN, // )
    LEFT_BRACE,  // {}
    RIGHT_BRACE, // }
    COMMA,       // ,
    DOT,         // .
    MINUS,       //+
    PLUS,        //-
    SEMICOLON,   //;
    SLASH,       // /
    STAR,        // *

    // One or two character tokens.
    BANG,          // !
    BANG_EQUAL,    // !=
    EQUAL,         // =
    EQUAL_EQUAL,   // ==
    GREATER,       // >
    GREATER_EQUAL, // >=
    LESS,          // <
    LESS_EQUAL,    // <=

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tty: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(tty: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        return Token {
            tty,
            lexeme,
            line,
            literal,
        };
    }
    pub fn eof(line: usize) -> Token {
        return Token {
            tty: TokenType::EOF,
            lexeme: String::from(""),
            literal: None,
            line,
        };
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.tty, self.lexeme, self.literal)
    }
}
