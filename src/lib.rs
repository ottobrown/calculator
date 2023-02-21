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
    IncorrectOperands,
    Io(std::io::Error),
}
impl From<std::io::Error> for CalculatorError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[cfg(test)]
mod eval_tests {
    use super::eval;

    #[test]
    fn eval_test() {
        assert_eq!(eval("5").unwrap(), 5.0);
        assert_eq!(eval("5 + 8").unwrap(), 13.0);

        assert!(close_enough(eval("(5 + 3.8)^2 - 3 * 2").unwrap(), 71.44));
    }

    #[test]
    fn eval_test_negatives() {
        assert!(
            // '-' has higher precedence than '/'
            close_enough(eval("1/-5").unwrap(), -0.2)
        );

        assert!(
            // '-' has lower precedence than '^' ...
            close_enough(eval("-2^2").unwrap(), -4.0)
        );

        assert!(
            // ... but this strill works
            close_enough(eval("2^-3").unwrap(), 0.125)
        );
    }

    fn close_enough(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.0000001
    }
}
