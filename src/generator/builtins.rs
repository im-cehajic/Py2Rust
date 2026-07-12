//! Python-specific built-in function mappings

use crate::error::{Error, Result};
use crate::ir::Expr;

/// Map Python built-in functions to Rust equivalents
pub fn map_builtin(func_name: &str, args: &[Expr]) -> Result<String> {
    match func_name {
        "print" => {
            if args.is_empty() {
                Ok("println!()".to_string())
            } else {
                let arg_strs: Result<Vec<_>> = args
                    .iter()
                    .map(|arg| expr_to_string(arg))
                    .collect();
                Ok(format!("println!(\"{{:?}}\", {})", arg_strs?.join(", ")))
            }
        }
        "len" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "len() requires exactly 1 argument".to_string(),
                ));
            }
            let arg = expr_to_string(&args[0])?;
            Ok(format!("{}.len()", arg))
        }
        "range" => {
            if args.is_empty() || args.len() > 3 {
                return Err(Error::GenerationError(
                    "range() requires 1-3 arguments".to_string(),
                ));
            }
            let arg_strs: Result<Vec<_>> = args
                .iter()
                .map(|arg| expr_to_string(arg))
                .collect();
            match args.len() {
                1 => Ok(format!("0..{}", arg_strs?[0])),
                2 => Ok(format!("{}..{}", arg_strs?[0], arg_strs?[1])),
                3 => Ok(format!("({}..{}).step_by({})", arg_strs?[0], arg_strs?[1], arg_strs?[2])),
                _ => unreachable!(),
            }
        }
        "str" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "str() requires exactly 1 argument".to_string(),
                ));
            }
            let arg = expr_to_string(&args[0])?;
            Ok(format!("{}.to_string()", arg))
        }
        "int" => {
            if args.is_empty() || args.len() > 2 {
                return Err(Error::GenerationError(
                    "int() requires 1-2 arguments".to_string(),
                ));
            }
            let arg = expr_to_string(&args[0])?;
            Ok(format!("{}.parse::<i32>().unwrap()", arg))
        }
        "float" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "float() requires exactly 1 argument".to_string(),
                ));
            }
            let arg = expr_to_string(&args[0])?;
            Ok(format!("{}.parse::<f64>().unwrap()", arg))
        }
        "bool" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "bool() requires exactly 1 argument".to_string(),
                ));
            }
            let arg = expr_to_string(&args[0])?;
            Ok(format!("!{}.is_empty()", arg))
        }
        "abs" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "abs() requires exactly 1 argument".to_string(),
                ));
            }
            let arg = expr_to_string(&args[0])?;
            Ok(format!("{}.abs()", arg))
        }
        "max" => {
            if args.is_empty() {
                return Err(Error::GenerationError(
                    "max() requires at least 1 argument".to_string(),
                ));
            }
            let arg_strs: Result<Vec<_>> = args
                .iter()
                .map(|arg| expr_to_string(arg))
                .collect();
            Ok(format!("[{}].iter().max().unwrap()", arg_strs?.join(", ")))
        }
        "min" => {
            if args.is_empty() {
                return Err(Error::GenerationError(
                    "min() requires at least 1 argument".to_string(),
                ));
            }
            let arg_strs: Result<Vec<_>> = args
                .iter()
                .map(|arg| expr_to_string(arg))
                .collect();
            Ok(format!("[{}].iter().min().unwrap()", arg_strs?.join(", ")))
        }
        "sum" => {
            if args.len() != 1 {
                return Err(Error::GenerationError(
                    "sum() requires exactly 1 argument".to_string(),
                ));
            }
            let arg = expr_to_string(&args[0])?;
            Ok(format!("{}.iter().sum()", arg))
        }
        "sorted" => {
            if args.len() < 1 || args.len() > 2 {
                return Err(Error::GenerationError(
                    "sorted() requires 1-2 arguments".to_string(),
                ));
            }
            let arg = expr_to_string(&args[0])?;
            Ok(format!("{{ let mut v = {}; v.sort(); v }}", arg))
        }
        _ => Err(Error::GenerationError(format!(
            "Unknown built-in function: {}",
            func_name
        ))),
    }
}

fn expr_to_string(expr: &Expr) -> Result<String> {
    use crate::ir::Expr as E;
    match expr {
        E::Int(i) => Ok(i.to_string()),
        E::Float(f) => Ok(f.to_string()),
        E::Str(s) => Ok(format!(r#"\"{}\""#, s)),
        E::Var(v) => Ok(v.clone()),
        _ => Err(Error::GenerationError(
            "Complex expressions in builtins not yet supported".to_string(),
        )),
    }
}
