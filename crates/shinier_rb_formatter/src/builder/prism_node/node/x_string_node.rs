use crate::builder::builder::{array, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::helper::escape::escape;
use crate::keyword::BACK_QUOTE;
use ruby_prism::XStringNode;
use std::collections::HashMap;

pub fn build_node(node: Option<&XStringNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    let node = node.unwrap();
    let unescaped = node.unescaped();
    array(&[
        string(BACK_QUOTE),
        string(escape(unescaped)),
        string(BACK_QUOTE),
    ])
}
