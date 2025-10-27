use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::LINE;
use ruby_prism::SourceLineNode;
use std::collections::HashMap;

pub fn build_node(_node: Option<&SourceLineNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    string(LINE)
}
