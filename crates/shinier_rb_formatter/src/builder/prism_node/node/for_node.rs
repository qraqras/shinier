use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{DO, END, FOR, IN};
use ruby_prism::Comments;
use ruby_prism::ForNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ForNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let index = node.index();
    let collection = node.collection();
    let statements = node.statements();
    group(array(&[
        string(FOR),
        space(),
        index.build(comments),
        space(),
        string(IN),
        space(),
        collection.build(comments),
        indent(statements.build_with(comments, Some(array(&[space(), string(DO), line()])), None)),
        line(),
        string(END),
    ]))
}
