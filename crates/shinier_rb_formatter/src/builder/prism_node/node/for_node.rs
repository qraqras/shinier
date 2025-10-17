use crate::buildable::Buildable;
use crate::doc::{Doc, group, indent, line, sequence, space, text};
use crate::keyword::{DO, END, FOR, IN};
use ruby_prism::ForNode;

pub fn build_node(node: Option<&ForNode>) -> Doc {
    let node = node.unwrap();
    let index = node.index();
    let collection = node.collection();
    let statements = node.statements();
    group(&[
        text(FOR),
        space(),
        index.build(),
        space(),
        text(IN),
        space(),
        collection.build(),
        indent(&[statements.build_with(Some(sequence(&[space(), text(DO), line()])), None)]),
        line(),
        text(END),
    ])
}
