use crate::ENCODING;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use ruby_prism::SourceEncodingNode;
use std::collections::HashMap;

pub fn build_node(_node: Option<&SourceEncodingNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    string(ENCODING)
}
