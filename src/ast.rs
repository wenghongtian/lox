use crate::token::{Object, Token};

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
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

pub trait Visitor<T> {
    fn visit_binary_expr(bin_expr: BinaryExpr) -> T;
    fn visit_grouping_expr(gouping_expr: GroupingExpr) -> T;
    fn visit_literal_expr(literal_expr: LiteralExpr) -> T;
    fn visit_unary_expr(unary_expr: UnaryExpr) -> T;
}
