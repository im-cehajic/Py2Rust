//! Type inference for Python code
//!
//! Attempts to infer types from usage patterns when explicit annotations are missing.

use crate::error::Result;
use crate::ir::{Expr, Module, Type};
use std::collections::HashMap;

/// Perform type inference on module
pub fn infer_types(module: &mut Module) -> Result<()> {
    let mut type_context = HashMap::new();

    // Infer types in function parameters and bodies
    for func in &mut module.functions {
        for stmt in &mut func.body {
            infer_statement_types(stmt, &mut type_context)?;
        }
        
        // Try to infer return type from return statements
        infer_return_type(func);
    }

    // Infer types in top-level statements
    for stmt in &mut module.statements {
        infer_statement_types(stmt, &mut type_context)?;
    }

    Ok(())
}

fn infer_statement_types(
    stmt: &mut crate::ir::Statement,
    type_context: &mut HashMap<String, Type>,
) -> Result<()> {
    match stmt {
        crate::ir::Statement::VarDecl(decl) => {
            if decl.type_.is_none() {
                if let Some(init) = &decl.initializer {
                    decl.type_ = infer_expr_type(init);
                    if let Some(ref ty) = decl.type_ {
                        type_context.insert(decl.name.clone(), ty.clone());
                    }
                }
            }
        }
        crate::ir::Statement::If {
            then_body,
            else_body,
            ..
        } => {
            for s in then_body {
                infer_statement_types(s, type_context)?;
            }
            if let Some(eb) = else_body {
                for s in eb {
                    infer_statement_types(s, type_context)?;
                }
            }
        }
        crate::ir::Statement::While { body, .. } | crate::ir::Statement::For { body, .. } => {
            for s in body {
                infer_statement_types(s, type_context)?;
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
        Expr::List(items) => {
            if items.is_empty() {
                Some(Type::Vec(Box::new(Type::Inferred)))
            } else if let Some(first_type) = infer_expr_type(&items[0]) {
                Some(Type::Vec(Box::new(first_type)))
            } else {
                Some(Type::Vec(Box::new(Type::Inferred)))
            }
        }
        Expr::Dict(_) => Some(Type::HashMap(
            Box::new(Type::String),
            Box::new(Type::Inferred),
        )),
        _ => None,
    }
}

fn infer_return_type(func: &mut crate::ir::Function) {
    for stmt in &func.body {
        if let crate::ir::Statement::Return(Some(expr)) = stmt {
            if let Some(ty) = infer_expr_type(expr) {
                func.return_type = Some(ty);
                return;
            }
        }
    }
}
