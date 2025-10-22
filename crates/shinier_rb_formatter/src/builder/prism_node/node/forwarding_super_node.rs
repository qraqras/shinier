use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::SUPER;
use ruby_prism::ForwardingSuperNode;

pub fn build_node(node: Option<&ForwardingSuperNode>) -> Doc {
    let node = node.unwrap();
    let block = node.block();
    group(array(&[string(SUPER), space(), block.build()]))
}
