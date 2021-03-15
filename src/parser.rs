#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExpr {
    Num(i32),
    List(Vec<SExpr>),
    Symbol(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArithExpr {
    Num(i32),
    Bool(bool),
    BinOp(String, Box<ArithExpr>, Box<ArithExpr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedSymbol(String),
    UnknownExpression(SExpr),
}

fn parse_list(list: Vec<SExpr>) -> Result<ArithExpr, ParseError> {
    match list.as_slice() {
        [SExpr::Symbol(op), left, right] => Ok(ArithExpr::BinOp(
            op.to_string(),
            Box::new(parse((*left).clone())?),
            Box::new(parse((*right).clone())?),
        )),
        _ => Err(ParseError::UnknownExpression(SExpr::List(list))),
    }
}

fn parse_symbol(symbol: String) -> Result<ArithExpr, ParseError> {
    match symbol.as_str() {
        "false" => Ok(ArithExpr::Bool(false)),
        "true" => Ok(ArithExpr::Bool(true)),
        _ => Err(ParseError::UnexpectedSymbol(symbol)),
    }
}

pub fn parse(exp: SExpr) -> Result<ArithExpr, ParseError> {
    match exp {
        SExpr::Num(number) => Ok(ArithExpr::Num(number)),
        SExpr::List(list) => parse_list(list),
        SExpr::Symbol(symbol) => parse_symbol(symbol),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::parser::ArithExpr;
    use crate::parser::ParseError;
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

    #[test]
    fn plus() {
        let expr = SExpr::List(vec![
            SExpr::Symbol("+".to_string()),
            SExpr::Num(1),
            SExpr::Num(2),
        ]);
        let res = parse(expr);
        assert_eq!(
            res,
            Ok(ArithExpr::BinOp(
                "+".to_string(),
                Box::new(ArithExpr::Num(1)),
                Box::new(ArithExpr::Num(2))
            ))
        );
    }

    #[test]
    fn unrecognized_expression() {
        let expr = SExpr::List(vec![
            SExpr::Symbol("+".to_string()),
            SExpr::Num(2),
            SExpr::Num(2),
            SExpr::Num(2),
        ]);
        let res = parse(expr.clone());
        assert_eq!(res, Err(ParseError::UnknownExpression(expr)));
    }

    #[test]
    fn mult() {
        let expr = SExpr::List(vec![
            SExpr::Symbol("*".to_string()),
            SExpr::Num(1),
            SExpr::Num(2),
        ]);
        let res = parse(expr);
        assert_eq!(
            res,
            Ok(ArithExpr::BinOp(
                "*".to_string(),
                Box::new(ArithExpr::Num(1)),
                Box::new(ArithExpr::Num(2))
            ))
        );
    }

    #[test]
    fn bool_false() {
        let expr = SExpr::Symbol("false".to_string());
        let res = parse(expr);
        assert_eq!(res, Ok(ArithExpr::Bool(false)));
    }

    #[test]
    fn bool_true() {
        let expr = SExpr::Symbol("true".to_string());
        let res = parse(expr);
        assert_eq!(res, Ok(ArithExpr::Bool(true)));
    }

    #[test]
    fn not_a_bool() {
        let expr = SExpr::Symbol("True".to_string());
        let res = parse(expr);
        assert_eq!(res, Err(ParseError::UnexpectedSymbol("True".to_string())));
    }
}
