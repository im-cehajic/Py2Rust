//! Rust code generation from Intermediate Representation
//!
//! Converts IR structures into valid, idiomatic Rust code.

mod expr_gen;
mod stmt_gen;
mod type_map;

use crate::error::{Error, Result};
use crate::ir::{Function, Module, Statement};
use stmt_gen::StatementGenerator;

/// Generate Rust code from IR module
pub fn generate(module: Module) -> Result<String> {
    let mut output = String::new();

    // Generate all functions
    for func in module.functions {
        output.push_str(&generate_function(&func)?);n        output.push_str("\n\n");
    }

    // Generate main function if there are top-level statements
    if !module.statements.is_empty() {
        output.push_str("fn main() {\n");
        let mut stmt_gen = StatementGenerator::new();
        for stmt in module.statements {
            let stmt_code = stmt_gen.generate_statement(&stmt)?;
            output.push_str(&indent(&stmt_code, 1));
        }
        output.push_str("}");
    }

    Ok(output)
}

/// Generate a single Rust function
fn generate_function(func: &Function) -> Result<String> {
    let mut output = String::new();

    // Function signature
    output.push_str(&format!("fn {}(", func.name));
    
    for (i, param) in func.params.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        output.push_str(&format!("${}: ", param.name));
        output.push_str(&type_map::infer_type_string(param.type_.as_ref()));
    }

    output.push_str(") ");

    // Return type
    if let Some(ret_type) = &func.return_type {
        output.push_str("-> ");
        output.push_str(&type_map::type_to_rust_str(ret_type));
        output.push_str(" ");
    }

    output.push_str("{\n");

    // Function body
    let mut stmt_gen = StatementGenerator::new();
    for stmt in &func.body {
        let stmt_code = stmt_gen.generate_statement(stmt)?;
        output.push_str(&indent(&stmt_code, 1));
    }

    output.push_str("}");

    Ok(output)
}

/// Indent a string by n levels (each level = 4 spaces)
fn indent(s: &str, levels: usize) -> String {
    let indent_str = "    ".repeat(levels);
    s.lines()
        .map(|line| {
            if line.trim().is_empty() {
                String::new()
            } else {
                format!("{}{}\n", indent_str, line)
            }
        })
        .collect()
}
