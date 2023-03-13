mod env;
mod parse;
mod rpn;
pub mod token;

pub use env::Env;
pub use parse::parse;
pub use rpn::to_rpn;
pub use token::*;

pub fn eval(s: impl Into<String>) -> Result<f64, CalculatorError> {
    let env = Env::default();
    rpn::eval_rpn(to_rpn(parse(s.into(), &env)?))
}

#[derive(Debug)]
pub enum CalculatorError {
    IllegalCharacter(char),
    ParseError,
    IncorrectOperands,
    UnknownSymbol(String),
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
            // ... but this still works
            close_enough(eval("2^-3").unwrap(), 0.125)
        );
    }

    #[test]
    fn eval_test_implied_multiplication() {
        assert!(close_enough(eval("3(-4 + 1)").unwrap(), -9.0));

        assert!(
            // implied multiplication has higher precedence than division
            close_enough(eval("1/(2)(5)").unwrap(), 0.1)
        );
        assert!(close_enough(eval("1/-(2)(5)").unwrap(), -0.1));

        assert!(close_enough(eval("(4 + 1)3").unwrap(), 15.0));
    }

    #[test]
    fn constants() {
        assert!(close_enough(eval("pi").unwrap(), 3.14159265));
        assert!(close_enough(eval("e^2").unwrap(), 7.389056098));
    }

    #[test]
    fn functions() {
        assert!(close_enough(eval("cos(pi/2)").unwrap(), 0.0));
        assert!(close_enough(eval("cos pi").unwrap(), -1.0));

        assert!(close_enough(eval("e^ln2").unwrap(), 2.0));

        assert_eq!(eval("sin(1)").unwrap(), 1.0_f64.sin());
        assert_eq!(eval("tan(1)").unwrap(), 1.0_f64.tan());

        assert_eq!(eval("ln2").unwrap(), 2.0_f64.ln());
        assert_eq!(eval("3ln2").unwrap(), 3.0 * 2.0_f64.ln());

        assert_eq!(eval("ln2/ln3").unwrap(), 2.0_f64.ln() / 3.0_f64.ln());
    }

    fn close_enough(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.0000001
    }
}
