use crate::{
    ast::{Expr, GroupingExpr, LiteralExpr},
    token::Object,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn visitLiteralExpr(expr: LiteralExpr) -> Object {
        expr.value
    }
    pub fn visitGroupingExpr(expr: GroupingExpr) -> Object {}
    fn evaluate(expr: Expr) {}
}
