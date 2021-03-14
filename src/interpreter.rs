use crate::desugarer::CompExpr;

#[derive(Debug, PartialEq, Eq)]
pub enum ValExpr {
    Num(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterpError {
    Unimplemented,
}

pub fn interp_plus(left: CompExpr, right: CompExpr) -> Result<ValExpr, InterpError> {
    let left_val = interp(left)?;
    let right_val = interp(right)?;
    let l = match left_val {
        ValExpr::Num(number) => number,
    };
    let r = match right_val {
        ValExpr::Num(number) => number,
    };
    Ok(ValExpr::Num(l + r))
}

pub fn interp(exp: CompExpr) -> Result<ValExpr, InterpError> {
    match exp {
        CompExpr::Num(number) => Ok(ValExpr::Num(number)),
        CompExpr::Plus(left, right) => interp_plus(*left, *right),
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
}
