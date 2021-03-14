use crate::parser::ArithExpr;

#[derive(Debug, PartialEq, Eq)]
pub enum CompExpr {
    Num(i32),
    Plus(Box<CompExpr>, Box<CompExpr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DesugarError {
    Unimplemented,
}

pub fn desugar(exp: ArithExpr) -> Result<CompExpr, DesugarError> {
    match exp {
        _ => Err(DesugarError::Unimplemented),
    }
}
