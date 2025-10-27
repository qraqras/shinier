use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::RETRY;
use ruby_prism::RetryNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(_node: Option<&RetryNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    string(RETRY)
}
