//! An interpreter for an Applesoft II BASIC program

use std::collections::HashMap;

use lpi_parser::{node_type::NodeType, parse_node::ParseNode};

#[allow(dead_code)]
pub struct Interpreter {
    root: ParseNode,
    float_scalars: HashMap<String, f64>,
    float_arrays: HashMap<String, Vec<f64>>,
}

impl Interpreter {
    pub fn new(root: ParseNode) -> Self {
        Self {
            root,
            float_scalars: HashMap::new(),
            float_arrays: HashMap::new(),
        }
    }

    pub fn get_float_scalar(&self, name: &str) -> f64 {
        self.float_scalars.get(name).copied().unwrap_or(0.0)
    }

    pub fn set_float_scalar(&mut self, name: &str, value: f64) {
        let _unused = self.float_scalars.insert(name.to_owned(), value);
    }

    pub fn dim_float_array(&mut self, name: &str, size: usize) {
        let _unused = self.float_arrays.insert(name.to_owned(), vec![0.0; size]);
    }

    #[allow(dead_code)]
    pub fn get_float_array(&mut self, name: &str, index: usize) -> Result<f64, String> {
        if !self.float_arrays.contains_key(name) {
            self.dim_float_array(name, 10);
        }
        // Safety: we just created this
        #[allow(clippy::unwrap_used)]
        let array = self.float_arrays.get(name).unwrap();
        if index >= array.len() {
            return Err(format!("Array {name} index out of bounds: {index}"));
        }
        Ok(array[index])
    }

    pub fn set_float_array(&mut self, name: &str, index: usize, value: f64) -> Result<(), String> {
        if !self.float_arrays.contains_key(name) {
            self.dim_float_array(name, 10);
        }
        // Safety: we just created this
        #[allow(clippy::unwrap_used)]
        let array = self.float_arrays.get_mut(name).unwrap();
        if index >= array.len() {
            return Err(format!("Array {name} index out of bounds: {index}"));
        }
        array[index] = value;
        Ok(())
    }

    pub fn evaluate_expression(&self, node: &ParseNode) -> Result<f64, String> {
        match node.get_node_type() {
            // Safety: this is an invariant
            #[allow(clippy::unwrap_used)]
            NodeType::Number | NodeType::Float => Ok(node.get_value().parse().unwrap()),
            NodeType::Identifier => Ok(self.get_float_scalar(node.get_value())),
            NodeType::Expression => {
                let children = node.get_children();
                let left = self.evaluate_expression(&children[0])?;
                let right = self.evaluate_expression(&children[2])?;
                match children[1].get_value() {
                    "+" => Ok(left + right),
                    "-" => Ok(left - right),
                    "*" => Ok(left * right),
                    "/" => Ok(left / right),
                    "^" => Ok(left.powf(right)),
                    _ => Err(format!("Unexpected operator: {}", children[1].get_value())),
                }
            }
            _ => Err(format!("Unexpected node type: {:?}", node.get_node_type())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_evaluate_expression() {
        let mut interpreter = Interpreter::new(ParseNode::new(
            NodeType::Program,
            String::new(),
            vec![ParseNode::new(
                NodeType::Line,
                String::new(),
                vec![ParseNode::new(
                    NodeType::StatementName,
                    "LET".to_owned(),
                    vec![
                        ParseNode::new(NodeType::Identifier, "A".to_owned(), vec![]),
                        ParseNode::new(
                            NodeType::Expression,
                            String::new(),
                            vec![
                                ParseNode::new(NodeType::Number, "1".to_owned(), vec![]),
                                ParseNode::new(NodeType::Symbol, "+".to_owned(), vec![]),
                                ParseNode::new(NodeType::Number, "2".to_owned(), vec![]),
                            ],
                        ),
                    ],
                )],
            )],
        ));

        let variable_node_value;
        let result;
        #[allow(clippy::unwrap_used, clippy::expect_used)]
        {
            let node = interpreter.root.get_children().first().unwrap();
            let let_node = node.get_children().first().unwrap();
            let variable_node = let_node.get_children().first().unwrap();
            let expression_node = let_node.get_children().last().unwrap();
            result = interpreter
                .evaluate_expression(expression_node)
                .expect("Error evaluating expression");
            variable_node_value = variable_node.get_value().to_owned();
        }
        assert_eq!(result, 3.0);
        interpreter.set_float_scalar(&variable_node_value, result);
        assert_eq!(interpreter.get_float_scalar(&variable_node_value), 3.0);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_evaluate_expression_with_parens() {
        let mut interpreter = Interpreter::new(ParseNode::new(
            NodeType::Program,
            String::new(),
            vec![ParseNode::new(
                NodeType::Line,
                String::new(),
                vec![ParseNode::new(
                    NodeType::StatementName,
                    "LET".to_owned(),
                    vec![
                        ParseNode::new(NodeType::Identifier, "A".to_owned(), vec![]),
                        ParseNode::new(
                            NodeType::Expression,
                            String::new(),
                            vec![
                                ParseNode::new(NodeType::Number, "1".to_owned(), vec![]),
                                ParseNode::new(NodeType::Symbol, "+".to_owned(), vec![]),
                                ParseNode::new(
                                    NodeType::Expression,
                                    String::new(),
                                    vec![
                                        ParseNode::new(NodeType::Number, "2".to_owned(), vec![]),
                                        ParseNode::new(NodeType::Symbol, "*".to_owned(), vec![]),
                                        ParseNode::new(NodeType::Number, "3".to_owned(), vec![]),
                                    ],
                                ),
                            ],
                        ),
                    ],
                )],
            )],
        ));

        let variable_node_value;
        let result;
        #[allow(clippy::unwrap_used, clippy::expect_used)]
        {
            let node = interpreter.root.get_children().first().unwrap();
            let let_node = node.get_children().first().unwrap();
            let variable_node = let_node.get_children().first().unwrap();
            let expression_node = let_node.get_children().last().unwrap();
            result = interpreter
                .evaluate_expression(expression_node)
                .expect("Error evaluating expression");
            variable_node_value = variable_node.get_value().to_owned();
        }
        assert_eq!(result, 7.0);
        interpreter.set_float_scalar(&variable_node_value, result);
        assert_eq!(interpreter.get_float_scalar(&variable_node_value), 7.0);
    }
}
