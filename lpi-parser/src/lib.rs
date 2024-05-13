//! Applesoft II BASIC parser
//!
//! The parser takes the output of the lexer and builds a parse tree.

use node_type::NodeType;
use parse_node::ParseNode;

mod expressions;
mod node_type;
mod parse_node;
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

    root
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
