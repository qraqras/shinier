use crate::builder::Buildable;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::SUPER;
use ruby_prism::ForwardingSuperNode;

pub fn build_node(node: Option<&ForwardingSuperNode>) -> Document {
    let node = node.unwrap();
    let block = node.block();
    group(array(&[string(SUPER), space(), block.build()]))
}
