//! Node type

/// Node type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NodeType {
    /// Root node
    Program,
    /// Line
    Line,
    /// Line Number node
    LineNumber,
    /// Statement name node
    StatementName,
    /// String node
    String,
    /// Number node
    Number,
    /// Float node
    Float,
    /// Identifier node
    Identifier,
    /// Symbol node
    Symbol,
    /// Expression node
    Expression,
}
