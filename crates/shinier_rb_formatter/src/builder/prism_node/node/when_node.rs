use crate::buildable::{Buildable, BuildableList};
use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::WHEN;
use ruby_prism::WhenNode;

pub fn build_node(node: Option<&WhenNode>) -> Document {
    let node = node.unwrap();
    let conditions = node.conditions();
    let statements = node.statements();
    group(array(&[
        string(WHEN),
        space(),
        conditions.build(hardline()),
        indent(statements.build_with(Some(hardline()), None)),
    ]))
}
