use crate::expr::Expr;
use crate::{utils, Env, Val};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_ident(s)?;
        let (s, _) = utils::take_while(|c| c == ' ', s);

        let (s, params) = utils::sequence1(Expr::new, |s| utils::take_while(|c| c == ' ', s), s)?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        let mut child_env = env.create_child();

        let (param_names, body) = env.get_func(&self.callee)?;

        let num_expected_params = param_names.len();
        let num_actual_params = self.params.len();

        if num_expected_params != num_actual_params {
            return Err(format!(
                "expected {} parameters, got {}",
                num_expected_params, num_actual_params
            ));
        }

        for (param_name, param_expr) in param_names.into_iter().zip(&self.params) {
            let param_val = param_expr.eval(&child_env)?;
            child_env.store_binding(param_name, param_val);
        }

        body.eval(&mut child_env)
    }
}

#[cfg(test)]
mod tests {
    use crate::stmt::Stmt;

    use super::super::{BindingUsage, Number, Op};
    use super::*;

    #[test]
    fn parse_func_call_one_params() {
        assert_eq!(
            FuncCall::new("factorial 10"),
            Ok((
                "",
                FuncCall {
                    callee: "factorial".to_string(),
                    params: vec![Expr::Number(Number(10))],
                },
            ))
        )
    }

    #[test]
    fn eval_func_call() {
        let mut env = Env::default();

        env.store_func(
            "id".to_string(),
            vec!["x".to_string()],
            Stmt::Expr(Expr::BindingUsage(BindingUsage {
                name: "x".to_string(),
            })),
        );

        assert_eq!(
            FuncCall {
                callee: "id".to_string(),
                params: vec![Expr::Number(Number(10))],
            }
            .eval(&env),
            Ok(Val::Number(10)),
        )
    }

    #[test]
    fn eval_non_existing_func_call() {
        let env = Env::default();
        assert_eq!(
            FuncCall {
                callee: "i_dont_exist".to_string(),
                params: vec![Expr::Number(Number(1))],
            }
            .eval(&env),
            Err("function with name 'i_dont_exist' does not exist".to_string())
        )
    }

    #[test]
    fn eval_func_call_with_too_few_params() {
        let mut env = Env::default();

        env.store_func(
            "mul".to_string(),
            vec!["a".to_string(), "b".to_string()],
            Stmt::Expr(Expr::Operation {
                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "a".to_string(),
                })),
                rhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "b".to_string(),
                })),
                op: Op::Mul,
            }),
        );

        assert_eq!(
            FuncCall {
                callee: "mul".to_string(),
                params: vec![Expr::Number(Number(100))]
            }
            .eval(&env),
            Err("expected 2 parameters, got 1".to_string()),
        )
    }

    #[test]
    fn eval_func_with_too_many_params() {
        let mut env = Env::default();

        env.store_func(
            "mul".to_string(),
            vec!["a".to_string(), "b".to_string()],
            Stmt::Expr(Expr::Operation {
                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "a".to_string(),
                })),
                rhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "b".to_string(),
                })),
                op: Op::Mul,
            }),
        );

        assert_eq!(
            FuncCall {
                callee: "mul".to_string(),
                params: vec![
                    Expr::Number(Number(100)),
                    Expr::Number(Number(100)),
                    Expr::Number(Number(100))
                ]
            }
            .eval(&env),
            Err("expected 2 parameters, got 3".to_string()),
        )
    }
}
