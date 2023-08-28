use crate::{
    error::LoxError,
    token::{Object, Token},
};

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
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

trait Visitor<T> {
    type ExprType;
    fn visit(&self) -> Result<Self::ExprType, LoxError>;
}

pub struct BinaryExpr {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Token,
}
pub struct GroupingExpr {
    expression: Box<Expr>,
}
pub struct LiteralExpr {
    value: Object,
}
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}
