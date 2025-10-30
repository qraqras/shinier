use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, line, space, string};
use crate::builder::prism::helper::layout::build_rest;
use crate::document::Document;
use crate::keyword::{COMMA, WRITE_OPERATOR};
use ruby_prism::MultiWriteNode;

impl<'sh> Build for MultiWriteNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &MultiWriteNode, context: &mut BuildContext) -> Document {
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
