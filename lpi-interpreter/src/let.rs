//! LET statement logic

use crate::interpreter::Interpreter;
use lpi_parser::{node_type::NodeType, parse_node::ParseNode};

#[allow(clippy::module_name_repetitions)]
pub fn let_statement(interpreter: &mut Interpreter, node: &ParseNode) -> Result<(), String> {
    // here, we do not have a line number node, so the first node will be the LET node,
    // then we have an optional node which is an expression in parens (the index into a dim'd variable array)
    // then we have the equal sign, then the value to assign as an expression

    let mut children = node.get_children().iter();
    let _let_node = children
        .next()
        .ok_or("error: LET statement missing 'LET'")?;
    let variable_node = children
        .next()
        .ok_or("error: LET statement missing variable")?;
    let next_node = children
        .next()
        .ok_or("error: Not enough nodes in LET statement")?;
    if next_node.get_node_type() == NodeType::Expression {
        // we have an array index
        let index = interpreter.evaluate_expression(next_node)?;
        let value_node = children
            .next()
            .ok_or("error: LET statement missing value")?;
        let value = interpreter.evaluate_expression(value_node)?;
        if index.fract() != 0.0 || index < 0.0 {
            return Err("error: LET statement index must be a non-negative integer".to_owned());
        }
        // safe because of the check above
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let index = index as usize;
        interpreter.set_float_array(variable_node.get_value(), index, value)?;
    } else if next_node.get_node_type() == NodeType::Symbol && next_node.get_value() == "=" {
        let value_node = next_node;
        let value = interpreter.evaluate_expression(value_node)?;
        interpreter.set_float_scalar(variable_node.get_value(), value);
    } else {
        return Err("error: LET statement missing '='".to_owned());
    }
    Ok(())
}
