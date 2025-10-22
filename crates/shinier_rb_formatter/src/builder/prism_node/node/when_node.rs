use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::WHEN;
use ruby_prism::WhenNode;

pub fn build_node(node: Option<&WhenNode>) -> Document {
    let node = node.unwrap();
    let conditions = node.conditions();
    let statements = node.statements();
    array(&[
        string(WHEN),
        conditions.build_with(hardline(), Some(space()), None),
        indent(statements.build_with(Some(hardline()), None)),
    ])
}
