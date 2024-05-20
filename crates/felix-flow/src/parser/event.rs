use syntax::SyntaxKind;

#[derive(Debug, PartialEq, Clone)]
pub(super) enum Event {
    StartNode {
        kind: SyntaxKind,
        foward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Placeholder,
}
