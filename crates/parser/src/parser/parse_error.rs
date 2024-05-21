use syntax::SyntaxKind;
use text_size::TextRange;

#[derive(Debug, PartialEq)]
pub(crate) struct ParseError {
    pub(super) expected: Vec<SyntaxKind>,
    pub(super) found: Option<SyntaxKind>,
    pub(super) range: TextRange,
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::ops::Range as StdRange;
    //
    // fn check(
    //     expected: Vec<SyntaxKind>,
    //     found: Option<SyntaxKind>,
    //     range: StdRange<u32>,
    //     output: &str,
    // ) {
    //     let error = ParseError {
    //         expected,
    //         found,
    //         range: {
    //             let start = range.start.into();
    //             let end = range.end.into();
    //             TextRange::new(start, end)
    //         },
    //     };
    //
    //     assert_eq!(format!("{}", error), output)
    // }
    //
    // #[test]
    // fn one_expected_did_find() {
    //     check(
    //         vec![SyntaxKind::Equals],
    //         Some(SyntaxKind::Ident),
    //         10..20,
    //         "error at 10..20: expected '=', but found identifier",
    //     )
    // }
}
