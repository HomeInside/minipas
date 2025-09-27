use bincode::{Decode, Encode};
use std::fmt;

//#[derive(Debug, Clone, PartialEq)]
#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub enum VarType {
    Integer,
    Real,
    Str,
    Boolean,
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
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
            Value::Real(r) => {
                if r.fract() == 0.0 {
                    write!(f, "{:.1}", r)
                } else {
                    write!(f, "{}", r)
                }
            }
            Value::Str(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Value {
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Integer(i) => *i,
            Value::Real(f) => *f as i64,
            _ => panic!("No se puede convertir {:?} a Integer", self),
        }
    }

    #[allow(dead_code)]
    pub fn as_real(&self) -> f64 {
        match self {
            Value::Real(f) => *f,
            Value::Integer(i) => *i as f64,
            _ => panic!("No se puede convertir {:?} a Real", self),
        }
    }

    #[allow(dead_code)]
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => panic!("No se puede convertir {:?} a Boolean", self),
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            _ => panic!("No se puede convertir {:?} a String", self),
        }
    }
}

//#[derive(Debug, Clone)]
#[derive(Encode, Decode, Debug, Clone)]
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

//#[derive(Debug, Clone)]
#[derive(Encode, Decode, Debug, Clone)]
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

//#[derive(Debug, Clone)]
#[derive(Encode, Decode, Debug, Clone)]
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
    // 👇 Nuevo
    For {
        var: String,       // variable de control
        start: Expr,       // expresión inicial
        end: Expr,         // expresión final
        direction: ForDir, // to o downto
        body: Box<Stmt>,   // cuerpo (una stmt o bloque)
    },
    // 👇 Nuevo
    While(WhileStmt),
}

// 👇 Nuevo
// #[derive(Debug, Clone)]
//#[derive(Encode, Decode, Debug, Clone)]
#[derive(Encode, Decode, Debug, Clone)]
pub struct WhileStmt {
    pub condition: Expr, // la expresión booleana
    pub body: Box<Stmt>, // el bloque o statement a ejecutar
}

//#[derive(Debug, Clone, PartialEq)]
#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub enum ForDir {
    To,
    DownTo,
}

//#[derive(Debug, Clone)]
#[derive(Encode, Decode, Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub params: Vec<Param>,
    pub locals: Vec<Param>,
    pub body: Vec<Stmt>,
}

#[allow(dead_code)]
//#[derive(Debug, Clone)]
#[derive(Encode, Decode, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub locals: Vec<Param>,
    pub return_type: VarType,
    pub body: Vec<Stmt>,
}

//#[derive(Debug, Clone)]
#[derive(Encode, Decode, Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: VarType,
}
