use crate::buildable::BuildableList;
use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::keyword::{COMMA, UNDEF};
use ruby_prism::UndefNode;

pub fn build_node(node: Option<&UndefNode>) -> Document {
    let node = node.unwrap();
    let names = node.names();
    group(array(&[
        string(UNDEF),
        space(),
        names.build(array(&[string(COMMA), line()])),
    ]))
}
