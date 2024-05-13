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
