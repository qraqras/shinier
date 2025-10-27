use crate::ENCODING;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::SourceEncodingNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(_node: Option<&SourceEncodingNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    string(ENCODING)
}
