use crate::parser::ArithExpr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompExpr {
    Num(i32),
    Bool(bool),
    Plus(Box<CompExpr>, Box<CompExpr>),
    Mult(Box<CompExpr>, Box<CompExpr>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DesugarError {
    UnrecognizedBinaryOperation,
}

fn desugar_bin_op(op: String, left: ArithExpr, right: ArithExpr) -> Result<CompExpr, DesugarError> {
    match op.as_str() {
        "+" => Ok(CompExpr::Plus(
            Box::new(desugar(left)?),
            Box::new(desugar(right)?),
        )),
        "*" => Ok(CompExpr::Mult(
            Box::new(desugar(left)?),
            Box::new(desugar(right)?),
        )),
        "-" => Ok(CompExpr::Plus(
            Box::new(desugar(left)?),
            Box::new(CompExpr::Mult(
                Box::new(CompExpr::Num(-1)),
                Box::new(desugar(right)?),
            )),
        )),
        _ => Err(DesugarError::UnrecognizedBinaryOperation),
    }
}

pub fn desugar(exp: ArithExpr) -> Result<CompExpr, DesugarError> {
    match exp {
        ArithExpr::Num(number) => Ok(CompExpr::Num(number)),
        ArithExpr::BinOp(op, left, right) => desugar_bin_op(op, *left, *right),
        ArithExpr::Bool(b) => Ok(CompExpr::Bool(b)),
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

    #[test]
    fn negative_number() {
        let expr = ArithExpr::Num(-3);
        let res = desugar(expr);
        assert_eq!(res, Ok(CompExpr::Num(-3)));
    }

    #[test]
    fn plus() {
        let expr = ArithExpr::BinOp(
            "+".to_string(),
            Box::new(ArithExpr::Num(1)),
            Box::new(ArithExpr::Num(2)),
        );
        let res = desugar(expr);
        assert_eq!(
            res,
            Ok(CompExpr::Plus(
                Box::new(CompExpr::Num(1)),
                Box::new(CompExpr::Num(2)),
            ))
        );
    }

    #[test]
    fn mult() {
        let expr = ArithExpr::BinOp(
            "*".to_string(),
            Box::new(ArithExpr::Num(1)),
            Box::new(ArithExpr::Num(2)),
        );
        let res = desugar(expr);
        assert_eq!(
            res,
            Ok(CompExpr::Mult(
                Box::new(CompExpr::Num(1)),
                Box::new(CompExpr::Num(2)),
            ))
        );
    }

    #[test]
    fn minus() {
        let expr = ArithExpr::BinOp(
            "-".to_string(),
            Box::new(ArithExpr::Num(4)),
            Box::new(ArithExpr::Num(2)),
        );
        let res = desugar(expr);
        assert_eq!(
            res,
            Ok(CompExpr::Plus(
                Box::new(CompExpr::Num(4)),
                Box::new(CompExpr::Mult(
                    Box::new(CompExpr::Num(-1)),
                    Box::new(CompExpr::Num(2)),
                ))
            ))
        );
    }

    #[test]
    fn bool_true() {
        let expr = ArithExpr::Bool(true);
        let res = desugar(expr);
        assert_eq!(res, Ok(CompExpr::Bool(true)));
    }

    #[test]
    fn bool_false() {
        let expr = ArithExpr::Bool(false);
        let res = desugar(expr);
        assert_eq!(res, Ok(CompExpr::Bool(false)));
    }
}
