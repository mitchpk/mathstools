/*pub enum Expr {
    Num(Num),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Abs(Box<Expr>),
    Floor(Box<Expr>),
    Ln(Box<Expr>),
    Log(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Arcsin(Box<Expr>),
    Arccos(Box<Expr>),
    Arctan(Box<Expr>),
    Ident(String),
    Assign(String, Box<Expr>),
}*/

use crate::eval::{Eval, EvalError};
use crate::num::Num;
use std::collections::HashMap;

pub struct Add<T: Eval, U: Eval> {
    pub left: T,
    pub right: U,
}

impl<T: Eval, U: Eval> Eval for Add<T, U> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(self.left.eval(env)? + self.right.eval(env)?)
    }
}

pub struct Sub<T: Eval, U: Eval> {
    pub left: T,
    pub right: U,
}

impl<T: Eval, U: Eval> Eval for Sub<T, U> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(self.left.eval(env)? - self.right.eval(env)?)
    }
}

pub struct Mul<T: Eval, U: Eval> {
    pub left: T,
    pub right: U,
}

impl<T: Eval, U: Eval> Eval for Mul<T, U> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(self.left.eval(env)? * self.right.eval(env)?)
    }
}

pub struct Div<T: Eval, U: Eval> {
    pub left: T,
    pub right: U,
}

impl<T: Eval, U: Eval> Eval for Div<T, U> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok((self.left.eval(env)? / self.right.eval(env)?)?)
    }
}

pub struct Neg<T: Eval> {
    pub value: T,
}

impl<T: Eval> Eval for Neg<T> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(-self.value.eval(env)?)
    }
}

pub struct Abs<T: Eval> {
    pub value: T,
}

impl<T: Eval> Eval for Abs<T> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(self.value.eval(env)?.abs())
    }
}

pub struct Floor<T: Eval> {
    pub value: T,
}

impl<T: Eval> Eval for Floor<T> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(self.value.eval(env)?.floor()?)
    }
}

pub struct Ln<T: Eval> {
    pub value: T,
}

impl<T: Eval> Eval for Ln<T> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(self.value.eval(env)?.ln())
    }
}

pub struct Log<T: Eval, U: Eval> {
    pub value: T,
    pub base: U,
}

impl<T: Eval, U: Eval> Eval for Log<T, U> {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
        Ok(self.value.eval(env)?.log(self.base.eval(env)?)?)
    }
}
