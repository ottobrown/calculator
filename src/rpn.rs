use crate::Token;
use crate::Operator;
use crate::Associativity;

/// Converts an infix expression to a reverse polish notation expression
pub fn to_rpn(exp: Vec<Token>) -> Vec<Token> {
    let mut stack = Vec::new();
    let mut output = Vec::new();
    
    for token in &exp {
        match token {
            Token::Op(op) => {
                while let Some(Token::Op(top)) = stack.last() {
                    if (op.associativity() == Associativity::Left && op.precedence() <= top.precedence()) 
                    || (op.associativity() == Associativity::Right && op.precedence() < top.precedence())  {
                        output.push(stack.pop().unwrap());
                    }
                    else {
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
                    }
                    else {
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
    
    return output;
}

#[cfg(test)]
mod rpn_tests {
    use super::*;
    use crate::Token::*;
    use crate::Operator;
    use crate::parse;

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
}
