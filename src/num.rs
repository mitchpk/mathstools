use crate::eval::{Eval, EvalError};
use rug::{Complex, Float, Integer, Rational};
use std::collections::HashMap;
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

impl Eval for Num {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(self)
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

impl Num {
    pub fn abs(self) -> Self {
        use Num::*;
        match self {
            Complex(c) => Num::from(c.abs()),
            Float(f) => Num::from(f.abs()),
            Integer(i) => Num::from(i.abs()),
            Rational(r) => Num::from(r.abs()),
        }
    }

    pub fn floor(self) -> Result<Self, NumError> {
        use Num::*;
        match self {
            Complex(_) => Err(NumError::InvalidOp("Cannot floor a Complex".to_string())),
            Float(f) => Ok(Num::from(f.floor())),
            Integer(_) => Err(NumError::InvalidOp("Cannot floor an Integer".to_string())),
            Rational(r) => Ok(Num::from(r.floor())),
        }
    }

    pub fn ln(self) -> Self {
        use Num::*;
        match self {
            Complex(c) => Num::from(c.ln()),
            Float(f) => Num::from(f.ln()),
            Integer(i) => Num::from(rug::Float::with_val(53, i).ln()),
            Rational(r) => Num::from(rug::Float::with_val(53, r).ln()),
        }
    }

    pub fn to_float(self) -> Result<Self, NumError> {
        use Num::*;
        match self {
            Complex(c) => Num::from(c.ln()),
            Float(f) => Num::from(f.ln()),
            Integer(i) => Num::from(rug::Float::with_val(53, i).ln()),
            Rational(r) => Num::from(rug::Float::with_val(53, r).ln()),
        }
    }

    pub fn log(self, base: Self) -> Result<Self, NumError> {
        use Num::*;
        match self {
            Complex(c1) => match base {
                Complex(c2) => Ok(Num::from(c1.log10() / c2.log10())),
                Float(d2) => Ok(Num::from(c1.log10() / d2.log10())),
                Integer(i2) => Ok(Num::from(c1.log10() / i2.log10())),
                Rational(r2) => Ok(Num::from(c1.log10() / r2.log10())),
            },
            Float(d1) => match base {
                Complex(c2) => Ok(Num::from(d1.log10() / c2)),
                Float(d2) => Ok(Num::from(d1.log10() / d2)),
                Integer(i2) => Ok(Num::from(d1.log10() / i2)),
                Rational(r2) => Ok(Num::from(d1.log10() / r2)),
            },
            Integer(i1) => match base {
                Complex(_) => Err(NumError::InvalidOp(format!(
                    "Cannot divide Integer by Complex"
                ))),
                Float(d2) => Ok(Num::from(i1.log10() / d2)),
                Integer(i2) => Ok(Num::from(i1.log10() / i2)),
                Rational(r2) => Ok(Num::from(i1.log10() / r2)),
            },
            Rational(r1) => match base {
                Complex(_) => Err(NumError::InvalidOp(format!(
                    "Cannot divide Rational by Complex"
                ))),
                Float(d2) => Ok(Num::from(r1.log10() / d2)),
                Integer(i2) => Ok(Num::from(r1.log10() / i2)),
                Rational(r2) => Ok(Num::from(r1.log10() / r2)),
            },
        }
    }
}

use thiserror::Error;
#[derive(Debug, Error)]
pub enum NumError {
    #[error("invalid operation")]
    InvalidOp(String),
}
