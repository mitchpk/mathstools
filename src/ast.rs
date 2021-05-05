use crate::num::Num;

#[derive(Debug)]
pub enum Func {
    Abs,
    Floor,
    Ln,
    Log,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
}

pub enum Token {
    Num(Num),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    RParen,
    LParen,
    Equals,
    Func(Func),
    Ident(String),
    Eof,
}

impl Token {
    /// Returns the precedence of this token
    pub fn get_precedence(&self) -> Precedence {
        use Precedence::*;
        use Token::*;

        match *self {
            Add | Sub => Sum,
            Mul | Div => Product,
            Pow => Power,
            Func(_) => Function,
            Equals => Assign,
            _ => Lowest,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Token::*;

        match self {
            Num(i) => write!(f, "Num({})", i),
            Add => write!(f, "Add"),
            Sub => write!(f, "Sub"),
            Mul => write!(f, "Mul"),
            Div => write!(f, "Div"),
            Pow => write!(f, "Pow"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            Equals => write!(f, "="),
            // Implement Display for func
            Func(func) => write!(f, "{:?}", func),
            Ident(s) => write!(f, "{}", s),
            Eof => write!(f, "Eof"),
        }
    }
}

pub enum Precedence {
    Lowest,
    Sum,
    Product,
    Power,
    Function,
    Prefix,
    Assign,
}

pub fn get_function_token(s: impl AsRef<str>) -> Option<Token> {
    match s.as_ref() {
        "abs" => Some(Token::Func(Func::Abs)),
        "floor" => Some(Token::Func(Func::Floor)),
        "log" => Some(Token::Func(Func::Log)),
        "ln" => Some(Token::Func(Func::Ln)),
        "sin" => Some(Token::Func(Func::Sin)),
        "cos" => Some(Token::Func(Func::Cos)),
        "tan" => Some(Token::Func(Func::Tan)),
        "arcsin" => Some(Token::Func(Func::Arcsin)),
        "arccos" => Some(Token::Func(Func::Arccos)),
        "arctan" => Some(Token::Func(Func::Arctan)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        let rational = Num::Rational(rug::Rational::from((15, 4)));
        println!("{}", rational);
    }
}
