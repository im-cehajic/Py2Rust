//! Python parser module
//!
//! Converts Python source code to an Abstract Syntax Tree (AST),
//! then converts the AST to our Intermediate Representation (IR).

mod ast_visitor;
pub mod error;

use crate::error::{Error, Result};
use crate::ir::Module;
use ast_visitor::AstToIrVisitor;
use rustpython_parser::parse;

/// Parse Python source code into an AST
pub fn parse_python(source: &str) -> Result<rustpython_parser::ast::Mod> {
    parse(source, "<input>").map_err(|e| Error::ParseError(format!("{:?}", e)))
}

/// Convert Python AST to Intermediate Representation
pub fn ast_to_ir(ast: rustpython_parser::ast::Mod) -> Result<Module> {
    let mut visitor = AstToIrVisitor::new();
    visitor.visit_module(ast)
}
