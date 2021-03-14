#[derive(Debug, PartialEq, Eq)]
pub enum SExpr {
    Num(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithExpr {
    Num(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {}

pub fn parse(exp: SExpr) -> Result<ArithExpr, ParseError> {
    match exp {
        SExpr::Num(number) => Ok(ArithExpr::Num(number)),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::parser::ArithExpr;
    use crate::SExpr;

    #[test]
    fn number() {
        let expr = SExpr::Num(1);
        let res = parse(expr);
        assert_eq!(res, Ok(ArithExpr::Num(1)));
    }

    #[test]
    fn negative_number() {
        let expr = SExpr::Num(-4);
        let res = parse(expr);
        assert_eq!(res, Ok(ArithExpr::Num(-4)));
    }
}
