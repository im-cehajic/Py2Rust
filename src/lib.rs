//! Py2Rust: Python to Rust code converter
//!
//! This library provides a three-stage pipeline for converting Python code to Rust:
//! 1. Parse Python source to AST
//! 2. Convert AST to Intermediate Representation (IR)
//! 3. Generate Rust code from IR

pub mod cli;
pub mod error;
pub mod generator;
pub mod ir;
pub mod parser;
pub mod transforms;

pub use error::{Error, Result};

/// Convert Python source code to Rust
pub fn convert(python_source: &str) -> Result<String> {
    // Parse Python to AST
    let ast = parser::parse_python(python_source)?;
    
    // Convert AST to IR
    let mut ir_module = parser::ast_to_ir(ast)?;
    
    // Apply transformations (type inference, optimizations, etc.)
    ir_module = transforms::transform(ir_module)?;
    
    // Generate Rust code
    let rust_code = generator::generate(ir_module)?;
    
    Ok(rust_code)
}
