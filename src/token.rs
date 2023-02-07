#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Op(Operator),
    LParen,
    RParen,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}
impl Operator {
    pub fn operate(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Subtract => left - right,
            Self::Multiply => left * right,
            Self::Divide => left / right,
            Self::Exponent => left.powf(right),
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Subtract => 1,
            Self::Multiply => 2,
            Self::Divide => 2,
            Self::Exponent => 3,
        }
    }

    pub fn associativity(&self) -> Associativity {
        match self {
            Self::Add | Self::Subtract | Self::Multiply | Self::Divide => Associativity::Left,
            Self::Exponent => Associativity::Right,
        }
    }
}
