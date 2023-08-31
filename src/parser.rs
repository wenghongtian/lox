use crate::{
    ast::{
        BinaryExpr, Expr, ExpressionStmt, GroupingExpr, LiteralExpr, PrintStmt, Stmt, UnaryExpr,
        VarStmt, VariableExpr,
    },
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token(TokenType::VAR) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::IDENTIFIER, String::from("Expect variable name."))?;
        let mut initializer = None;
        if self.match_token(TokenType::EQUAL) {
            initializer = Some(self.expression()?);
        }
        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after variable declaration."),
        );
        Ok(Stmt::VarStmt(VarStmt { name, initializer }))
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_token(TokenType::PRINT) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after value."),
        )?;
        Ok(Stmt::PrintStmt(PrintStmt { expression: value }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after value."),
        )?;
        Ok(Stmt::ExpressionStmt(ExpressionStmt { expression: value }))
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
                left: Box::new(expr),
                operator,
                right: Box::new(right),
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
                left: Box::new(expr),
                right: Box::new(right),
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
            let bin_expr = BinaryExpr {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
            expr = Expr::Binary(bin_expr);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_token(TokenType::SLASH) || self.match_token(TokenType::STAR) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                right: Box::new(right),
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
                right: Box::new(right),
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

        if self.match_token(TokenType::IDENTIFIER) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous().clone(),
            }));
        }

        if self.match_token(TokenType::NIL) {
            return Ok(Expr::Literal(LiteralExpr { value: Object::Nil }));
        }

        if self.match_token(TokenType::STRING) || self.match_token(TokenType::NUMBER) {
            let value = self.previous().literal.clone();
            return Ok(Expr::Literal(LiteralExpr {
                value: value.unwrap(),
            }));
        }

        if self.match_token(TokenType::LEFT_PAREN) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RIGHT_PAREN,
                "Expect ')' after expression.".to_string(),
            )?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
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
