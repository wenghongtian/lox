use crate::{
    error::LoxError,
    token::{Object, Token},
};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Variable(VariableExpr),
}

pub enum Stmt {
    ExpressionStmt(ExpressionStmt),
    PrintStmt(PrintStmt),
    VarStmt(VarStmt),
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, bin_expr: &BinaryExpr) -> Result<T, LoxError>;
    fn visit_grouping_expr(&self, gouping_expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visit_literal_expr(&self, literal_expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_unary_expr(&self, unary_expr: &UnaryExpr) -> Result<T, LoxError>;
    fn visit_variable_expr(&self, unary_expr: &VariableExpr) -> Result<T, LoxError>;
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<T, LoxError>;
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<T, LoxError>;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<T, LoxError>;
}

impl ExpressionStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    pub fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_print_stmt(self)
    }
}

impl VarStmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_var_stmt(self)
    }
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }
}
impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }
}
impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }
}
impl VariableExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visit_variable_expr(self)
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub operator: Token,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}
#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Object,
}
#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct VariableExpr {
    pub name: Token,
}
