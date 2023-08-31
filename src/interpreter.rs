use crate::{
    ast::*,
    error::LoxError,
    token::{Object, TokenType},
};

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evluate(&expr.left).unwrap().into();
        let right = self.evluate(&expr.right).unwrap();

        if let Object::Num(left_num) = left {
            if let Object::Num(right_num) = right {
                match expr.operator.tty {
                    TokenType::PLUS => return Ok(Object::Num(left_num + right_num)),
                    TokenType::MINUS => return Ok(Object::Num(left_num - right_num)),
                    TokenType::SLASH => return Ok(Object::Num(left_num / right_num)),
                    TokenType::STAR => return Ok(Object::Num(left_num * right_num)),
                    TokenType::GREATER => {
                        return Ok(if left_num > right_num {
                            Object::True
                        } else {
                            Object::False
                        })
                    }
                    TokenType::GREATER_EQUAL => {
                        return Ok(if left_num >= right_num {
                            Object::True
                        } else {
                            Object::False
                        })
                    }
                    TokenType::LESS => {
                        return Ok(if left_num < right_num {
                            Object::True
                        } else {
                            Object::False
                        })
                    }
                    TokenType::LESS_EQUAL => {
                        return Ok(if left_num <= right_num {
                            Object::True
                        } else {
                            Object::False
                        })
                    }
                    TokenType::BANG_EQUAL => {
                        return Ok(if left_num != right_num {
                            Object::True
                        } else {
                            Object::False
                        })
                    }
                    TokenType::EQUAL_EQUAL => {
                        return Ok(if left_num == right_num {
                            Object::True
                        } else {
                            Object::False
                        })
                    }
                    _ => {}
                }
            }
        }
        Err(LoxError {
            msg: String::from("RuntimeError: Invalid binary expression."),
            line: 0,
        })
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evluate(&expr.right).unwrap();

        if expr.operator.tty == TokenType::MINUS {
            if let Object::Num(num) = right {
                return Ok(Object::Num(-num));
            }
        }
        if expr.operator.tty == TokenType::BANG {
            if let Object::False = right {
                return Ok(Object::True);
            }
            if let Object::True = right {
                return Ok(Object::False);
            }
        }
        Err(LoxError {
            msg: String::from("RuntimeError: The expression after mius expected boolean."),
            line: 0,
        })
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) {
        self.evluate(&stmt.expression);
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) {
        let value = self.evluate(&stmt.expression).unwrap();
        println!("{}", self.stringify(&value));
    }
}

impl Interpreter {
    fn evluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        match expr {
            Expr::Binary(n) => n.accept(self),
            Expr::Grouping(n) => n.accept(self),
            Expr::Literal(n) => n.accept(self),
            Expr::Unary(n) => n.accept(self),
        }
    }
    pub fn new() -> Self {
        Self {}
    }
    pub fn interpret(&self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&self, stmt: &Stmt) {
        match stmt {
            Stmt::ExpressionStmt(n) => n.accept(self),
            Stmt::PrintStmt(n) => n.accept(self),
        }
    }

    fn stringify(&self, obj: &Object) -> String {
        match obj {
            Object::Num(n) => {
                let str = n.to_string();
                if str.ends_with(".0") {
                    return str[0..str.len() - 2].to_string();
                }
                return str;
            }
            Object::Str(s) => s.clone(),
            Object::Nil => String::from("nil"),
            Object::True => String::from("true"),
            Object::False => String::from("false"),
        }
    }
}
