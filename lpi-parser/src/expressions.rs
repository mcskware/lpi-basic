use crate::{node_type::NodeType, parse_node::ParseNode};

pub fn combine(node: &mut ParseNode) {
    let mut did_something = true;
    while did_something {
        did_something = false;
        did_something |= combine_arithmetic_expressions(node, &["*", "/"]);
        did_something |= combine_arithmetic_expressions(node, &["+", "-"]);
        did_something |= combine_arithmetic_expressions(node, &["^"]);
        did_something |= combine_parens(node);
    }
}

fn combine_parens(node: &mut ParseNode) -> bool {
    // here we want to combine a left paren, a numerical, and a right paren
    let mut did_something = false;
    let mut continue_parsing = true;
    while continue_parsing {
        continue_parsing = false;
        for i in 0..node.children.len() - 1 {
            if node.children[i].node_type == NodeType::Symbol
                && node.children[i].value == "("
                && is_numerical(node.children[i + 1].node_type)
                && node.children[i + 2].node_type == NodeType::Symbol
                && node.children[i + 2].value == ")"
            {
                let left = node.children.remove(i);
                let num = node.children.remove(i);
                let right = node.children.remove(i);
                // combine the expression into a new node with these children
                let new_node = ParseNode {
                    node_type: NodeType::Expression,
                    value: String::new(),
                    children: vec![left, num, right],
                };
                // replace the combined nodes with the new one
                node.children.insert(i, new_node);
                continue_parsing = true;
                did_something = true;
                break;
            }
        }
    }
    did_something
}

#[allow(clippy::collapsible_if)]
fn combine_arithmetic_expressions(node: &mut ParseNode, operators: &[&str]) -> bool {
    // combine arithmetic expressions
    let mut did_something = false;
    let mut continue_parsing = true;
    while continue_parsing {
        continue_parsing = false;
        for i in 0..node.children.len() - 1 {
            if is_arithmetic_operator(&node.children[i], operators) {
                if i > 0 && i < node.children.len() - 1 {
                    if is_numerical(node.children[i - 1].node_type)
                        && is_numerical(node.children[i + 1].node_type)
                    {
                        let left = node.children.remove(i - 1);
                        let op = node.children.remove(i - 1);
                        let right = node.children.remove(i - 1);
                        // combine the expression into a new node with these children
                        let new_node = ParseNode {
                            node_type: NodeType::Expression,
                            value: String::new(),
                            children: vec![left, op, right],
                        };
                        // replace the combined nodes with the new one
                        node.children.insert(i - 1, new_node);
                        continue_parsing = true;
                        did_something = true;
                        break;
                    }
                }
            }
        }
    }
    did_something
}

fn is_numerical(node_type: NodeType) -> bool {
    node_type == NodeType::Number
        || node_type == NodeType::Float
        || node_type == NodeType::Identifier
        || node_type == NodeType::Expression
}

fn is_arithmetic_operator(node: &ParseNode, operators: &[&str]) -> bool {
    node.node_type == NodeType::Symbol && operators.contains(&node.value.as_str())
}
