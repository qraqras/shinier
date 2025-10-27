use crate::builder::builder::{array, group, indent, line, string};
use crate::document::Document;
use crate::keyword::{BRACES, COMMA};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::HashNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&HashNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let elements = node.elements();
    group(array(&[
        string(BRACES.0),
        indent(array(&[
            line(),
            elements.build(&array(&[string(COMMA), line()]), comments),
        ])),
        line(),
        string(BRACES.1),
    ]))
}
