use crate::builder::build_optional;
use crate::doc::{Doc, indent, sequence, softline, text};
use crate::keyword::{BRACKETS, COMMA};
use crate::layout::{separate_docs, separate_nodelist};
use ruby_prism::ArrayPatternNode;

pub fn build_node(node: Option<&ArrayPatternNode>) -> Doc {
    let node = node.unwrap();
    let constant = node.constant();
    let requireds = node.requireds();
    let rest = node.rest();
    let posts = node.posts();

    let separated_requireds = separate_nodelist(&requireds, COMMA);
    let separated_posts = separate_nodelist(&posts, COMMA);

    sequence(&[
        build_optional(constant.as_ref()),
        text(BRACKETS.0),
        softline(),
        indent(&separate_docs(&[
            sequence(&separated_requireds),
            build_optional(rest.as_ref()),
            sequence(&separated_posts),
        ])),
        text(BRACKETS.1),
    ])
}
