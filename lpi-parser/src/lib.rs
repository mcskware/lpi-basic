//! Applesoft II BASIC parser
//!
//! The parser takes the output of the lexer and builds a parse tree.

use node_type::NodeType;
use parse_node::ParseNode;

mod expressions;
pub mod node_type;
pub mod parse_node;
mod parsing;

/// Stub function to return a string
#[must_use]
pub fn parse(tokens: &[String]) -> ParseNode {
    // we want to build a parse tree
    let mut root = ParseNode {
        node_type: NodeType::Root,
        value: String::new(),
        children: Vec::new(),
    };

    // first just map each token to its type
    for token in tokens {
        if let Some(node) = parsing::parse_string(token) {
            root.children.push(node);
            continue;
        }
        if root.children.is_empty() {
            if let Some(node) = parsing::parse_line_number(token) {
                root.children.push(node);
                continue;
            }
        }
        if let Some(node) = parsing::parse_numeric(token) {
            root.children.push(node);
            continue;
        }
        if let Some(node) = parsing::parse_identifier(token) {
            root.children.push(node);
            continue;
        }
        // anything else is just a symbol
        let node = ParseNode {
            node_type: NodeType::Symbol,
            value: token.clone(),
            children: Vec::new(),
        };
        root.children.push(node);
    }

    // combine arithmetic expressions until there are no more to combine
    expressions::combine(&mut root);

    // if we have a line number directly followed by an identifier, change it into a LET statement
    if root.children.len() > 1
        && root.children[0].node_type == NodeType::LineNumber
        && root.children[1].node_type == NodeType::Identifier
    {
        let let_node = ParseNode {
            node_type: NodeType::StatementName,
            value: "LET".to_owned(),
            children: Vec::new(),
        };
        root.children.insert(1, let_node);
    }

    root
}

#[cfg(test)]
mod tests;
