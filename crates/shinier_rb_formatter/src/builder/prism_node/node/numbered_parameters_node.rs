use crate::BuildPrismNode;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::NumberedParametersNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&NumberedParametersNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let maximum = node.maximum();
    string(maximum.to_string())
}
