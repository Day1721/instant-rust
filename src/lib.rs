#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub instant);

mod test;
mod ast;

mod llvm_comp;
mod jvm_comp;

use std::env;
use std::fs;
use std::path::{Path};

fn main() -> Result<(String,Vec<Box<ast::Stmt>>), String> {
    let path = env::args().nth(2).ok_or_else(||"File argument required")?;
    let contents = fs::read_to_string(&path).map_err(|err| err.to_string())?;

    let parser = instant::ProgramParser::new();
    let tree = parser.parse(&contents).map_err(|e| format!("Parsing error: {}", e))?;
    Ok((path, tree))
}

pub fn compile_llvm() {
    let (path_str, tree) = main().unwrap(); 
    let path = Path::new(&path_str);
    let translator = llvm_comp::Translator::new();
    let code = translator.translate(&tree).unwrap();

    path.parent().map_or((), 
        |dir| fs::create_dir_all(dir).unwrap()
    );
    let out = path.with_extension("ll");

    fs::write(out, code).unwrap();
}

pub fn compile_jvm() {
    let (path_str, mut tree) = main().unwrap(); 
    let path = Path::new(&path_str);
    let translator = jvm_comp::Translator::new();
    let filename = path.file_name().and_then(|n|n.to_str()).unwrap();
    let code = translator.translate(&mut tree, filename).unwrap();

    path.parent().map_or((), 
        |dir| fs::create_dir_all(dir).unwrap()
    );
    let out = path.with_extension("j");

    fs::write(out, code).unwrap();
}