use crate::Associativity;
use crate::Token;

/// Converts an infix expression to a reverse polish notation expression
pub fn to_rpn(exp: Vec<Token>) -> Vec<Token> {
    let mut stack = Vec::new();
    let mut output = Vec::new();

    for token in &exp {
        match token {
            Token::Op(op) => {
                while let Some(Token::Op(top)) = stack.last() {
                    if (op.associativity() == Associativity::Left
                        && op.precedence() <= top.precedence())
                        || (op.associativity() == Associativity::Right
                            && op.precedence() < top.precedence())
                    {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                stack.push(*token);
            }

            Token::LParen => stack.push(*token),
            Token::RParen => {
                while let Some(p) = stack.pop() {
                    if p != Token::LParen {
                        output.push(p);
                    } else {
                        break;
                    }
                }
            }

            _ => output.push(*token),
        }
    }

    while let Some(t) = stack.pop() {
        output.push(t);
    }

    output
}

/// Evaluate a reverse-polish notation expression
// TODO: handle errors
pub fn eval_rpn(rpn: Vec<Token>) -> Result<f64, crate::CalculatorError> {
    let mut stack: Vec<f64> = Vec::new();

    for token in rpn {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Op(o) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                stack.push(o.operate(b, a));
            },

            _ => {}
        }
    }

    Ok(stack.pop().unwrap())
}

#[cfg(test)]
mod rpn_tests {
    use super::*;
    use crate::parse;
    use crate::Token::*;
    use crate::Operator;

    #[test]
    fn parse_to_rpn() {
        let s = "6+9+(4*2+4^2)".to_string();

        assert_eq!(
            to_rpn(parse(s).unwrap()),
            vec![
                Number(6.0),
                Number(9.0),
                Op(Operator::Add),
                Number(4.0),
                Number(2.0),
                Op(Operator::Multiply),
                Number(4.0),
                Number(2.0),
                Op(Operator::Exponent),
                Op(Operator::Add),
                Op(Operator::Add),
            ]
        );
    }

    #[test]
    fn eval() {
        assert_eq!(
            eval_rpn(vec![
                Number(6.0),
                Number(9.0),
                Op(Operator::Add),
                Number(4.0),
                Number(2.0),
                Op(Operator::Multiply),
                Number(4.0),
                Number(2.0),
                Op(Operator::Exponent),
                Op(Operator::Add),
                Op(Operator::Add),
            ]).unwrap(),
            39.0
        );
    }
}
