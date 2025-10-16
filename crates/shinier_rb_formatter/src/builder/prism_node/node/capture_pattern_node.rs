use crate::buildable::Buildable;
use crate::doc::{Doc, line, sequence, space, text};
use crate::keyword::ROCKET;
use ruby_prism::CapturePatternNode;

pub fn build_node(node: Option<&CapturePatternNode>) -> Doc {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    sequence(&[
        target.as_node().build(),
        space(),
        text(ROCKET),
        line(),
        value.build(),
    ])
}
