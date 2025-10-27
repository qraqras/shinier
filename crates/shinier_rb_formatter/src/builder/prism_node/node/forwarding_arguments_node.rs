use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::TRIPLE_DOT;
use ruby_prism::ForwardingArgumentsNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&ForwardingArgumentsNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let _node = node.unwrap();
    string(TRIPLE_DOT)
}
