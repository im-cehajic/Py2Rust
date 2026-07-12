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
        Type::Vec(inner) => {
            let inner_str = type_to_rust_str(inner);
            match inner.as_ref() {
                Type::Inferred => "Vec<i32>".to_string(), // Default to i32 if inferred
                _ => format!("Vec<{}>", inner_str),
            }
        }
        Type::HashMap(k, v) => {
            let k_str = type_to_rust_str(k);
            let v_str = type_to_rust_str(v);
            format!("HashMap<{}, {}>", k_str, v_str)
        }
        Type::Custom(name) => name.clone(),
        Type::Inferred => "i32".to_string(), // Default inference
    }
}

/// Infer Rust type from optional IR type
pub fn infer_type_string(ty: Option<&Type>) -> String {
    match ty {
        Some(t) => type_to_rust_str(t),
        None => "i32".to_string(), // Default to i32
    }
}
