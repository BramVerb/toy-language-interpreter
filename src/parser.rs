#[derive(Debug, PartialEq, Eq)]
pub enum SExpr {
    List(Vec<SExpr>),
    Num(i32),
    Sym(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithOp {
    Plus,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithExpr {
    BinOp(ArithOp, Box<ArithExpr>, Box<ArithExpr>),
    Num(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Unimplemented,
}

pub fn parse(exp: SExpr) -> Result<ArithExpr, ParseError> {
    match exp {
        SExpr::Num(number) => Ok(ArithExpr::Num(number)),
        _ => Err(ParseError::Unimplemented),
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
}
