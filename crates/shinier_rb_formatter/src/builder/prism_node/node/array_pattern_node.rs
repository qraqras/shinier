use crate::builder::{Buildable, BuildableList};
use crate::doc::{Doc, indent, line, sequence, softline, text};
use crate::helper::separate_docs::separate_docs;
use crate::keyword::{BRACKETS, COMMA};
use ruby_prism::ArrayPatternNode;

pub fn build_node(node: Option<&ArrayPatternNode>) -> Doc {
    let node = node.unwrap();
    let constant = node.constant();
    let requireds = node.requireds();
    let rest = node.rest();
    let posts = node.posts();

    let separator = sequence(&[text(COMMA), line()]);

    let separated_requireds = requireds.build(separator.clone(), sequence);
    let separated_posts = posts.build(separator.clone(), sequence);

    sequence(&[
        constant.build(),
        text(BRACKETS.0),
        softline(),
        indent(&separate_docs(
            &[separated_requireds, rest.build(), separated_posts],
            separator.clone(),
        )),
        text(BRACKETS.1),
    ])
}
