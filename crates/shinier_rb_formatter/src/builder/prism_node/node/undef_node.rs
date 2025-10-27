use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::keyword::{COMMA, UNDEF};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::Comments;
use ruby_prism::UndefNode;
use std::collections::HashMap;

pub fn build_node(
    node: Option<&UndefNode>,
    comments: &mut Comments,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let names = node.names();
    group(array(&[
        string(UNDEF),
        space(),
        names.build(&array(&[string(COMMA), line()]), comments),
    ]))
}
