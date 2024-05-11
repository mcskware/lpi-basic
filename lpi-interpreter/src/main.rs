//! Applesoft II BASIC interpreter

fn main() {
    println!("Hello, world!");
    println!("Lexer: {}", lpi_lexer::lex());
    println!("Parser: {}", lpi_parser::parse());
    println!("Interpreter: {}", lpi_interpreter::interpret());
}
