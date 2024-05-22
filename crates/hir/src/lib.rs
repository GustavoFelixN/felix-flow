use smol_str::SmolStr;

#[derive(Debug)]
pub enum Stmt {
    VariableDef { name: SmolStr, value: Expr },
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    Literal {
        n: u64,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Self>,
    },
    VariableRef {
        var: SmolStr,
    },
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}
