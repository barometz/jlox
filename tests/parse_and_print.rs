use jlox::{self, expr::Expr, parser::ParserError};

fn parse(source: &str) -> Result<Expr, ParserError> {
    let mut scanner = jlox::scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens().unwrap();
    jlox::parser::Parser { tokens: &tokens }.parse()
}

fn source_and_print(source: &str, print: &str) {
    let ast = parse(source).unwrap();
    let mut printer = jlox::ast_printer::AstPrinter {};
    assert_eq!(printer.print(&ast), print);
}

#[test]
fn simple_expression() {
    source_and_print("4 + true", "(+ 4 true)");
}

#[test]
fn compound_expression() {
    source_and_print(
        "(4 + 2) / /* comment */ (10.5 * 0)",
        "(/ (group (+ 4 2)) (group (* 10.5 0)))",
    );
}

#[test]
fn endless_group() {
    let error = parse("6 + (!true * ").unwrap_err();
    assert_eq!(
        error.message,
        "Unexpected end of file. Expected one of Number, String, True, False, Nil, or (Expr)"
    );
}

#[test]
fn incomplete_binary() {
    let error = parse("(6 + )").unwrap_err();
    assert_eq!(
        error.message,
        "Unexpected token ')'. Expected one of Number, String, True, False, Nil, or (Expr)"
    );
}

#[test]
fn unexpected_identifier() {
    let error = parse("(5 + 4 q)").unwrap_err();
    assert_eq!(error.message, "Unexpected token 'q'. Unterminated (Expr)");
}
