mod parse;
mod rpn;
pub mod token;

pub use parse::parse;
pub use rpn::to_rpn;
pub use token::*;

pub fn eval(s: impl Into<String>) -> Result<f64, CalculatorError> {
    rpn::eval_rpn(to_rpn(parse(s.into())?))
}

#[derive(Debug)]
pub enum CalculatorError {
    IllegalCharacter(char),
    ParseError,
    Io(std::io::Error),
}
impl From<std::io::Error> for CalculatorError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[test]
fn eval_test() {
    assert_eq!(eval("5").unwrap(), 5.0);
    assert_eq!(eval("5 + 8").unwrap(), 13.0);

    assert!(
        // these not exactly the same because of floating-point precision
        (eval("(5 + 3.8)^2 - 3 * 2").unwrap() - 71.44).abs() < 0.000001
    );
}
