#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_block() {
        assert_eq!(Block::new("{}"), Ok(("", Block { expr: Vec::new() })))
    }
}
