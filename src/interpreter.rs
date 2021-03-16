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

pub fn as_bool(expr: CompExpr) -> Result<bool, InterpError> {
    let interpreteted = interp(expr)?;
    if let ValExpr::Bool(num) = interpreteted {
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

pub fn interp_not(expr: CompExpr) -> Result<ValExpr, InterpError> {
    let b = as_bool(expr)?;
    Ok(ValExpr::Bool(!b))
}

pub fn interp(exp: CompExpr) -> Result<ValExpr, InterpError> {
    match exp {
        CompExpr::Num(number) => Ok(ValExpr::Num(number)),
        CompExpr::Bool(b) => Ok(ValExpr::Bool(b)),
        CompExpr::Plus(left, right) => interp_plus(*left, *right),
        CompExpr::Mult(left, right) => interp_mult(*left, *right),
        CompExpr::Not(expr) => interp_not(*expr),
    }
}

#[cfg(test)]
mod tests {

    use crate::interp;
    use crate::interpreter::CompExpr;
    use crate::interpreter::InterpError;
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

    #[test]
    fn plus_bool_should_err() {
        let expr = CompExpr::Plus(Box::new(CompExpr::Num(-5)), Box::new(CompExpr::Bool(false)));
        let res = interp(expr);
        assert_eq!(res, Err(InterpError::InvalidType));
    }

    #[test]
    fn mult_bool_should_err() {
        let expr = CompExpr::Mult(Box::new(CompExpr::Num(-5)), Box::new(CompExpr::Bool(false)));
        let res = interp(expr);
        assert_eq!(res, Err(InterpError::InvalidType));
    }

    #[test]
    fn not_true() {
        let expr = CompExpr::Not(Box::new(CompExpr::Bool(true)));
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Bool(false)));
    }

    #[test]
    fn not_false() {
        let expr = CompExpr::Not(Box::new(CompExpr::Bool(false)));
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Bool(true)));
    }

    #[test]
    fn not_number() {
        let expr = CompExpr::Not(Box::new(CompExpr::Num(1)));
        let res = interp(expr);
        assert_eq!(res, Err(InterpError::InvalidType));
    }

    #[test]
    fn if_true() {
        let expr = CompExpr::If(
            Box::new(CompExpr::Bool(true)),
            Box::new(CompExpr::Num(3)),
            Box::new(CompExpr::Num(4)),
        );
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Num(3)));
    }

    #[test]
    fn if_false() {
        let expr = CompExpr::If(
            Box::new(CompExpr::Bool(false)),
            Box::new(CompExpr::Num(3)),
            Box::new(CompExpr::Num(4)),
        );
        let res = interp(expr);
        assert_eq!(res, Ok(ValExpr::Num(4)));
    }

    #[test]
    fn if_number() {
        let expr = CompExpr::If(
            Box::new(CompExpr::Num(1)),
            Box::new(CompExpr::Num(3)),
            Box::new(CompExpr::Num(4)),
        );
        let res = interp(expr);
        assert_eq!(res, Err(InterpError::InvalidType));
    }
}
