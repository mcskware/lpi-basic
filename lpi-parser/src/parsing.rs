use crate::{node_type::NodeType, parse_node::ParseNode};

const KEYWORDS: &[&str] = &["PRINT", "LET"];

pub fn parse_string(token: &str) -> Option<ParseNode> {
    if token.starts_with('"') && token.ends_with('"') {
        let node = ParseNode {
            node_type: NodeType::String,
            value: token.to_owned(),
            children: Vec::new(),
        };

        return Some(node);
    }
    None
}

pub fn parse_numeric(token: &str) -> Option<ParseNode> {
    if token.chars().any(char::is_numeric) {
        let node_type = if token.contains('.') {
            NodeType::Float
        } else {
            NodeType::Number
        };
        let node = ParseNode {
            node_type,
            value: token.to_owned(),
            children: Vec::new(),
        };
        return Some(node);
    }
    None
}

pub fn parse_identifier(token: &str) -> Option<ParseNode> {
    if token.chars().any(char::is_alphabetic) {
        let node_type = if KEYWORDS.contains(&token) {
            NodeType::StatementName
        } else {
            NodeType::Identifier
        };
        let node = ParseNode {
            node_type,
            value: token.to_owned(),
            children: Vec::new(),
        };
        return Some(node);
    }
    None
}

pub fn parse_line_number(token: &str) -> Option<ParseNode> {
    if token.chars().all(char::is_numeric) {
        let node = ParseNode {
            node_type: NodeType::LineNumber,
            value: token.to_owned(),
            children: Vec::new(),
        };
        return Some(node);
    }
    None
}
