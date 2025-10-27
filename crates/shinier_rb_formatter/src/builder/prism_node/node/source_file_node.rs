use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::FILE;
use ruby_prism::SourceFileNode;
use std::collections::HashMap;

pub fn build_node(_node: Option<&SourceFileNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    string(FILE)
}
