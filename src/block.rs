use crate::{stmt::Smt, utils};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<Smt>,
}

impl Block {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("{}", s)?;
        Ok((s, Block { stmts: Vec::new() }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { stmts: Vec::new() })))
    }
}
