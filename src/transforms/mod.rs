//! Intermediate Representation transformations
//!
//! Performs analysis and transformations on IR to improve code generation:
//! - Type inference
//! - Ownership analysis
//! - Optimization passes

pub mod type_inference;

use crate::error::Result;
use crate::ir::Module;

/// Run all transformations on the IR module
pub fn transform(mut module: Module) -> Result<Module> {
    // Run type inference
    type_inference::infer_types(&mut module)?;
    
    Ok(module)
}
