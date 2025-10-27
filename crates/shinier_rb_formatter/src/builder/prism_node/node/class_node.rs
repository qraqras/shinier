use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::{CLASS, END, INHERITES};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::ClassNode;
use ruby_prism::Comments;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ClassNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let constant_path = node.constant_path();
    let super_class = node.superclass();
    let body = node.body();
    group(array(&[
        string(CLASS),
        space(),
        constant_path.build(comments),
        super_class.build_with(
            comments,
            Some(array(&[space(), string(INHERITES), space()])),
            Some(space()),
        ),
        indent(body.build_with(comments, Some(hardline()), None)),
        hardline(),
        string(END),
    ]))
}
