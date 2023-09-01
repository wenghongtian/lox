use crate::{
    ast::*,
    environment::Environment,
    error::LoxError,
    token::{Object, TokenType},
};

pub struct Interpreter {
    env: Environment,
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left).unwrap().into();
        let right = self.evaluate(&expr.right).unwrap();

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
        Err(LoxError::new_runtime(String::from(
            "RuntimeError: Invalid binary expression.",
        )))
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right).unwrap();

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
        Err(LoxError::new_runtime(String::from(
            "RuntimeError: The expression after mius expected boolean.",
        )))
    }

    fn visit_variable_expr(&self, unary_expr: &VariableExpr) -> Result<Object, LoxError> {
        self.env.get(&unary_expr.name)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), LoxError> {
        let mut value = Object::Nil;
        if stmt.initializer.is_some() {
            let ini = stmt.initializer.as_ref().clone().unwrap();
            value = self.evaluate(&ini)?;
        }

        self.env.define(stmt.name.lexeme.clone(), value);
        Ok(())
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        match expr {
            Expr::Binary(n) => n.accept(self),
            Expr::Grouping(n) => n.accept(self),
            Expr::Literal(n) => n.accept(self),
            Expr::Unary(n) => n.accept(self),
            Expr::Variable(n) => n.accept(self),
        }
    }
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }
    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        match stmt {
            Stmt::ExpressionStmt(n) => n.accept(self),
            Stmt::PrintStmt(n) => n.accept(self),
            Stmt::VarStmt(n) => n.accept(self),
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
