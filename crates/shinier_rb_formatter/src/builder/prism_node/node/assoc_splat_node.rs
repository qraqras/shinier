use crate::builder::Buildable;
use crate::doc::{Doc, sequence, text};
use crate::keyword::SPLAT;
use ruby_prism::AssocSplatNode;

pub fn build_node(node: Option<&AssocSplatNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    sequence(&[text(SPLAT), value.build()])
}
