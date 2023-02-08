mod parse;
mod rpn;
pub mod token;

pub use parse::parse;
pub use rpn::{to_rpn, eval_rpn};
pub use token::*;

pub fn eval(s: impl Into<String>) -> Result<f64, CalculatorError> {
    eval_rpn(to_rpn(parse(s.into())?))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalculatorError {
    IllegalCharacter(char),
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
