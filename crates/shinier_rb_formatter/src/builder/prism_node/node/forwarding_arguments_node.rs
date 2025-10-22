use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::TRIPLE_DOT;
use ruby_prism::ForwardingArgumentsNode;

pub fn build_node(node: Option<&ForwardingArgumentsNode>) -> Document {
    let _node = node.unwrap();
    string(TRIPLE_DOT)
}
