
use crate::errors::AppError;
use syn::{visit::Visit, ItemFn, Stmt, Expr, File};
use std::path::PathBuf;


#[derive(Clone, Debug)]
pub struct FunctionAst {
    pub name: String,
    pub calls: Vec<String>,
}


#[derive(Clone, Debug)]
pub struct FileAst {
    pub file_path: PathBuf,
    pub functions: Vec<FunctionAst>,
}


pub fn extract_ast_info(file_path: PathBuf, content: &str) -> Result<FileAst, AppError> {
    let syntax: File = syn::parse_file(content)
            .map_err(|e| AppError::Generic(format!("Parse error in {}: {}", file_path.display(), e)))?;

    let mut visitor = FunctionVisitor {
        functions: Vec::new(),
    };

  
    visitor.visit_file(&syntax);


    let file_ast = FileAst {
        file_path,
        functions: visitor.functions,
    };

    Ok(file_ast)
}

struct FunctionVisitor {
    pub functions: Vec<FunctionAst>,
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let func_name = node.sig.ident.to_string();
        let mut calls = Vec::new();

        if let Some(block) = &node.block.stmts.get(0..) {
            for stmt in *block {
                let sub_visitor = CallVisitor { calls: Vec::new() };
                let mut sub_visitor = sub_visitor;
                sub_visitor.visit_stmt(stmt);
                calls.extend(sub_visitor.calls);
            }
        }

        let function_ast = FunctionAst {
            name: func_name,
            calls,
        };
        self.functions.push(function_ast);

        syn::visit::visit_item_fn(self, node);
    }
}

struct CallVisitor {
    pub calls: Vec<String>,
}

impl<'ast> Visit<'ast> for CallVisitor {
    fn visit_expr(&mut self, node: &'ast Expr) {
        match node {
            Expr::Call(call_expr) => {
                if let Expr::Path(expr_path) = &*call_expr.func {
                    if let Some(ident) = expr_path.path.segments.last() {
                        self.calls.push(ident.ident.to_string());
                    }
                }
            }
            _ => {}
        }
        syn::visit::visit_expr(self, node);
    }

    fn visit_stmt(&mut self, node: &'ast Stmt) {
        syn::visit::visit_stmt(self, node);
    }
}
