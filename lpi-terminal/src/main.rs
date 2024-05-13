//! Applesoft II BASIC terminal

fn main() {
    let input = "10 PRINT \"HELLO, WORLD!\"";
    let tokens = lpi_lexer::lex(input);
    println!("Lexer: {tokens:?}");
    let parse = lpi_parser::parse(&tokens);
    println!("Parser: {parse}");
    println!("Interpreter: {}", lpi_interpreter::interpret());
}
