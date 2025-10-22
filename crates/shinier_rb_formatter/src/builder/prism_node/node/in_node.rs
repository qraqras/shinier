use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::keyword::IN;
use ruby_prism::InNode;

pub fn build_node(node: Option<&InNode>) -> Doc {
    let node = node.unwrap();
    let pattern = node.pattern();
    let statements = node.statements();

    array(&[
        string(IN),
        space(),
        pattern.build(),
        indent(array(&[statements.build_with(Some(hardline()), None)])),
    ])
}
