use std::vec::Vec;
use std::boxed::Box;
use std::fmt::{Display, Formatter, Error};
use std::collections::HashMap;

use super::ast::{Stmt, Expr, Oper};

#[derive(Copy, Clone)]
enum ValueType {
    Register(i32),
    Constant(i32)
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::ValueType::*;
        match *self {
            Register(reg) => write!(f, "%_{}", reg),
            Constant(val) => write!(f, "{}", val)
        }
    }
}

fn show(o: &Oper) -> &'static str {
    use self::Oper::*;
    match o {
        Add => "add",
        Sub => "sub",
        Mul => "mul",
        Div => "sdiv"
    }
}

pub struct Translator {
    output: String,
    variables: HashMap<String, ValueType>,
    regs_counter: i32
}

impl Translator {
    pub fn new() -> Translator {
        Translator { 
            output: String::new(),
            variables: HashMap::new(),
            regs_counter: 0
        }
    }

    fn tell(&mut self, instr: &str) {
        self.output += instr;
    }

    fn tell_instr(&mut self, instr: &str) {
        let formated = format!("\t{}\n", instr);
        self.tell(&formated)
    }

    fn push_variable(&mut self, id: &String, val: ValueType) {
        self.variables.insert(id.clone(), val);
    }

    fn new_register(&mut self) -> ValueType {
        let res = self.regs_counter;
        self.regs_counter += 1;
        ValueType::Register(res)
    }

    pub fn translate(mut self, tree: &Vec<Box<Stmt>>) -> Result<String, String> {
        self.tell("declare i32 @printf(i8*, ...)\n");
        self.tell("@formatString = private constant [4 x i8] c\"%d\\0A\\00\"\n");
        self.tell("define i32 @main() {\n");
        for stmt in tree {
            self.translate_stmt(stmt)?;
        }
        self.tell_instr("ret i32 0");
        self.tell("}\n");
        Ok(self.output)
    }

    fn translate_stmt(&mut self, stmt: &Box<Stmt>) -> Result<(), String> {
        use self::Stmt::*;
        match stmt.as_ref() {
            Ass(id, expr) => {
                let e = self.translate_expr(expr)?;
                self.push_variable(id, e);
                Ok(())
            },
            Expr(expr) => {
                let e = self.translate_expr(expr)?;
                let print_cmd = format!("call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @formatString, i32 0, i32 0), i32 {})", e);
                self.tell_instr(&print_cmd);
                Ok(())
            }
        }
    }

    fn translate_expr(&mut self, expr: &Box<Expr>) -> Result<ValueType, String> {
        use self::Expr::*;
        use self::ValueType::*;
        match expr.as_ref() {
            Num(val) => Ok(Constant(*val)),
            Ident(id) => {
                let val = self.variables.get(id).ok_or_else(||"Undeclared variable")?;
                Ok(*val)
            }
            Op(l, o, r) => {
                let lv = self.translate_expr(l)?;
                let rv = self.translate_expr(r)?;
                let reg = self.new_register();
                let cmd = format!("{} = {} i32 {}, {}", reg, show(o), lv, rv);
                self.tell_instr(&cmd);
                Ok(reg)
            }
        }
    }
}
