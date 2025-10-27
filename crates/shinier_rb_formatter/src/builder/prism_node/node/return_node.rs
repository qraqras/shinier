use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, space, string};
use crate::document::Document;
use crate::keyword::RETURN;
use ruby_prism::Comments;
use ruby_prism::ReturnNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&ReturnNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[
        string(RETURN),
        space(),
        indent(arguments.build(comments)),
    ]))
}
