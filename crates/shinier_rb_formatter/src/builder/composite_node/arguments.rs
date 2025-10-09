use crate::builder::node::{arguments_node, block_argument_node};
use crate::doc::{Doc, none, sequence};
use ruby_prism::{ArgumentsNode, BlockArgumentNode};

const OPEN_DELIMITER: &str = "(";
const CLOSE_DELIMITER: &str = ")";
const ARGUMENTS_SEPARATOR: &str = ",";

pub fn build_composite_node(
    arguments: Option<&ArgumentsNode>,
    block_argument: Option<&BlockArgumentNode>,
) -> Doc {
    let arguments = arguments_node::build_node(arguments);
    let block_argument_node = block_argument_node::build_node(block_argument);
    sequence(&[arguments, block_argument_node])
}
