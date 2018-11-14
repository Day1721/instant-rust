use std::collections::HashMap;

use super::ast::{Stmt, Expr, Oper};

fn show(o: &Oper) -> &'static str {
    use self::Oper::*;
    match o {
        Add => "iadd",
        Sub => "isub",
        Mul => "imul",
        Div => "idiv"
    }
}

pub struct Translator {
    output: String,
    variables: HashMap<String, i32>,
}

impl Translator {
    pub fn new() -> Translator {
        Translator {
            output: String::new(),
            variables: HashMap::new()
        }
    }

    pub fn translate(mut self, tree: &mut Vec<Box<Stmt>>, name: &str) -> Result<String, String> {
        Optimizer::new().optimize(tree);

        for stmt in tree {
            self.translate_stmt(stmt)?;
        }
        Ok(self.output)
    }
    
    fn translate_stmt(&mut self, stmt: &Box<Stmt>) -> Result<(), String> {
        Ok(())
    }
}


struct Optimizer {
    variables: Vec<String>
}

impl Optimizer {
    pub fn new() -> Optimizer {
        Optimizer {
            variables: Vec::new()
        }
    }

    pub fn optimize(&mut self, tree: &mut Vec<Box<Stmt>>) {
        for stmt in tree {
            self.optimize_stmt(stmt);
        }
    }

    fn optimize_stmt(&mut self, stmt: &mut Box<Stmt>) {
        
    }
}