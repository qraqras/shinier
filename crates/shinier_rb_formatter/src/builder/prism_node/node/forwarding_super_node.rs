use crate::builder::Buildable;
use crate::doc::*;
use crate::keyword::SUPER;
use ruby_prism::ForwardingSuperNode;

pub fn build_node(node: Option<&ForwardingSuperNode>) -> Doc {
    let node = node.unwrap();
    let block = node.block();
    group(&[text(SUPER), space(), block.build()])
}
