use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Number(i32),
    //WARN: this is basically a null righ now. remove later
    Unit,
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Unit => write!(f, "Unit"),
        }
    }
}
