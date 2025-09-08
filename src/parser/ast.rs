#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Integer,
    Real,
    Str,
    Boolean,
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Str(String),
    Boolean(bool),
}

impl Value {
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Real(f) => format!("{:.4}", f),
            Value::Str(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Number(f64), // integer o real
    Ident(String),
    StringLiteral(String),
    BooleanLiteral(bool), // nuevo
    BinaryOp { left: Box<Expr>, op: Op, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> }, // 👈 nuevo
                                            //FuncCall(String, Vec<Expr>),            // <-- nueva rama
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
