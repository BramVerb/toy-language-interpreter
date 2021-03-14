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

#[cfg(test)]
mod tests {

    use crate::interp;
    use crate::interpreter::CompExpr;
    use crate::interpreter::ValExpr;

    #[test]
    fn number() {
        let expr = CompExpr::Num(1);
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Num(1)));
    }
}
