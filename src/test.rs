lalrpop_mod!(pub instant);

fn test(input: &str, output: &str) {
    let res = instant::ProgramParser::new().parse(input).unwrap();
    let formated = format!("{:?}", res);
    assert_eq!(formated, output);
}

#[test]
fn simple_tests() {
    test("2+2+2", "[(2 + (2 + 2))]");
    test("2", "[2]")
}