use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use ruby_prism::SplatNode;

pub fn build_node(node: Option<&SplatNode>) -> Doc {
    let node = node.unwrap();
    if let Some(node) = node.expression() {
        return array(&[string("*"), node.build()]);
    }
    string("") // TODO: 要確認
}
