use crate::CalculatorError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Op(Operator),
    LParen,
    RParen,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Associativity {
    Left,
    Right,

    Unary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    ImpliedMultiply,
    Divide,
    Exponent,

    Negative,
}
impl Operator {
    pub fn operate(&self, left: f64, right: Option<f64>) -> Result<f64, CalculatorError> {
        let x = match right {
            Some(r) => match self {
                Self::Add => left + r,
                Self::Subtract => left - r,
                Self::Multiply => left * r,
                Self::ImpliedMultiply => left * r,
                Self::Divide => left / r,
                Self::Exponent => left.powf(r),

                _ => return Err(CalculatorError::IncorrectOperands),
            },

            None => match self {
                Self::Negative => -left,

                _ => return Err(CalculatorError::IncorrectOperands),
            },
        };

        Ok(x)
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Subtract => 1,
            Self::Multiply => 2,
            Self::Divide => 2,
            Self::ImpliedMultiply => 3,
            Self::Negative => 4,
            Self::Exponent => 5,
        }
    }

    pub fn associativity(&self) -> Associativity {
        match self {
            Self::Add | Self::Subtract | Self::Multiply | Self::ImpliedMultiply | Self::Divide => {
                Associativity::Left
            }
            Self::Exponent => Associativity::Right,
            Self::Negative => Associativity::Unary,
        }
    }
}
