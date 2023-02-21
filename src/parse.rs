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
            continue;
        }

        if c.is_ascii_digit() {
            number.digits.push((c as u8) - 48);
            continue;
        } else if c == '.' {
            number.decimal_point = Some(number.digits.len() - 1);
            continue;
        } else if !number.digits.is_empty() {
            expression.push(Token::Number(number.parse()));

            // This is marked as an unused assignment
            number = ParseNumber::default();
        }

        expression.push(match c {
            '*' => Token::Op(Operator::Multiply),
            '+' => Token::Op(Operator::Add),
            '-' => match expression.last() {
                Some(Token::RParen) | Some(Token::Number(_)) => Token::Op(Operator::Subtract),

                _ => Token::Op(Operator::Negative),
            },
            '/' => Token::Op(Operator::Divide),
            '^' => Token::Op(Operator::Exponent),
            '(' => Token::LParen,
            ')' => Token::RParen,

            _ => return Err(CalculatorError::IllegalCharacter(c)),
        });
    }

    if !number.digits.is_empty() {
        expression.push(Token::Number(number.parse()));
        number = ParseNumber::default();
    }

    Ok(expression)
}

/// A base-10 number in the process of being parsed
#[derive(Default, Debug)]
struct ParseNumber {
    digits: Vec<u8>,
    decimal_point: Option<usize>,
}
impl ParseNumber {
    pub fn parse(self) -> f64 {
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
}
