use crate::{env::Env, stmt::Stmt, utils, val::Val};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let mut s = s;
        let mut stmts = Vec::new();

        while let Ok((new_s, stmt)) = Stmt::new(s) {
            s = new_s;
            stmts.push(stmt);

            let (new_s, _) = utils::extract_whitespace(s);
            s = new_s;
        }

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;
        Ok((s, Block { stmts }))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        self.stmts.last().map_or(Ok(Val::Unit), |stmt| match stmt {
            Stmt::BindingDef(_) => todo!(),
            Stmt::Expr(expr) => expr.eval(env),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Expr, Number};
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::binding_usage::BindingUsage;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })))
    }

    #[test]
    fn parse_empty_block_with_whitespace() {
        assert_eq!(Block::new("{   }"), Ok(("", Block { stmts: Vec::new() })))
    }

    #[test]
    fn parse_block_with_one_statements() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    stmts: vec![Stmt::Expr(Expr::Number(Number(5)))]
                }
            ))
        )
    }

    #[test]
    fn parse_block_with_multiple_statements() {
        assert_eq!(
            Block::new(
                "{
    let a = 10
    let b = a
    b
}"
            ),
            Ok((
                "",
                Block {
                    stmts: vec![
                        Stmt::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(10)),
                        }),
                        Stmt::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string()
                            })
                        }),
                        Stmt::Expr(Expr::BindingUsage(BindingUsage {
                            name: "b".to_string()
                        })),
                    ]
                }
            ))
        )
    }

    #[test]
    fn eval_empty_block() {
        assert_eq!(
            Block { stmts: Vec::new() }.eval(&Env::default()),
            Ok(Val::Unit),
        );
    }

    #[test]
    fn eval_block_with_one_expr() {
        assert_eq!(
            Block {
                stmts: vec![Stmt::Expr(Expr::Number(Number(25)))]
            }
            .eval(&Env::default()),
            Ok(Val::Number(25))
        )
    }
}
