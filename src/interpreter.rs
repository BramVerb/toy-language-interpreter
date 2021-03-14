use crate::desugarer::CompExpr;

#[derive(Debug, PartialEq, Eq)]
pub enum ValExpr {
    Num(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterpError {
    Unimplemented,
}

pub fn interp(exp: CompExpr) -> Result<ValExpr, InterpError> {
    match exp {
        _ => Err(InterpError::Unimplemented),
    }
}
