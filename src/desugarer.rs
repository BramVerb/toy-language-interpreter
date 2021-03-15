use crate::parser::ArithExpr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompExpr {
    Num(i32),
    Plus(Box<CompExpr>, Box<CompExpr>),
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
        _ => Err(DesugarError::UnrecognizedBinaryOperation),
    }
}

pub fn desugar(exp: ArithExpr) -> Result<CompExpr, DesugarError> {
    match exp {
        ArithExpr::Num(number) => Ok(CompExpr::Num(number)),
        ArithExpr::BinOp(op, left, right) => desugar_bin_op(op, *left, *right),
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
}
