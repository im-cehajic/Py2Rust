//! Class definition support in IR

use serde::{Deserialize, Serialize};
use crate::ir::{Statement, VarDecl};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub fields: Vec<VarDecl>,
    pub methods: Vec<ClassMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<VarDecl>,
    pub body: Vec<Statement>,
    pub is_static: bool,
}
