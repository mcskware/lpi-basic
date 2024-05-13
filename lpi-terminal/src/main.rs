//! Applesoft II BASIC terminal

fn main() {
    let input = include_str!("../programs/hello_world.bas");
    let tokens = lpi_lexer::lex(input);
    println!("Lexer: {tokens:?}");
    let parse = lpi_parser::parse(&tokens);
    println!("Parser: {parse}");
    let _unused = lpi_interpreter::interpret(&mut std::io::stdout(), &parse);
}
