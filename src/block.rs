use crate::{stmt::Stmt, utils};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, stmts) = if let Ok((s, stmt)) = Stmt::new(s) {
            (s, vec![stmt])
        } else {
            (s, Vec::new())
        };

        let (s, _) = utils::extract_whitespace(s);
        let s = utils::tag("}", s)?;
        Ok((s, Block { stmts }))
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{Expr, Number};

    use super::*;

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
}
