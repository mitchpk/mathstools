use crate::expr;
use crate::num::{Num, NumError};

use std::collections::HashMap;

pub trait Eval {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError>;
}

/*
pub fn eval(ast: Expr, env: &mut HashMap<String, Num>) -> Result<Num, EvalError> {
    use crate::ast::Expr::*;
    match ast {
        Num(i) => Ok(i),
        Ident(s) => match env.get(&s) {
            Some(f) => Ok(f.clone()),
            None => Err(EvalError::UnknownVar(format!("Unknown variable: {}", s))),
        },
        Assign(s, e) => {
            let val = eval(*e, env)?;
            env.insert(s, val.clone());
            Ok(val)
        }
        Add(e1, e2) => Ok(eval(*e1, env)? + eval(*e2, env)?),
        _ => Ok((rug::Integer::from(0) + rug::Rational::from((13, 12))).into()),
    }
}*/

use thiserror::Error;
#[derive(Debug, Error)]
pub enum EvalError {
    #[error("unknown variable {0}")]
    UnknownVar(String),
    #[error("number evaluation failed")]
    Num(#[from] NumError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = expr::Number {
            value: Num::from(rug::Integer::from(2)),
        };
        let mut env: HashMap<String, Num> = HashMap::new();
        env.insert(
            "e".to_string(),
            Num::from(rug::Float::with_val(53, 1).exp()),
        );

        let y = expr::Number {
            value: Num::from(rug::Complex::with_val(53, (12, 25))),
        };

        let ast = expr::Div { left: x, right: y };
        let ast = expr::Neg { value: ast };
        if let Err(EvalError::Num(error)) = ast.eval(&mut env) {
            println!("{}", error);
        }
    }
}
