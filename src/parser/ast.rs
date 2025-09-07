#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Ident(String),
    StringLiteral(String),
    BinaryOp { left: Box<Expr>, op: Op, right: Box<Expr> },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub enum Stmt {
    //VarDecl(Vec<String>),
    Assign(String, Expr),
    WritelnList(Vec<Expr>), // nuevo
    IfElse {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Block(Vec<Stmt>),
}
