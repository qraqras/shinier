use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::helper::build_rest::build_rest;
use crate::keyword::{COMMA, WRITE_OPERATOR};
use ruby_prism::MultiWriteNode;

pub fn build_node(node: Option<&MultiWriteNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let lefts = node.lefts();
    let rest = node.rest();
    let rights = node.rights();
    let value = node.value();

    let separator = array(&[string(COMMA), line()]);
    group(array(&[
        build_rest(lefts, rest, rights, &separator, context),
        space(),
        string(WRITE_OPERATOR),
        line(),
        value.build(context),
    ]))
}
