use crate::BuildPrismNode;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{CLASS, END, SINGLETON};
use ruby_prism::Comments;
use ruby_prism::SingletonClassNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&SingletonClassNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let expression = node.expression();
    let body = node.body();
    group(array(&[
        string(CLASS),
        space(),
        string(SINGLETON),
        space(),
        expression.build(comments),
        indent(body.build_with(comments, Some(hardline()), None)),
        line(),
        string(END),
    ]))
}
