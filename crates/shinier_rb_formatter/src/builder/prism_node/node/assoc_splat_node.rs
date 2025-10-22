use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::SPLAT;
use ruby_prism::AssocSplatNode;

pub fn build_node(node: Option<&AssocSplatNode>) -> Document {
    let node = node.unwrap();
    let value = node.value();
    group(array(&[string(SPLAT), value.build()]))
}
