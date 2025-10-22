use crate::builder::Buildable;
use crate::builder::builder::*;
use crate::document::*;
use crate::helper::build_rest::build_rest;
use crate::keyword::{COMMA, WRITE_OPERATOR};
use ruby_prism::MultiWriteNode;

pub fn build_node(node: Option<&MultiWriteNode>) -> Doc {
    let node = node.unwrap();
    let lefts = node.lefts();
    let rest = node.rest();
    let rights = node.rights();
    let value = node.value();

    let separator = array(&[string(COMMA), line()]);
    array(&[
        build_rest(lefts, rest, rights, &separator),
        space(),
        string(WRITE_OPERATOR),
        line(),
        value.build(),
    ])
}
