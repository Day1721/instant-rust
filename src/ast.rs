use std::fmt::{Debug, Formatter, Error};

pub enum Stmt {
    Ass(String, Box<Expr>),
    Expr(Box<Expr>)
}

pub enum Expr {
    Num(i32),
    Ident(String),
    Op(Box<Expr>, Oper, Box<Expr>)
}

#[derive(Copy, Clone)]
pub enum Oper {
    Add,
    Sub,
    Mul,
    Div
}



impl Debug for Stmt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::Stmt::*;
        match *self {
            Ass(ref x, ref e) => write!(f, "{} = {:?}", x, e),
            Expr(ref e) => write!(f, "{:?}", e)
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Num(i) => write!(f, "{:?}", i),
            Ident(ref s) => write!(f, "{}", s),
            Op(ref l, o, ref r) => write!(f, "({:?} {:?} {:?})", l, o, r)
        }
    }
}

impl Debug for Oper {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::Oper::*;
        match *self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/")
        }
    }
}