use crate::builder::builder::{array, group, indent, line, softline, string};
use crate::builder::{Buildable, BuildableList};
use crate::document::Document;
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{BRACKETS, COMMA};
use ruby_prism::ArrayPatternNode;

pub fn build_node(node: Option<&ArrayPatternNode>) -> Document {
    let node = node.unwrap();
    let constant = node.constant();
    let requireds = node.requireds();
    let rest = node.rest();
    let posts = node.posts();

    let separator = array(&[string(COMMA), line()]);

    let separated_requireds = requireds.build(separator.clone());
    let separated_posts = posts.build(separator.clone());

    group(array(&[
        constant.build(),
        string(BRACKETS.0),
        softline(),
        indent(array(&[group(array(&separate_docs(
            &[separated_requireds, rest.build(), separated_posts],
            separator.clone(),
        )))])),
        softline(),
        string(BRACKETS.1),
    ]))
}
