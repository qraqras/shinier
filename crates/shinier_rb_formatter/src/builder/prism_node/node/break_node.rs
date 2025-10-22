use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::BREAK;
use ruby_prism::BreakNode;

pub fn build_node(node: Option<&BreakNode>) -> Doc {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[
        string(BREAK),
        arguments.build_with(Some(line()), None),
    ]))
}
