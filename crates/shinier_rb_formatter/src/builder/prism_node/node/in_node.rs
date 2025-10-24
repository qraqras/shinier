use crate::buildable::Buildable;
use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::IN;
use ruby_prism::InNode;

pub fn build_node(node: Option<&InNode>) -> Document {
    let node = node.unwrap();
    let pattern = node.pattern();
    let statements = node.statements();
    group(array(&[
        string(IN),
        space(),
        pattern.build(),
        indent(statements.build_with(Some(hardline()), None)),
    ]))
}
