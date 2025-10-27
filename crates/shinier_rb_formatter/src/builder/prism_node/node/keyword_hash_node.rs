use crate::BuildPrismNodeList;
use crate::builder::builder::{array, line, string};
use crate::document::Document;
use crate::keyword::COMMA;
use ruby_prism::Comments;
use ruby_prism::KeywordHashNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&KeywordHashNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let elements = node.elements();
    elements.build(&array(&[string(COMMA), line()]), comments)
}
