use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::ROCKET;
use ruby_prism::CapturePatternNode;

pub fn build_node(node: Option<&CapturePatternNode>) -> Doc {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    group(array(&[
        target.as_node().build(),
        space(),
        string(ROCKET),
        line(),
        value.build(),
    ]))
}
