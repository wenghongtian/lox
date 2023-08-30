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
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, bin_expr: &BinaryExpr) -> Result<T, LoxError>;
    fn visit_grouping_expr(&self, gouping_expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visit_literal_expr(&self, literal_expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_unary_expr(&self, unary_expr: &UnaryExpr) -> Result<T, LoxError>;
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }
}
impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }
}
impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
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
