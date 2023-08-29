use crate::{
    ast::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    error::LoxError,
    lox::Lox,
    token::{Object, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/**
*  expression → literal
              | unary
              | binary
              | grouping ;

   literal    → NUMBER | STRING | "true" | "false" | "nil" ;
   grouping   → "(" expression ")" ;
   unary      → ( "-" | "!" ) expression ;
   binary     → expression operator expression ;
   operator   → "==" | "!=" | "<" | "<=" | ">" | ">="
              | "+"  | "-"  | "*" | "/" ;
*
*/

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_token(TokenType::BANG_EQUAL) || self.match_token(TokenType::EQUAL_EQUAL) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                operator,
                right: Box::from(right),
            })
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.match_token(TokenType::GREATER)
            || self.match_token(TokenType::GREATER_EQUAL)
            || self.match_token(TokenType::LESS)
            || self.match_token(TokenType::LESS_EQUAL)
        {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                right: Box::from(right),
                operator,
            })
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.match_token(TokenType::MINUS) || self.match_token(TokenType::PLUS) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                right: Box::from(right),
                operator,
            })
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_token(TokenType::SLASH) || self.match_token(TokenType::STAR) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::from(expr),
                right: Box::from(right),
                operator,
            })
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(TokenType::BANG) || self.match_token(TokenType::MINUS) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::from(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.match_token(TokenType::FALSE) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::False,
            }));
        }

        if self.match_token(TokenType::TRUE) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Object::True,
            }));
        }

        if self.match_token(TokenType::NIL) {
            return Ok(Expr::Literal(LiteralExpr { value: Object::Nil }));
        }

        if self.match_token(TokenType::STRING) || self.match_token(TokenType::NUMBER) {
            let value = self.previous().literal.clone();
            return Ok(Expr::Literal(LiteralExpr { value: value.unwrap() }));
        }

        if self.match_token(TokenType::LEFT_PAREN) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RIGHT_PAREN,
                "Expect ')' after expression.".to_string(),
            )?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::from(expr),
            }));
        }

        Err(self.error(
            self.peek().clone(),
            String::from("Unrecongnized token during parsed."),
        ))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().tty == TokenType::SEMICOLON {
                return;
            }

            match self.peek().tty {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => {}
                _ => {}
            }
            self.advance();
        }
    }

    fn consume(&mut self, tty: TokenType, message: String) -> Result<Token, LoxError> {
        if self.check(tty) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek().clone(), message))
    }

    fn error(&self, token: Token, message: String) -> LoxError {
        LoxError {
            msg: message,
            line: token.line,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous().clone();
    }

    fn check(&self, tty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().tty == tty
    }

    fn is_at_end(&self) -> bool {
        self.peek().tty == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn match_token(&mut self, tty: TokenType) -> bool {
        if self.check(tty) {
            self.advance();
            return true;
        }
        return false;
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
