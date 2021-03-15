use crate::desugarer::CompExpr;

#[derive(Debug, PartialEq, Eq)]
pub enum ValExpr {
    Num(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterpError {
    InvalidType,
}

pub fn as_number(expr: CompExpr) -> Result<i32, InterpError> {
    let interpreteted = interp(expr)?;
    if let ValExpr::Num(num) = interpreteted {
        Ok(num)
    } else {
        Err(InterpError::InvalidType)
    }
}

pub fn interp_plus(left: CompExpr, right: CompExpr) -> Result<ValExpr, InterpError> {
    let l = as_number(left)?;
    let r = as_number(right)?;
    Ok(ValExpr::Num(l + r))
}

pub fn interp_mult(left: CompExpr, right: CompExpr) -> Result<ValExpr, InterpError> {
    let l = as_number(left)?;
    let r = as_number(right)?;
    Ok(ValExpr::Num(l * r))
}

pub fn interp(exp: CompExpr) -> Result<ValExpr, InterpError> {
    match exp {
        CompExpr::Num(number) => Ok(ValExpr::Num(number)),
        CompExpr::Bool(b) => Ok(ValExpr::Bool(b)),
        CompExpr::Plus(left, right) => interp_plus(*left, *right),
        CompExpr::Mult(left, right) => interp_mult(*left, *right),
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

    #[test]
    fn negative_number() {
        let expr = CompExpr::Num(-10);
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Num(-10)));
    }

    #[test]
    fn plus() {
        let expr = CompExpr::Plus(Box::new(CompExpr::Num(1)), Box::new(CompExpr::Num(2)));
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Num(3)));
    }

    #[test]
    fn mult() {
        let expr = CompExpr::Mult(Box::new(CompExpr::Num(2)), Box::new(CompExpr::Num(3)));
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Num(6)));
    }

    #[test]
    fn negative_mult() {
        let expr = CompExpr::Mult(Box::new(CompExpr::Num(-2)), Box::new(CompExpr::Num(3)));
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Num(-6)));
    }

    #[test]
    fn double_negative_mult() {
        let expr = CompExpr::Mult(Box::new(CompExpr::Num(-5)), Box::new(CompExpr::Num(-4)));
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Num(20)));
    }
}
