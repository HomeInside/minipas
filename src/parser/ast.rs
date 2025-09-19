use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Integer,
    Real,
    Str,
    Boolean,
    Nil,
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Str(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Real(r) => write!(f, "{}", r),
            Value::Str(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64), // integer o real
    Ident(String),
    StringLiteral(String),
    BooleanLiteral(bool),
    BinaryOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    #[allow(dead_code)]
    Nil,
}

#[derive(Debug, Clone)]
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
    Mod,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    //VarDecl(Vec<String>),
    Assign(String, Expr),
    IfElse {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Block(Vec<Stmt>),
    Expr(Expr),
    Return(Expr),
    ProcDecl {
        // 👈 NUEVO
        name: String,
        params: Vec<(String, VarType)>, // nombre + tipo
        locals: Vec<(String, VarType)>, // variables locales con tipo
        body: Vec<Stmt>,
    },
    FuncDecl {
        name: String,
        params: Vec<(String, VarType)>,
        locals: Vec<(String, VarType)>,
        return_type: VarType,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub params: Vec<Param>, // 👈 NUEVO ahora incluyen tipos
    pub locals: Vec<Param>, // 👈 NUEVO locales
    pub body: Vec<Stmt>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub locals: Vec<Param>,
    pub return_type: VarType,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: VarType,
}
