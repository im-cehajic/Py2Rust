//! Type mapping from Python/IR types to Rust types

use crate::ir::Type;

/// Map IR types to Rust type strings
pub fn type_to_rust_str(ty: &Type) -> String {
    match ty {
        Type::Int => "i32".to_string(),
        Type::Float => "f64".to_string(),
        Type::Bool => "bool".to_string(),
        Type::String => "String".to_string(),
        Type::None => "()".to_string(),
        Type::Vec(inner) => format!("Vec<{}>", type_to_rust_str(inner)),
        Type::HashMap(k, v) => format!(
            "HashMap<{}, {}>",
            type_to_rust_str(k),
            type_to_rust_str(v)
        ),
        Type::Custom(name) => name.clone(),
        Type::Inferred => "_".to_string(),
    }
}

/// Infer Rust type from optional IR type
pub fn infer_type_string(ty: Option<&Type>) -> String {
    match ty {
        Some(t) => type_to_rust_str(t),
        None => "_".to_string(), // Let Rust infer
    }
}
