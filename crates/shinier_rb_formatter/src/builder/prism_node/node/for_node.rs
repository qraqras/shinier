use crate::buildable::Buildable;
use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{DO, END, FOR, IN};
use ruby_prism::ForNode;

pub fn build_node(node: Option<&ForNode>) -> Document {
    let node = node.unwrap();
    let index = node.index();
    let collection = node.collection();
    let statements = node.statements();
    group(array(&[
        string(FOR),
        space(),
        index.build(),
        space(),
        string(IN),
        space(),
        collection.build(),
        indent(array(&[statements.build_with(
            Some(array(&[space(), string(DO), line()])),
            None,
        )])),
        line(),
        string(END),
    ]))
}
