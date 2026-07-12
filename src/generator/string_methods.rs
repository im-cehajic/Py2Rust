//! String method support

use crate::error::{Error, Result};

/// Map Python string methods to Rust
pub fn map_string_method(method_name: &str, object: &str, args: &[String]) -> Result<String> {
    match method_name {
        "upper" => Ok(format!("{}.to_uppercase()", object)),
        "lower" => Ok(format!("{}.to_lowercase()", object)),
        "strip" => Ok(format!("{}.trim().to_string()", object)),
        "replace" => {
            if args.len() != 2 {
                return Err(Error::GenerationError(
                    "replace() requires 2 arguments".to_string(),
                ));
            }
            Ok(format!(
                "{}.replace({}, {})",
                object, args[0], args[1]
            ))
        }
        "split" => {
            if args.is_empty() {
                Ok(format!("{}.split_whitespace().collect::<Vec<_>>()", object))
            } else {
                Ok(format!(
                    "{}.split({}).collect::<Vec<_>>()",
                    object, args[0]
                ))
            }
        }
        "startswith" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "startswith() requires 1 argument".to_string(),
                ));
            }
            Ok(format!("{}.starts_with({})", object, args[0]))
        }
        "endswith" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "endswith() requires 1 argument".to_string(),
                ));
            }
            Ok(format!("{}.ends_with({})", object, args[0]))
        }
        "contains" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "contains() requires 1 argument".to_string(),
                ));
            }
            Ok(format!("{}.contains({})", object, args[0]))
        }
        _ => Err(Error::GenerationError(format!(
            "Unknown string method: {}",
            method_name
        ))),
    }
}
