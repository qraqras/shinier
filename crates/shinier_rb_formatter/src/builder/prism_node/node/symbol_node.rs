use crate::builder::builder::{array, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::helper::escape::escape;
use crate::keyword::COLON;
use ruby_prism::SymbolNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&SymbolNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    array(&[string(COLON), string(escape(unescaped))])
}
