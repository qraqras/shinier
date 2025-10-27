use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, softline, string};
use crate::document::Document;
use crate::keyword::{BRACES, HASH};
use ruby_prism::Comments;
use ruby_prism::EmbeddedStatementsNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&EmbeddedStatementsNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    group(array(&[
        string(HASH),
        string(BRACES.0),
        indent(array(&[softline(), statements.build(comments)])),
        softline(),
        string(BRACES.1),
    ]))
}
