use crate::builder::Buildable;
use crate::doc::{Doc, group, text};
use crate::keyword::SPLAT;
use ruby_prism::AssocSplatNode;

pub fn build_node(node: Option<&AssocSplatNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    group(&[text(SPLAT), value.build()])
}
