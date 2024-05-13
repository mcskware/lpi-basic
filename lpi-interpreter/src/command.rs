//! Represents a command that can be executed by the interpreter.

pub enum StatementKind {
    Print,
}

pub struct Command {
    line_number: Option<u16>,
    kind: StatementKind,
    arguments: Vec<String>,
}

impl Command {
    pub fn new(line_number: Option<u16>, kind: StatementKind, arguments: Vec<String>) -> Self {
        Self {
            line_number,
            kind,
            arguments,
        }
    }

    pub fn execute(&self) -> String {
        match self.kind {
            StatementKind::Print => format!(
                "Line {:?}: PRINT {}",
                self.line_number,
                self.arguments.join(" ")
            ),
        }
    }
}
