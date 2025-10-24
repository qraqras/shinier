use crate::buildable::Buildable;
use crate::builder::builder::group;
use crate::document::Document;
use ruby_prism::MatchWriteNode;

pub fn build_node(node: Option<&MatchWriteNode>) -> Document {
    let node = node.unwrap();
    let call = node.call();
    group(call.as_node().build())
}
