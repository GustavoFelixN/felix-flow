use crate::expr::Expr;
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FuncCall {
    pub(crate) callee: String,
    pub(crate) params: Vec<Expr>,
}

impl FuncCall {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, callee) = utils::extract_ident(s)?;
        let (s, _) = utils::extract_whitespace1(s)?;

        let (s, params) = utils::sequence(Expr::new, s)?;

        Ok((
            s,
            Self {
                callee: callee.to_string(),
                params,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::Number;
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
}
