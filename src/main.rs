use crate::desugarer::desugar;
use crate::interpreter::interp;
use crate::parser::SExpr;
use parser::parse;

mod desugarer;
mod interpreter;
mod parser;

fn main() {
    let program = SExpr::List(vec![
        SExpr::Symbol("+".to_string()),
        SExpr::Num(1),
        SExpr::Num(1),
    ]);
    let parsed = parse(program);
    let desugared = desugar(parsed.unwrap());
    let interpreted = interp(desugared.unwrap());
    println!("output: {:?}", interpreted.unwrap());
}
