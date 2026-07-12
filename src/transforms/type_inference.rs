//! Type inference for Python code
//!
//! Attempts to infer types from usage patterns when explicit annotations are missing.

use crate::error::Result;
use crate::ir::{Expr, Module, Type};

/// Perform type inference on module
pub fn infer_types(module: &mut Module) -> Result<()> {
    // Infer types in function parameters and bodies
    for func in &mut module.functions {
        for stmt in &mut func.body {
            infer_statement_types(stmt)?;
        }
    }

    // Infer types in top-level statements
    for stmt in &mut module.statements {
        infer_statement_types(stmt)?;
    }

    Ok(())
}

fn infer_statement_types(stmt: &mut crate::ir::Statement) -> Result<()> {
    match stmt {
        crate::ir::Statement::VarDecl(decl) => {
            if decl.type_.is_none() {
                if let Some(init) = &decl.initializer {
                    decl.type_ = infer_expr_type(init);
                }
            }
        }
        crate::ir::Statement::If {
            then_body,
            else_body,
            ..
        } => {
            for s in then_body {
                infer_statement_types(s)?;
            }
            if let Some(eb) = else_body {
                for s in eb {
                    infer_statement_types(s)?;
                }
            }
        }
        crate::ir::Statement::While { body, .. } | crate::ir::Statement::For { body, .. } => {
            for s in body {
                infer_statement_types(s)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn infer_expr_type(expr: &Expr) -> Option<Type> {
    match expr {
        Expr::Int(_) => Some(Type::Int),
        Expr::Float(_) => Some(Type::Float),
        Expr::Bool(_) => Some(Type::Bool),
        Expr::Str(_) => Some(Type::String),
        Expr::None => Some(Type::None),
        _ => None,
    }
}
