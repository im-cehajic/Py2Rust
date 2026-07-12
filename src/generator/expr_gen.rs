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
            Expr::Float(f) => Ok(f.to_string()),
            Expr::Bool(b) => Ok(b.to_string()),
            Expr::Str(s) => Ok(format!(r#"\"{}\""#, s)),
            Expr::None => Ok("()".to_string()),
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
                Ok(format!("{}({})", op_str, operand))
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
        }
    }

    fn binop_to_str(&self, op: BinOp) -> &'static str {
        match op {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Pow => "** (not directly supported, use pow())",
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
