#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Val {
    Number(i32),
    //WARN: this is basically a null righ now. remove later
    Unit,
}
