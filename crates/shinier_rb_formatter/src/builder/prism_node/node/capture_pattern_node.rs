use crate::buildable::Buildable;
use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::keyword::ROCKET;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::CapturePatternNode;
use ruby_prism::Comments;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&CapturePatternNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    group(array(&[
        target.as_node().build(comments),
        space(),
        string(ROCKET),
        line(),
        value.build(comments),
    ]))
}
