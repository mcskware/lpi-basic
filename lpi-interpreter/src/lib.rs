//! Applesoft II BASIC interpreter
//!
//! The interpreter takes the output of the parser and builds commands that can be run to interpret the program.

use std::io::Write;

use lpi_parser::parse_node::ParseNode;

mod command;

/// Interpret the parsed tokens
/// # Errors
/// Returns an error if there is a problem writing to the output
pub fn interpret<T: Write>(output: &mut T, root: &ParseNode) -> Result<(), std::io::Error> {
    writeln!(output, "Interpreting...")?;
    Ok(())
}
