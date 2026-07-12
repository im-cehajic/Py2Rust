//! Intermediate Representation (IR) for code
//!
//! This module defines language-agnostic IR structures that represent
//! the semantic meaning of code independent of source/target language.

pub mod class;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub functions: Vec<Function>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub params: Vec<VarDecl>,
    pub body: Vec<Statement>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarDecl {
    pub name: String,
    pub type_: Option<Type>,
    pub initializer: Option<Box<Expr>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    None,
    Vec(Box<Type>),
    HashMap(Box<Type>, Box<Type>),
    Custom(String),
    Inferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    VarDecl(VarDecl),
    Return(Option<Expr>),
    If {
        cond: Expr,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    While {
        cond: Expr,
        body: Vec<Statement>,
    },
    For {
        var: String,
        iter: Box<Expr>,
        body: Vec<Statement>,
    },
    Expr(Expr),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    None,
    Var(String),
    BinOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Call {
        func: String,
        args: Vec<Expr>,
    },
    Compare {
        left: Box<Expr>,
        op: CmpOp,
        right: Box<Expr>,
    },
    List(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Attribute {
        object: Box<Expr>,
        attr: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Neg,
    Pos,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CmpOp {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
}
