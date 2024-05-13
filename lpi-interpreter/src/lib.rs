//! Applesoft II BASIC interpreter
//!
//! The interpreter takes the output of the parser and builds commands that can be run to interpret the program.

use lpi_parser::parse_node::ParseNode;
use r#let::let_statement;
use std::{collections::HashMap, io::Write};

mod interpreter;
mod r#let;

/// Interpret the parsed tokens
/// # Errors
/// Returns an error if there is a problem writing to the output
pub fn interpret<T: Write>(
    output: &mut T,
    root: &ParseNode,
) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(output, "Interpreting...")?;

    let mut interpreter = interpreter::Interpreter::new(root.clone());

    let mut lines: HashMap<u16, &ParseNode> = HashMap::new();
    for line in root.get_children() {
        if let Some(line_number) = line.get_children().first() {
            if let Ok(line_number) = line_number.get_value().parse::<u16>() {
                let _unused = lines.insert(line_number, line);
            } else {
                writeln!(output, "Error: bad Line number not found: {line_number}")?;
            }
        }
    }

    for line in root.get_children() {
        writeln!(output, "Line: {line}")?;
        let_statement(&mut interpreter, line)?;
    }
    Ok(())
}
