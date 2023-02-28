use crate::token::Operator;
use crate::token::Token;
use crate::CalculatorError;

/// Parse a `String` into a `Vec` of tokens
#[allow(unused_assignments)]
pub fn parse(s: String) -> Result<Vec<Token>, CalculatorError> {
    let mut expression = Vec::new();
    let mut number = ParseNumber::default();

    for c in s.chars() {
        if c.is_ascii_whitespace() {
            end_number(&mut number, &mut expression);
            continue;
        }

        if c.is_ascii_digit() {
            number.digits.push((c as u8) - 48);
            continue;
        } else if c == '.' {
            number.decimal_point = Some(number.digits.len() - 1);
            continue;
        }
            
        end_number(&mut number, &mut expression);

        let t = match c {
            '*' => Token::Op(Operator::Multiply),
            '+' => Token::Op(Operator::Add),
            '-' => match expression.last() {
                Some(Token::RParen) | Some(Token::Number(_)) => Token::Op(Operator::Subtract),

                _ => Token::Op(Operator::Negative),
            },
            '/' => Token::Op(Operator::Divide),
            '^' => Token::Op(Operator::Exponent),
            '(' => {
                if let Some(Token::RParen) = expression.last() {
                    // if a left paren is next to a right paren, multiplication is implied
                    expression.push(Token::Op(Operator::ImpliedMultiply));
                }

                if let Some(Token::Number(_)) = expression.last() {
                    // if a left paren is right after a number, multiplication is implied
                    expression.push(Token::Op(Operator::ImpliedMultiply));
                }

                Token::LParen
            }
            ')' => Token::RParen,

            _ => return Err(CalculatorError::IllegalCharacter(c)),
        };

        expression.push(t)
    }

    end_number(&mut number, &mut expression);

    Ok(expression)
}

fn end_number(number: &mut ParseNumber, expression: &mut Vec<Token>) {
    if number.digits.is_empty() {
        return;
    }

    if let Some(Token::Number(_)) = expression.last() {
        // if two numbers are right next to each other, multiplication is implied
        expression.push(Token::Op(Operator::ImpliedMultiply));
    }
    if let Some(Token::RParen) = expression.last() {
        // if a number is next to a right paren, multiplication is implied
        expression.push(Token::Op(Operator::ImpliedMultiply));
    }

    expression.push(Token::Number(number.parse()));

    *number = ParseNumber::default();
}

/// A base-10 number in the process of being parsed
#[derive(Default, Debug)]
struct ParseNumber {
    digits: Vec<u8>,
    decimal_point: Option<usize>,
}
impl ParseNumber {
    pub fn parse(&self) -> f64 {
        let mut number = 0.0;

        for i in 0..self.digits.len() {
            let d = self.digits[i] as f64;

            match self.decimal_point {
                Some(dp) => number += d * 10_f64.powi(dp as i32 - i as i32),
                None => number += d * 10_f64.powi((self.digits.len() - 1 - i) as i32),
            }
        }

        number
    }
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn parse_integer() {
        let exp = parse("723".to_string()).unwrap();

        assert_eq!(exp, vec![Token::Number(723.0)]);
    }

    #[test]
    fn parse_noninteger() {
        let exp = parse("723.81".to_string()).unwrap();

        assert_eq!(exp, vec![Token::Number(723.81)]);
    }

    #[test]
    fn parse_expression() {
        let exp = parse("5 * (14 - 1)^2 - 355".to_string()).unwrap();

        assert_eq!(
            exp,
            vec![
                Token::Number(5.0),
                Token::Op(Operator::Multiply),
                Token::LParen,
                Token::Number(14.0),
                Token::Op(Operator::Subtract),
                Token::Number(1.0),
                Token::RParen,
                Token::Op(Operator::Exponent),
                Token::Number(2.0),
                Token::Op(Operator::Subtract),
                Token::Number(355.0),
            ]
        );
    }

    #[test]
    fn implied_multiplication() {
        let exp = parse("(15)(5)".to_string()).unwrap();

        assert_eq!(
            exp,
            vec![
                Token::LParen,
                Token::Number(15.0),
                Token::RParen,
                Token::Op(Operator::ImpliedMultiply),
                Token::LParen,
                Token::Number(5.0),
                Token::RParen,
            ]
        );

        let exp2 = parse("3(2 + 1)".to_string()).unwrap();

        assert_eq!(
            exp2,
            vec![
                Token::Number(3.0),
                Token::Op(Operator::ImpliedMultiply),
                Token::LParen,
                Token::Number(2.0),
                Token::Op(Operator::Add),
                Token::Number(1.0),
                Token::RParen,
            ]
        );

        let exp3 = parse("(2+1)3".to_string()).unwrap();

        assert_eq!(
            exp3,
            vec![
                Token::LParen,
                Token::Number(2.0),
                Token::Op(Operator::Add),
                Token::Number(1.0),
                Token::RParen,
                Token::Op(Operator::ImpliedMultiply),
                Token::Number(3.0),
            ]
        );
    }
}
