use rug::{Complex, Float, Integer, Rational};
use std::ops::*;

#[derive(Clone, Debug)]
pub enum Num {
    Complex(Complex),
    Float(Float),
    Integer(Integer),
    Rational(Rational),
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Num::*;
        match self {
            Complex(c) => write!(f, "Complex({})", c),
            Float(d) => write!(f, "Float({})", d),
            Integer(i) => write!(f, "Integer({})", i),
            Rational(r) => write!(f, "Rational({})", r),
        }
    }
}

impl From<Complex> for Num {
    fn from(item: Complex) -> Num {
        Num::Complex(item)
    }
}

impl From<Float> for Num {
    fn from(item: Float) -> Num {
        Num::Float(item)
    }
}

impl From<Integer> for Num {
    fn from(item: Integer) -> Num {
        Num::Integer(item)
    }
}

impl From<Rational> for Num {
    fn from(item: Rational) -> Num {
        Num::Rational(item)
    }
}

impl Add for Num {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        use Num::*;
        match self {
            Complex(c1) => match rhs {
                Complex(c2) => Num::from(c1 + c2),
                Float(d2) => Num::from(c1 + d2),
                Integer(i2) => Num::from(c1 + i2),
                Rational(r2) => Num::from(c1 + r2),
            },
            Float(d1) => match rhs {
                Complex(c2) => Num::from(d1 + c2),
                Float(d2) => Num::from(d1 + d2),
                Integer(i2) => Num::from(d1 + i2),
                Rational(r2) => Num::from(d1 + r2),
            },
            Integer(i1) => match rhs {
                Complex(c2) => Num::from(i1 + c2),
                Float(d2) => Num::from(i1 + d2),
                Integer(i2) => Num::from(i1 + i2),
                Rational(r2) => Num::from(i1 + r2),
            },
            Rational(r1) => match rhs {
                Complex(c2) => Num::from(r1 + c2),
                Float(d2) => Num::from(r1 + d2),
                Integer(i2) => Num::from(r1 + i2),
                Rational(r2) => Num::from(r1 + r2),
            },
        }
    }
}

impl Sub for Num {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        use Num::*;
        match self {
            Complex(c1) => match rhs {
                Complex(c2) => Num::from(c1 - c2),
                Float(d2) => Num::from(c1 - d2),
                Integer(i2) => Num::from(c1 - i2),
                Rational(r2) => Num::from(c1 - r2),
            },
            Float(d1) => match rhs {
                Complex(c2) => Num::from(d1 - c2),
                Float(d2) => Num::from(d1 - d2),
                Integer(i2) => Num::from(d1 - i2),
                Rational(r2) => Num::from(d1 - r2),
            },
            Integer(i1) => match rhs {
                Complex(c2) => Num::from(i1 - c2),
                Float(d2) => Num::from(i1 - d2),
                Integer(i2) => Num::from(i1 - i2),
                Rational(r2) => Num::from(i1 - r2),
            },
            Rational(r1) => match rhs {
                Complex(c2) => Num::from(r1 - c2),
                Float(d2) => Num::from(r1 - d2),
                Integer(i2) => Num::from(r1 - i2),
                Rational(r2) => Num::from(r1 - r2),
            },
        }
    }
}

impl Mul for Num {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        use Num::*;
        match self {
            Complex(c1) => match rhs {
                Complex(c2) => Num::from(c1 * c2),
                Float(d2) => Num::from(c1 * d2),
                Integer(i2) => Num::from(c1 * i2),
                Rational(r2) => Num::from(c1 * r2),
            },
            Float(d1) => match rhs {
                Complex(c2) => Num::from(d1 * c2),
                Float(d2) => Num::from(d1 * d2),
                Integer(i2) => Num::from(d1 * i2),
                Rational(r2) => Num::from(d1 * r2),
            },
            Integer(i1) => match rhs {
                Complex(c2) => Num::from(i1 * c2),
                Float(d2) => Num::from(i1 * d2),
                Integer(i2) => Num::from(i1 * i2),
                Rational(r2) => Num::from(i1 * r2),
            },
            Rational(r1) => match rhs {
                Complex(c2) => Num::from(r1 * c2),
                Float(d2) => Num::from(r1 * d2),
                Integer(i2) => Num::from(r1 * i2),
                Rational(r2) => Num::from(r1 * r2),
            },
        }
    }
}

impl Div for Num {
    type Output = Result<Self, NumError>;
    fn div(self, rhs: Self) -> Result<Self, NumError> {
        use Num::*;
        match self {
            Complex(c1) => match rhs {
                Complex(c2) => Ok(Num::from(c1 / c2)),
                Float(d2) => Ok(Num::from(c1 / d2)),
                Integer(i2) => Ok(Num::from(c1 / i2)),
                Rational(r2) => Ok(Num::from(c1 / r2)),
            },
            Float(d1) => match rhs {
                Complex(c2) => Ok(Num::from(d1 / c2)),
                Float(d2) => Ok(Num::from(d1 / d2)),
                Integer(i2) => Ok(Num::from(d1 / i2)),
                Rational(r2) => Ok(Num::from(d1 / r2)),
            },
            Integer(i1) => match rhs {
                Complex(_) => Err(NumError::InvalidOp(format!(
                    "Cannot divide Integer by Complex"
                ))),
                Float(d2) => Ok(Num::from(i1 / d2)),
                Integer(i2) => Ok(Num::from(i1 / i2)),
                Rational(r2) => Ok(Num::from(i1 / r2)),
            },
            Rational(r1) => match rhs {
                Complex(_) => Err(NumError::InvalidOp(format!(
                    "Cannot divide Rational by Complex"
                ))),
                Float(d2) => Ok(Num::from(r1 / d2)),
                Integer(i2) => Ok(Num::from(r1 / i2)),
                Rational(r2) => Ok(Num::from(r1 / r2)),
            },
        }
    }
}

impl Neg for Num {
    type Output = Self;
    fn neg(self) -> Self {
        use Num::*;
        match self {
            Complex(c) => Num::from(-c),
            Float(f) => Num::from(-f),
            Integer(i) => Num::from(-i),
            Rational(r) => Num::from(-r),
        }
    }
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum NumError {
    #[error("invalid operation")]
    InvalidOp(String),
}
