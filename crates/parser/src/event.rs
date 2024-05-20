use syntax::SyntaxKind;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Event {
    StartNode {
        kind: SyntaxKind,
        foward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Placeholder,
}
