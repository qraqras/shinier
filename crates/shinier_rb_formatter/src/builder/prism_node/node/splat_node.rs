use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use ruby_prism::SplatNode;

pub fn build_node(node: Option<&SplatNode>) -> Document {
    let node = node.unwrap();
    if let Some(node) = node.expression() {
        return array(&[string("*"), node.build()]);
    }
    string("") // TODO: 要確認
}
