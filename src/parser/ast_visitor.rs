//! Visitor pattern implementation for converting Python AST to IR

use crate::error::{Error, Result};
use crate::ir::{
    BinOp, Expr, Function, Module, Statement, Type, UnaryOp, VarDecl,
};
use rustpython_parser::ast::{
    Arguments, Constant, Expr as PyExpr, ExprKind, Mod, Stmt, StmtKind,
};

pub struct AstToIrVisitor {
    functions: Vec<Function>,
    statements: Vec<Statement>,
}

impl AstToIrVisitor {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            statements: Vec::new(),
        }
    }

    pub fn visit_module(&mut self, module: Mod) -> Result<Module> {
        match module {
            Mod::Module(body) => {
                for stmt in body {
                    self.visit_stmt(stmt)?;
                }
            }
            _ => return Err(Error::ParseError("Unsupported module type".to_string())),
        }

        Ok(Module {
            functions: self.functions.clone(),
            statements: self.statements.clone(),
        })
    }

    fn visit_stmt(&mut self, stmt: Stmt) -> Result<()> {
        match stmt.node {
            StmtKind::FunctionDef {
                name,
                args,
                body,
                decorator_list: _,
                returns: _,
            } => {
                let params = self.visit_arguments(args)?;
                let mut func_body = Vec::new();
                for s in body {
                    func_body.push(self.visit_stmt_to_ir(s)?);
                }

                self.functions.push(Function {
                    name,
                    params,
                    body: func_body,
                    return_type: None, // Inferred later
                });
            }
            StmtKind::Return { value } => {
                if let Some(expr) = value {
                    let e = self.visit_expr(expr)?;
                    self.statements
                        .push(Statement::Return(Some(e)));
                } else {
                    self.statements.push(Statement::Return(None));
                }
            }
            StmtKind::Assign { targets, value, .. } => {
                let expr = self.visit_expr(value)?;
                for target in targets {
                    if let PyExpr {
                        node: ExprKind::Name { id, .. },
                        ..
                    } = target
                    {
                        self.statements.push(Statement::VarDecl(VarDecl {
                            name: id,
                            type_: None, // Inferred later
                            initializer: Some(Box::new(expr.clone())),
                        }));
                    }
                }
            }
            StmtKind::If {
                test,
                body,
                orelse,
            } => {
                let cond = self.visit_expr(test)?;
                let mut then_body = Vec::new();
                for s in body {
                    then_body.push(self.visit_stmt_to_ir(s)?);
                }
                let else_body = if !orelse.is_empty() {
                    let mut eb = Vec::new();
                    for s in orelse {
                        eb.push(self.visit_stmt_to_ir(s)?);
                    }
                    Some(eb)
                } else {
                    None
                };

                self.statements
                    .push(Statement::If { cond, then_body, else_body });
            }
            StmtKind::While { test, body, .. } => {
                let cond = self.visit_expr(test)?;
                let mut loop_body = Vec::new();
                for s in body {
                    loop_body.push(self.visit_stmt_to_ir(s)?);
                }
                self.statements
                    .push(Statement::While { cond, body: loop_body });
            }
            StmtKind::For {
                target,
                iter,
                body,
                ..
            } => {
                let var_name = match target.node {
                    ExprKind::Name { id, .. } => id,
                    _ => return Err(Error::ParseError("Complex for targets not supported".into())),
                };
                let iterable = self.visit_expr(iter)?;
                let mut loop_body = Vec::new();
                for s in body {
                    loop_body.push(self.visit_stmt_to_ir(s)?);
                }
                self.statements.push(Statement::For {
                    var: var_name,
                    iter: Box::new(iterable),
                    body: loop_body,
                });
            }
            _ => {
                // Ignore other statement types for now
            }
        }
        Ok(())
    }

    fn visit_stmt_to_ir(&mut self, stmt: Stmt) -> Result<Statement> {
        match stmt.node {
            StmtKind::Return { value } => {
                if let Some(expr) = value {
                    let e = self.visit_expr(expr)?;
                    Ok(Statement::Return(Some(e)))
                } else {
                    Ok(Statement::Return(None))
                }
            }
            StmtKind::Assign { targets, value, .. } => {
                let expr = self.visit_expr(value)?;
                let target = targets.into_iter().next().ok_or_else(|| {
                    Error::ParseError("Assign with no targets".into())
                })?;

                let name = match target.node {
                    ExprKind::Name { id, .. } => id,
                    _ => return Err(Error::ParseError("Complex assign targets not supported".into())),
                };

                Ok(Statement::VarDecl(VarDecl {
                    name,
                    type_: None,
                    initializer: Some(Box::new(expr)),
                }))
            }
            StmtKind::If {
                test,
                body,
                orelse,
            } => {
                let cond = self.visit_expr(test)?;
                let mut then_body = Vec::new();
                for s in body {
                    then_body.push(self.visit_stmt_to_ir(s)?);
                }
                let else_body = if !orelse.is_empty() {
                    let mut eb = Vec::new();
                    for s in orelse {
                        eb.push(self.visit_stmt_to_ir(s)?);
                    }
                    Some(eb)
                } else {
                    None
                };

                Ok(Statement::If {
                    cond,
                    then_body,
                    else_body,
                })
            }
            _ => Err(Error::ParseError("Unsupported statement type".into())),
        }
    }

    fn visit_arguments(&mut self, args: Arguments) -> Result<Vec<VarDecl>> {
        let mut params = Vec::new();
        for arg in args.args {
            params.push(VarDecl {
                name: arg.arg,
                type_: None, // Will be inferred
                initializer: None,
            });
        }
        Ok(params)
    }

    fn visit_expr(&mut self, expr: PyExpr) -> Result<Expr> {
        match expr.node {
            ExprKind::Constant { value, .. } => self.visit_constant(value),
            ExprKind::Name { id, .. } => Ok(Expr::Var(id)),
            ExprKind::BinOp { left, op, right } => {
                let left = self.visit_expr(*left)?;
                let right = self.visit_expr(*right)?;
                let bin_op = self.visit_binop(op)?;
                Ok(Expr::BinOp {
                    left: Box::new(left),
                    op: bin_op,
                    right: Box::new(right),
                })
            }
            ExprKind::UnaryOp { op, operand } => {
                let operand = self.visit_expr(*operand)?;
                let unary_op = self.visit_unaryop(op)?;
                Ok(Expr::UnaryOp {
                    op: unary_op,
                    operand: Box::new(operand),
                })
            }
            ExprKind::Call { func, args, .. } => {
                let func_name = match func.node {
                    ExprKind::Name { id, .. } => id,
                    _ => return Err(Error::ParseError("Complex function calls not supported".into())),
                };
                let mut call_args = Vec::new();
                for arg in args {
                    call_args.push(self.visit_expr(arg)?);
                }
                Ok(Expr::Call {
                    func: func_name,
                    args: call_args,
                })
            }
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => {
                let left = self.visit_expr(*left)?;
                let op = self.visit_cmpop(
                    ops.into_iter().next().ok_or_else(|| {
                        Error::ParseError("Compare with no ops".into())
                    })?,
                )?;
                let right = self.visit_expr(
                    comparators
                        .into_iter()
                        .next()
                        .ok_or_else(|| Error::ParseError("Compare with no comparators".into()))?,
                )?;
                Ok(Expr::Compare {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                })
            }
            ExprKind::List { elts, .. } => {
                let mut items = Vec::new();
                for elt in elts {
                    items.push(self.visit_expr(elt)?);
                }
                Ok(Expr::List(items))
            }
            ExprKind::Dict { keys, values } => {
                let mut pairs = Vec::new();
                for (k, v) in keys.into_iter().zip(values.into_iter()) {
                    let key = self.visit_expr(k)?;
                    let val = self.visit_expr(v)?;
                    pairs.push((key, val));
                }
                Ok(Expr::Dict(pairs))
            }
            ExprKind::Subscript { value, slice, .. } => {
                let obj = self.visit_expr(*value)?;
                let idx = self.visit_expr(*slice)?;
                Ok(Expr::Index {
                    object: Box::new(obj),
                    index: Box::new(idx),
                })
            }
            ExprKind::Attribute { value, attr, .. } => {
                let obj = self.visit_expr(*value)?;
                Ok(Expr::Attribute {
                    object: Box::new(obj),
                    attr,
                })
            }
            _ => Err(Error::ParseError(
                "Unsupported expression type".to_string(),
            )),
        }
    }

    fn visit_constant(&mut self, value: rustpython_parser::ast::Constant) -> Result<Expr> {
        use rustpython_parser::ast::Constant as PyConstant;
        match value {
            PyConstant::Int(i) => Ok(Expr::Int(i.try_into().unwrap_or(0))),
            PyConstant::Float(f) => Ok(Expr::Float(f)),
            PyConstant::Str(s) => Ok(Expr::Str(s)),
            PyConstant::Bool(b) => Ok(Expr::Bool(b)),
            PyConstant::None => Ok(Expr::None),
            _ => Err(Error::ParseError("Unsupported constant type".into())),
        }
    }

    fn visit_binop(
        &mut self,
        op: rustpython_parser::ast::Operator,
    ) -> Result<BinOp> {
        use rustpython_parser::ast::Operator as PyOp;
        match op {
            PyOp::Add => Ok(BinOp::Add),
            PyOp::Sub => Ok(BinOp::Sub),
            PyOp::Mult => Ok(BinOp::Mul),
            PyOp::Div => Ok(BinOp::Div),
            PyOp::Mod => Ok(BinOp::Mod),
            PyOp::Pow => Ok(BinOp::Pow),
            _ => Err(Error::ParseError("Unsupported binary operator".into())),
        }
    }

    fn visit_unaryop(
        &mut self,
        op: rustpython_parser::ast::Unaryop,
    ) -> Result<UnaryOp> {
        use rustpython_parser::ast::Unaryop as PyUnOp;
        match op {
            PyUnOp::Not => Ok(UnaryOp::Not),
            PyUnOp::USub => Ok(UnaryOp::Neg),
            PyUnOp::UAdd => Ok(UnaryOp::Pos),
            _ => Err(Error::ParseError("Unsupported unary operator".into())),
        }
    }

    fn visit_cmpop(
        &mut self,
        op: rustpython_parser::ast::Cmpop,
    ) -> Result<crate::ir::CmpOp> {
        use rustpython_parser::ast::Cmpop as PyCmpOp;
        match op {
            PyCmpOp::Eq => Ok(crate::ir::CmpOp::Eq),
            PyCmpOp::NotEq => Ok(crate::ir::CmpOp::NotEq),
            PyCmpOp::Lt => Ok(crate::ir::CmpOp::Lt),
            PyCmpOp::LtE => Ok(crate::ir::CmpOp::LtE),
            PyCmpOp::Gt => Ok(crate::ir::CmpOp::Gt),
            PyCmpOp::GtE => Ok(crate::ir::CmpOp::GtE),
            _ => Err(Error::ParseError("Unsupported comparison operator".into())),
        }
    }
}
