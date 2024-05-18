use crate::lexer::SyntaxKind;
use smol_str::SmolStr;

#[derive(Debug, PartialEq, Clone)]
pub(super) enum Event {
    StartNode {
        kind: SyntaxKind,
        foward_parent: Option<usize>,
    },
    AddToken {
        kind: SyntaxKind,
        text: SmolStr,
    },
    FinishNode,
    Placeholder,
}
