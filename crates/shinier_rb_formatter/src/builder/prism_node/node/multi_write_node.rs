use crate::builder::Buildable;
use crate::doc::*;
use crate::helper::build_rest::build_rest;
use crate::keyword::{COMMA, WRITE_OPERATOR};
use ruby_prism::MultiWriteNode;

pub fn build_node(node: Option<&MultiWriteNode>) -> Doc {
    let node = node.unwrap();
    let lefts = node.lefts();
    let rest = node.rest();
    let rights = node.rights();
    let value = node.value();

    let separator = sequence(&[text(COMMA), line()]);
    sequence(&[
        build_rest(lefts, rest, rights, &separator),
        space(),
        text(WRITE_OPERATOR),
        line(),
        value.build(),
    ])
}
