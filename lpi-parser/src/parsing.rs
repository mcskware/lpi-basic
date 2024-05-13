use crate::{node_type::NodeType, parse_node::ParseNode};

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
