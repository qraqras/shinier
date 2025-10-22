use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::TRIPLE_DOT;
use ruby_prism::ForwardingParameterNode;

pub fn build_node(node: Option<&ForwardingParameterNode>) -> Document {
    let _node = node.unwrap();
    string(TRIPLE_DOT)
}
