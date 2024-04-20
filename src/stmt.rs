use crate::binding_def::BindingDef;
use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Smt {
    BindingDef(BindingDef),
    Expr(Expr),
}

impl Smt {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, binding_def)| (s, Self::BindingDef(binding_def)))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Self::Expr(expr))))
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{Number, Op};

    use super::*;

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Smt::new("let a = 10"),
            Ok((
                "",
                Smt::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(10))
                })
            ))
        )
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Smt::new("5+5"),
            Ok((
                "",
                Smt::Expr(Expr::Operation {
                    lhs: Number(5),
                    rhs: Number(5),
                    op: Op::Add
                })
            ))
        )
    }
}