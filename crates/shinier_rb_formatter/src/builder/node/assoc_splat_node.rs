use crate::builder::build_optional;
use crate::doc::{Doc, sequence, text};
use crate::keyword::SPLAT;
use ruby_prism::AssocSplatNode;

pub fn build_node(node: Option<&AssocSplatNode>) -> Doc {
    let node = node.unwrap();
    let value = node.value();
    sequence(&[text(SPLAT), build_optional(value.as_ref())])
}
