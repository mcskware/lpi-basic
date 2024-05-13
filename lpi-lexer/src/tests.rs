use super::*;

#[test]
fn test_print_statement() {
    let input = "10 PRINT \"HELLO, WORLD!\"";
    let expected = vec!["10", "PRINT", "\"HELLO, WORLD!\""];
    assert_eq!(lex(input), expected);
}

#[test]
fn test_identifiers() {
    let input = "LET A = 10";
    let expected = vec!["LET", "A", "=", "10"];
    assert_eq!(lex(input), expected);
}

#[test]
fn test_array_indexing() {
    let input = "AL(0) = 10";
    let expected = vec!["AL", "(", "0", ")", "=", "10"];
    assert_eq!(lex(input), expected);
}

#[test]
fn test_mathematical_expression() {
    let input = "A = 10 + 20";
    let expected = vec!["A", "=", "10", "+", "20"];
    assert_eq!(lex(input), expected);
}

#[test]
fn test_float_number() {
    let input = "A = 10.5";
    let expected = vec!["A", "=", "10.5"];
    assert_eq!(lex(input), expected);
}

#[test]
fn test_exponential_number() {
    let input = "A = 1.5E-10";
    let expected = vec!["A", "=", "1.5E-10"];
    assert_eq!(lex(input), expected);
}

#[test]
fn test_identifier_with_suffix() {
    let input = "A$ = 10";
    let expected = vec!["A$", "=", "10"];
    assert_eq!(lex(input), expected);
}

#[test]
fn test_bad_identifier() {
    let input = "A$B = 10";
    let expected = vec!["A$", "B", "=", "10"];
    assert_eq!(lex(input), expected);
}
