//! Applesoft II BASIC interpreter
//!
//! The interpreter takes the output of the parser and builds commands that can be run to interpret the program.

mod command;

/// Interpret the parsed tokens
#[must_use]
pub fn interpret() -> String {
    let command = command::Command::new(
        Some(10),
        command::StatementKind::Print,
        vec!["Hello, world!".to_owned()],
    );
    format!("Applesoft II BASIC interpreter: {}", command.execute())
}
