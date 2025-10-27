use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::REDO;
use ruby_prism::RedoNode;
use std::collections::HashMap;

pub fn build_node(_node: Option<&RedoNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    string(REDO)
}
