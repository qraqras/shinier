use crate::BuildPrismNode;
use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::{DEFINED, PARENTHESES};
use ruby_prism::Comments;
use ruby_prism::DefinedNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&DefinedNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let value = node.value();
    group(array(&[
        string(DEFINED),
        string(PARENTHESES.0),
        value.build(comments),
        string(PARENTHESES.1),
    ]))
}
