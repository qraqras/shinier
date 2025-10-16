use crate::buildable::{Buildable, BuildableList};
use crate::doc::{Doc, hardline, indent, sequence, space, text};
use crate::keyword::WHEN;
use ruby_prism::WhenNode;

pub fn build_node(node: Option<&WhenNode>) -> Doc {
    let node = node.unwrap();
    let conditions = node.conditions();
    let statements = node.statements();
    sequence(&[
        text(WHEN),
        conditions.build_with(hardline(), sequence, Some(space()), None),
        indent(&[statements.build_with(Some(hardline()), None)]),
    ])
}
