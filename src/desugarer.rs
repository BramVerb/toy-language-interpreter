use crate::parser::ArithExpr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CompExpr {
    Num(i32),
    Bool(bool),
    Plus(Box<CompExpr>, Box<CompExpr>),
    Mult(Box<CompExpr>, Box<CompExpr>),
    Not(Box<CompExpr>),
    If(Box<CompExpr>, Box<CompExpr>, Box<CompExpr>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DesugarError {
    UnrecognizedUnaryOperation,
    UnrecognizedBinaryOperation,
    UnrecognizedTernaryOperation,
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

fn desugar_un_op(op: String, expr: ArithExpr) -> Result<CompExpr, DesugarError> {
    match op.as_str() {
        "not" => Ok(CompExpr::Not(Box::new(desugar(expr)?))),
        _ => Err(DesugarError::UnrecognizedUnaryOperation),
    }
}

fn desugar_tri_op(
    op: String,
    first: ArithExpr,
    second: ArithExpr,
    third: ArithExpr,
) -> Result<CompExpr, DesugarError> {
    match op.as_str() {
        "if" => Ok(CompExpr::If(
            Box::new(desugar(first)?),
            Box::new(desugar(second)?),
            Box::new(desugar(third)?),
        )),
        _ => Err(DesugarError::UnrecognizedTernaryOperation),
    }
}

pub fn desugar(exp: ArithExpr) -> Result<CompExpr, DesugarError> {
    match exp {
        ArithExpr::Num(number) => Ok(CompExpr::Num(number)),
        ArithExpr::UnOp(op, expr) => desugar_un_op(op, *expr),
        ArithExpr::BinOp(op, left, right) => desugar_bin_op(op, *left, *right),
        ArithExpr::Bool(b) => Ok(CompExpr::Bool(b)),
        ArithExpr::TriOp(op, first, second, third) => desugar_tri_op(op, *first, *second, *third),
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

    #[test]
    fn unop_not() {
        let expr = ArithExpr::UnOp("not".to_string(), Box::new(ArithExpr::Bool(false)));
        let res = desugar(expr);
        assert_eq!(res, Ok(CompExpr::Not(Box::new(CompExpr::Bool(false)))));
    }

    #[test]
    fn triop_if() {
        let expr = ArithExpr::TriOp(
            "if".to_string(),
            Box::new(ArithExpr::Bool(true)),
            Box::new(ArithExpr::Num(3)),
            Box::new(ArithExpr::Num(4)),
        );
        let res = desugar(expr);
        assert_eq!(
            res,
            Ok(CompExpr::If(
                Box::new(CompExpr::Bool(true)),
                Box::new(CompExpr::Num(3)),
                Box::new(CompExpr::Num(4))
            ))
        );
    }
}
