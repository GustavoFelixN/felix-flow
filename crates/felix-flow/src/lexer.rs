use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub(super) enum SyntaxKind {
    #[regex(" +")]
    WhiteSpace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_spaces() {
        let mut lexer = SyntaxKind::lexer("  ");

        assert_eq!(lexer.next(), Some(Ok(SyntaxKind::WhiteSpace)));
        assert_eq!(lexer.slice(), "  ");
    }
}
