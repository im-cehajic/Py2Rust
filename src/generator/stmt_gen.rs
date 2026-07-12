//! Statement code generation

use crate::error::Result;
use crate::ir::{Statement, VarDecl};
use super::expr_gen::ExpressionGenerator;

pub struct StatementGenerator {
    expr_gen: ExpressionGenerator,
}

impl StatementGenerator {
    pub fn new() -> Self {
        Self {
            expr_gen: ExpressionGenerator::new(),
        }
    }

    pub fn generate_statement(&mut self, stmt: &Statement) -> Result<String> {
        match stmt {
            Statement::VarDecl(decl) => self.generate_var_decl(decl),
            Statement::Return(expr) => self.generate_return(expr.as_ref()),
            Statement::If {
                cond,
                then_body,
                else_body,
            } => self.generate_if(cond, then_body, else_body.as_ref()),
            Statement::While { cond, body } => self.generate_while(cond, body),
            Statement::For { var, iter, body } => self.generate_for(var, iter, body),
            Statement::Expr(expr) => self.expr_gen.generate_expr(expr),
        }
    }

    fn generate_var_decl(&mut self, decl: &VarDecl) -> Result<String> {
        let mut output = format!("let {}", decl.name);

        // Add type annotation if present
        if let Some(ty) = &decl.type_ {
            output.push_str(": ");
            output.push_str(&super::type_map::type_to_rust_str(ty));
        }

        // Add initializer if present
        if let Some(init) = &decl.initializer {
            output.push_str(" = ");
            output.push_str(&self.expr_gen.generate_expr(init)?);
        }

        output.push_str(";\n");
        Ok(output)
    }

    fn generate_return(&mut self, expr: Option<&crate::ir::Expr>) -> Result<String> {
        if let Some(e) = expr {
            Ok(format!("return {};\n", self.expr_gen.generate_expr(e)?))
        } else {
            Ok("return;\n".to_string())
        }
    }

    fn generate_if(
        &mut self,
        cond: &crate::ir::Expr,
        then_body: &[Statement],
        else_body: Option<&Vec<Statement>>,
    ) -> Result<String> {
        let mut output = format!("if {} {{", self.expr_gen.generate_expr(cond)?);
        output.push('\n');

        for stmt in then_body {
            let stmt_code = self.generate_statement(stmt)?;
            for line in stmt_code.lines() {
                if !line.is_empty() {
                    output.push_str("    ");
                    output.push_str(line);
                    output.push('\n');
                }
            }
        }

        if let Some(else_stmts) = else_body {
            output.push_str("} else {\n");
            for stmt in else_stmts {
                let stmt_code = self.generate_statement(stmt)?;
                for line in stmt_code.lines() {
                    if !line.is_empty() {
                        output.push_str("    ");
                        output.push_str(line);
                        output.push('\n');
                    }
                }
            }
        }

        output.push_str("}\n");
        Ok(output)
    }

    fn generate_while(&mut self, cond: &crate::ir::Expr, body: &[Statement]) -> Result<String> {
        let mut output = format!("while {} {{", self.expr_gen.generate_expr(cond)?);
        output.push('\n');

        for stmt in body {
            let stmt_code = self.generate_statement(stmt)?;
            for line in stmt_code.lines() {
                if !line.is_empty() {
                    output.push_str("    ");
                    output.push_str(line);
                    output.push('\n');
                }
            }
        }

        output.push_str("}\n");
        Ok(output)
    }

    fn generate_for(
        &mut self,
        var: &str,
        iter: &crate::ir::Expr,
        body: &[Statement],
    ) -> Result<String> {
        let mut output = format!(
            "for {} in {} {{",
            var,
            self.expr_gen.generate_expr(iter)?
        );
        output.push('\n');

        for stmt in body {
            let stmt_code = self.generate_statement(stmt)?;
            for line in stmt_code.lines() {
                if !line.is_empty() {
                    output.push_str("    ");
                    output.push_str(line);
                    output.push('\n');
                }
            }
        }

        output.push_str("}\n");
        Ok(output)
    }
}
