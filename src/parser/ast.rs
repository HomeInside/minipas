use std::fmt;

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
    BinaryOp { left: Box<Expr>, op: Op, right: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
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
    //Return(Expr), // 👈 NUEVO
    ProcDecl {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub params: Vec<String>, // o Vec<(String, VarType)> si quieres tipos
    pub body: Vec<Stmt>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>, // después puedes cambiar a Vec<(String, VarType)>
    pub return_type: VarType,
    pub body: Vec<Stmt>,
}
