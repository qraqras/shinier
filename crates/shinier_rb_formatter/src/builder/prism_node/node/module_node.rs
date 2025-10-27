use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, MODULE};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::ModuleNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ModuleNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let constant_path = node.constant_path();
    let body = node.body();
    group(array(&[
        string(MODULE),
        space(),
        constant_path.build(comments),
        indent(body.build_with(comments, Some(hardline()), None)),
        line(),
        string(END),
    ]))
}
