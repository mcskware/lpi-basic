//! Represents a command that can be executed by the interpreter.

use lpi_parser::{node_type::NodeType, parse_node::ParseNode};

pub struct Command<'this, T: std::io::Write> {
    node: &'this ParseNode,
    output: &'this mut T,
}

impl<'this, T: std::io::Write> Command<'this, T> {
    pub fn new(node: &'this ParseNode, output: &'this mut T) -> Self {
        Self { node, output }
    }

    pub fn execute(&self) -> Result<(), String> {
        Ok(())
    }
}
