//! Applesoft II BASIC parser
//!
//! The parser takes the output of the lexer and builds a parse tree.

use node_type::NodeType;
use parse_node::ParseNode;

mod node_type;
mod parse_node;
mod parsing;

const KEYWORDS: [&str; 1] = ["PRINT"];

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
        if token.chars().all(char::is_numeric) && root.children.is_empty() {
            let node = ParseNode {
                node_type: NodeType::LineNumber,
                value: token.clone(),
                children: Vec::new(),
            };

            root.children.push(node);
            continue;
        }
        if let Some(node) = parsing::parse_numeric(token) {
            root.children.push(node);
            continue;
        }
        if token.chars().any(char::is_alphabetic) {
            let node_type = if KEYWORDS.contains(&token.as_str()) {
                NodeType::StatementName
            } else {
                NodeType::Identifier
            };
            let node = ParseNode {
                node_type,
                value: token.clone(),
                children: Vec::new(),
            };

            root.children.push(node);
            continue;
        }
        let node = ParseNode {
            node_type: NodeType::Symbol,
            value: token.clone(),
            children: Vec::new(),
        };
        root.children.push(node);
    }

    // combine arithmetic expressions until there are no more to combine
    combine_expressions(&mut root);

    root
}

fn combine_expressions(node: &mut ParseNode) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = vec![
            "10".to_owned(),
            "PRINT".to_owned(),
            "\"HELLO, WORLD!\"".to_owned(),
        ];
        let parse = parse(&tokens);
        let expected = ParseNode {
            node_type: NodeType::Root,
            value: String::new(),
            children: vec![
                ParseNode {
                    node_type: NodeType::LineNumber,
                    value: "10".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::StatementName,
                    value: "PRINT".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::String,
                    value: "\"HELLO, WORLD!\"".to_owned(),
                    children: Vec::new(),
                },
            ],
        };

        assert_eq!(parse, expected);
    }

    #[test]
    fn test_float_parse() {
        let tokens = vec!["10".to_owned(), "PRINT".to_owned(), "3.14".to_owned()];
        let parse = parse(&tokens);
        let expected = ParseNode {
            node_type: NodeType::Root,
            value: String::new(),
            children: vec![
                ParseNode {
                    node_type: NodeType::LineNumber,
                    value: "10".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::StatementName,
                    value: "PRINT".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::Float,
                    value: "3.14".to_owned(),
                    children: Vec::new(),
                },
            ],
        };

        assert_eq!(parse, expected);
    }

    #[test]
    fn test_expression_parse() {
        let tokens = vec![
            "10".to_owned(),
            "PRINT".to_owned(),
            "3".to_owned(),
            "*".to_owned(),
            "4".to_owned(),
            "+".to_owned(),
            "5".to_owned(),
        ];
        let parse = parse(&tokens);
        let expected = ParseNode {
            node_type: NodeType::Root,
            value: String::new(),
            children: vec![
                ParseNode {
                    node_type: NodeType::LineNumber,
                    value: "10".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::StatementName,
                    value: "PRINT".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::Expression,
                    value: String::new(),
                    children: vec![
                        ParseNode {
                            node_type: NodeType::Expression,
                            value: String::new(),
                            children: vec![
                                ParseNode {
                                    node_type: NodeType::Number,
                                    value: "3".to_owned(),
                                    children: Vec::new(),
                                },
                                ParseNode {
                                    node_type: NodeType::Symbol,
                                    value: "*".to_owned(),
                                    children: Vec::new(),
                                },
                                ParseNode {
                                    node_type: NodeType::Number,
                                    value: "4".to_owned(),
                                    children: Vec::new(),
                                },
                            ],
                        },
                        ParseNode {
                            node_type: NodeType::Symbol,
                            value: "+".to_owned(),
                            children: Vec::new(),
                        },
                        ParseNode {
                            node_type: NodeType::Number,
                            value: "5".to_owned(),
                            children: Vec::new(),
                        },
                    ],
                },
            ],
        };

        assert_eq!(parse, expected);
    }

    #[test]
    fn test_operator_precedence() {
        let tokens = vec![
            "10".to_owned(),
            "PRINT".to_owned(),
            "3".to_owned(),
            "+".to_owned(),
            "4".to_owned(),
            "*".to_owned(),
            "5".to_owned(),
        ];
        let parse = parse(&tokens);
        let expected = ParseNode {
            node_type: NodeType::Root,
            value: String::new(),
            children: vec![
                ParseNode {
                    node_type: NodeType::LineNumber,
                    value: "10".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::StatementName,
                    value: "PRINT".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::Expression,
                    value: String::new(),
                    children: vec![
                        ParseNode {
                            node_type: NodeType::Number,
                            value: "3".to_owned(),
                            children: Vec::new(),
                        },
                        ParseNode {
                            node_type: NodeType::Symbol,
                            value: "+".to_owned(),
                            children: Vec::new(),
                        },
                        ParseNode {
                            node_type: NodeType::Expression,
                            value: String::new(),
                            children: vec![
                                ParseNode {
                                    node_type: NodeType::Number,
                                    value: "4".to_owned(),
                                    children: Vec::new(),
                                },
                                ParseNode {
                                    node_type: NodeType::Symbol,
                                    value: "*".to_owned(),
                                    children: Vec::new(),
                                },
                                ParseNode {
                                    node_type: NodeType::Number,
                                    value: "5".to_owned(),
                                    children: Vec::new(),
                                },
                            ],
                        },
                    ],
                },
            ],
        };

        assert_eq!(parse, expected);
    }

    #[test]
    fn test_parens() {
        let tokens = vec![
            "10".to_owned(),
            "PRINT".to_owned(),
            "(".to_owned(),
            "3".to_owned(),
            "+".to_owned(),
            "4".to_owned(),
            ")".to_owned(),
            "*".to_owned(),
            "5".to_owned(),
        ];
        let parse = parse(&tokens);
        let expected = ParseNode {
            node_type: NodeType::Root,
            value: String::new(),
            children: vec![
                ParseNode {
                    node_type: NodeType::LineNumber,
                    value: "10".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::StatementName,
                    value: "PRINT".to_owned(),
                    children: Vec::new(),
                },
                ParseNode {
                    node_type: NodeType::Expression,
                    value: String::new(),
                    children: vec![
                        ParseNode {
                            node_type: NodeType::Expression,
                            value: String::new(),
                            children: vec![
                                ParseNode {
                                    node_type: NodeType::Symbol,
                                    value: "(".to_owned(),
                                    children: Vec::new(),
                                },
                                ParseNode {
                                    node_type: NodeType::Expression,
                                    value: String::new(),
                                    children: vec![
                                        ParseNode {
                                            node_type: NodeType::Number,
                                            value: "3".to_owned(),
                                            children: Vec::new(),
                                        },
                                        ParseNode {
                                            node_type: NodeType::Symbol,
                                            value: "+".to_owned(),
                                            children: Vec::new(),
                                        },
                                        ParseNode {
                                            node_type: NodeType::Number,
                                            value: "4".to_owned(),
                                            children: Vec::new(),
                                        },
                                    ],
                                },
                                ParseNode {
                                    node_type: NodeType::Symbol,
                                    value: ")".to_owned(),
                                    children: Vec::new(),
                                },
                            ],
                        },
                        ParseNode {
                            node_type: NodeType::Symbol,
                            value: "*".to_owned(),
                            children: Vec::new(),
                        },
                        ParseNode {
                            node_type: NodeType::Number,
                            value: "5".to_owned(),
                            children: Vec::new(),
                        },
                    ],
                },
            ],
        };

        assert_eq!(parse, expected);
    }
}
