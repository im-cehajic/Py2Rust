//! Expression code generation

use crate::error::Result;
use crate::ir::{BinOp, CmpOp, Expr, UnaryOp};

pub struct ExpressionGenerator;

impl ExpressionGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_expr(&mut self, expr: &Expr) -> Result<String> {
        match expr {
            Expr::Int(i) => Ok(i.to_string()),
            Expr::Float(f) => {
                let s = f.to_string();
                // Ensure floats always have a decimal point
                if s.contains('.') {
                    Ok(s)
                } else {
                    Ok(format!("{}.0", s))
                }
            }
            Expr::Bool(b) => Ok(b.to_string()),
            Expr::Str(s) => Ok(format!(r#"\"{}\""#, s.escape_default())),
            Expr::None => Ok("None".to_string()),
            Expr::Var(name) => Ok(name.clone()),
            Expr::BinOp { left, op, right } => {
                let left = self.generate_expr(left)?;
                let right = self.generate_expr(right)?;
                let op_str = self.binop_to_str(*op);
                Ok(format!("({} {} {})", left, op_str, right))
            }
            Expr::UnaryOp { op, operand } => {
                let operand = self.generate_expr(operand)?;
                let op_str = self.unaryop_to_str(*op);
                match op {
                    UnaryOp::Not => Ok(format!("!({})", operand)),
                    UnaryOp::Neg => Ok(format!("-({})", operand)),
                    UnaryOp::Pos => Ok(format!("+({})", operand)),
                }
            }
            Expr::Call { func, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| self.generate_expr(arg))
                    .collect::<Result<Vec<_>>>()?;
                Ok(format!("{}({})", func, args_str.join(", ")))
            }
            Expr::Compare { left, op, right } => {
                let left = self.generate_expr(left)?;
                let right = self.generate_expr(right)?;
                let op_str = self.cmpop_to_str(*op);
                Ok(format!("({} {} {})", left, op_str, right))
            }
            Expr::List(items) => {
                let items_str = items
                    .iter()
                    .map(|item| self.generate_expr(item))
                    .collect::<Result<Vec<_>>>()?;
                Ok(format!("vec![{}]", items_str.join(", ")))
            }
            Expr::Dict(pairs) => {
                let mut map_str = String::from("{");
                map_str.push_str("\n");
                for (k, v) in pairs {
                    let key = self.generate_expr(k)?;
                    let val = self.generate_expr(v)?;
                    map_str.push_str(&format!("    {}: {},\n", key, val));
                }
                map_str.push_str("}");
                Ok(format!("HashMap::from([{}])", map_str))
            }
            Expr::Index { object, index } => {
                let obj = self.generate_expr(object)?;
                let idx = self.generate_expr(index)?;
                Ok(format!("{}[{}]", obj, idx))
            }
            Expr::Attribute { object, attr } => {
                let obj = self.generate_expr(object)?;
                Ok(format!("{}.{}", obj, attr))
            }
        }
    }

    fn binop_to_str(&self, op: BinOp) -> &'static str {
        match op {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Pow => ".pow", // Will need special handling
        }
    }

    fn unaryop_to_str(&self, op: UnaryOp) -> &'static str {
        match op {
            UnaryOp::Not => "!",
            UnaryOp::Neg => "-",
            UnaryOp::Pos => "+",
        }
    }

    fn cmpop_to_str(&self, op: CmpOp) -> &'static str {
        match op {
            CmpOp::Eq => "==",
            CmpOp::NotEq => "!=",
            CmpOp::Lt => "<",
            CmpOp::LtE => "<=",
            CmpOp::Gt => ">",
            CmpOp::GtE => ">=",
        }
    }
}
