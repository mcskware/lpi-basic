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
        node_type: NodeType::Program,
        value: String::new(),
        children: Vec::new(),
    };

    let mut line = ParseNode {
        node_type: NodeType::Line,
        value: String::new(),
        children: Vec::new(),
    };
    // first just map each token to its type
    for token in tokens {
        if let Some(node) = parsing::parse_string(token) {
            line.children.push(node);
            continue;
        }
        if line.children.is_empty() {
            if let Some(node) = parsing::parse_line_number(token) {
                line.children.push(node);
                continue;
            }
        }
        if let Some(node) = parsing::parse_numeric(token) {
            line.children.push(node);
            continue;
        }
        if let Some(node) = parsing::parse_identifier(token) {
            line.children.push(node);
            continue;
        }
        if token == "\n" {
            // push the current node as a new line
            parse_line(&mut line);
            root.children.push(line);
            line = ParseNode {
                node_type: NodeType::Line,
                value: String::new(),
                children: Vec::new(),
            };
            continue;
        }
        // anything else is just a symbol
        let node = ParseNode {
            node_type: NodeType::Symbol,
            value: token.clone(),
            children: Vec::new(),
        };
        line.children.push(node);
    }
    if !line.children.is_empty() {
        parse_line(&mut line);
        root.children.push(line);
    }

    root
}

fn parse_line(node: &mut ParseNode) {
    // combine arithmetic expressions until there are no more to combine
    expressions::combine(node);

    // if we have a line number directly followed by an identifier, change it into a LET statement
    if node.children.len() > 1
        && node.children[0].node_type == NodeType::LineNumber
        && node.children[1].node_type == NodeType::Identifier
    {
        let let_node = ParseNode {
            node_type: NodeType::StatementName,
            value: "LET".to_owned(),
            children: Vec::new(),
        };
        node.children.insert(1, let_node);
    }
}

#[cfg(test)]
mod tests;
