use crate::BuildPrismNode;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::HASH;
use ruby_prism::Comments;
use ruby_prism::EmbeddedVariableNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&EmbeddedVariableNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let variable = node.variable();
    variable.build_with(comments, Some(string(HASH)), None)
}
