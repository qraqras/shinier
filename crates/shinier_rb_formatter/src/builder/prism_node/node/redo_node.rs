use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::REDO;
use ruby_prism::RedoNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(_node: Option<&RedoNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    string(REDO)
}
