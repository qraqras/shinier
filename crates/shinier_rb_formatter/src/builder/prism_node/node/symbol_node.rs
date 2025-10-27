use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::helper::escape::escape;
use crate::keyword::COLON;
use ruby_prism::Comments;
use ruby_prism::SymbolNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(
    node: Option<&SymbolNode>,
    comments: &mut Peekable<Comments>,
    option: Option<&HashMap<&str, bool>>,
) -> Document {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    array(&[string(COLON), string(escape(unescaped))])
}
