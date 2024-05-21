use crate::parser::ParseError;
use syntax::SyntaxKind;

#[derive(Debug, PartialEq)]
pub(crate) enum Event {
    StartNode {
        kind: SyntaxKind,
        foward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Placeholder,
    Error(ParseError),
}
