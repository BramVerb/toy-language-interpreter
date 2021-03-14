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

#[cfg(test)]
mod tests {

    use crate::desugar;
    use crate::desugarer::CompExpr;
    use crate::parser::ArithExpr;

    #[test]
    fn number() {
        let expr = ArithExpr::Num(1);
        let res = desugar(expr);
        assert_eq!(res, Ok(CompExpr::Num(1)));
    }
}
