lalrpop_mod!(pub instant);

#[cfg(test)]
fn test(input: &str, output: &str) {
    let res = instant::ProgramParser::new().parse(input).unwrap();
    let formated = format!("{:?}", res);
    assert_eq!(formated, output);
}

#[cfg(test)]
fn guard(input: &str) {
    let parser = instant::ProgramParser::new();
    assert!(parser.parse(input).is_err());
}

#[test]
fn simple_tests() {
    test("2+2", "[(2 + 2)]");
    test("2", "[2]")
}

#[test]
fn parentheses_tests() {
    test("22", "[22]");
    test("(22)", "[22]");
    test("(((22)))", "[22]");
    guard("((22)");
}

#[test]
fn variables_tests() {
    test("x=1", "[x = 1]");
    test("x=2+2", "[x = (2 + 2)]");
    test("x'=1", "[x' = 1]");
}

#[test]
fn semicolon_tests() {
    test("2;2", "[2, 2]");
    test("x = 22;\nx+1", "[x = 22, (x + 1)]");
    guard("2;");
}

#[test]
fn recursion_tests() {
    test("2+2+2", "[(2 + (2 + 2))]");
    test("2-2-2", "[((2 - 2) - 2)]");
    test("2*2*2", "[((2 * 2) * 2)]");
    test("2/2/2", "[((2 / 2) / 2)]");

    test("2-2+2", "[((2 - 2) + 2)]");
    test("2+2-2", "[(2 + (2 - 2))]");
    test("2*2-2+2", "[(((2 * 2) - 2) + 2)]");
}