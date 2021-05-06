use crate::num::{Num, NumError};
use std::collections::HashMap;

pub trait Eval {
    fn eval(self, env: &mut HashMap<String, Num>) -> Result<Num, EvalError>;
}

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
    use crate::expr;

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

    #[test]
    fn test_ln() {
        let num = Num::from(rug::Float::with_val(53, 1).exp());
        let mut env: HashMap<String, Num> = HashMap::new();

        let ast = expr::Ln {
            value: expr::Number { value: num },
        };

        println!("{}", ast.eval(&mut env).unwrap());
    }
}
