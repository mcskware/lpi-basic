//! Represents a parse tree node

use crate::node_type::NodeType;

/// Parse node
#[derive(PartialEq, Clone)]
pub struct ParseNode {
    pub(crate) node_type: NodeType,
    pub(crate) value: String,
    pub(crate) children: Vec<ParseNode>,
}

impl ParseNode {
    fn display_at_depth(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
        depth: usize,
    ) -> std::fmt::Result {
        for _ in 0..depth {
            write!(fmt, "  ")?;
        }
        writeln!(fmt, "{:?} '{}'", self.node_type, self.value)?;
        for child in &self.children {
            child.display_at_depth(fmt, depth + 1)?;
        }
        Ok(())
    }

    /// Get the type of the node
    #[must_use]
    pub const fn get_node_type(&self) -> NodeType {
        self.node_type
    }

    /// Get the children of the node
    #[must_use]
    pub const fn get_children(&self) -> &Vec<Self> {
        &self.children
    }

    /// Get the value of the node
    #[must_use]
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Create a new parse node
    #[must_use]
    pub fn new(node_type: NodeType, value: String, children: Vec<Self>) -> Self {
        Self {
            node_type,
            value,
            children,
        }
    }
}

impl std::fmt::Debug for ParseNode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self}")
    }
}

impl std::fmt::Display for ParseNode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display_at_depth(fmt, 0)
    }
}
