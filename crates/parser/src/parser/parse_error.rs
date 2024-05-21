use std::fmt::Display;

use syntax::SyntaxKind;
use text_size::TextRange;

#[derive(Debug, PartialEq)]
pub(crate) struct ParseError {
    pub(super) expected: Vec<SyntaxKind>,
    pub(super) found: Option<SyntaxKind>,
    pub(super) range: TextRange,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error at {}..{}: expected {}",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
            self.expected[0],
        )?;

        if let Some(found) = self.found {
            write!(f, ", but found {}", found)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range as StdRange;

    fn check(
        expected: Vec<SyntaxKind>,
        found: Option<SyntaxKind>,
        range: StdRange<u32>,
        output: &str,
    ) {
        let error = ParseError {
            expected,
            found,
            range: {
                let start = range.start.into();
                let end = range.end.into();
                TextRange::new(start, end)
            },
        };

        assert_eq!(format!("{}", error), output)
    }

    #[test]
    fn one_expected_did_find() {
        check(
            vec![SyntaxKind::Equals],
            Some(SyntaxKind::Ident),
            10..20,
            "error at 10..20: expected '=', but found identifier",
        )
    }

    #[test]
    fn one_expected_did_not_find() {
        check(
            vec![SyntaxKind::RParen],
            None,
            5..6,
            "error at 5..6: expected ')'",
        )
    }
}
