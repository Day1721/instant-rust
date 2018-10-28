#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub instant); // synthesized by LALRPOP

mod test;
mod ast;

#[test]
fn instant() {
    assert!(instant::StmtParser::new().parse("22").is_ok());
    assert!(instant::StmtParser::new().parse("(22)").is_ok());
    assert!(instant::StmtParser::new().parse("((((22))))").is_ok());
    assert!(instant::StmtParser::new().parse("((22)").is_err());
}

fn main() {
    println!("Hello, world!");
}
