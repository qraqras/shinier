use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::LINE;
use ruby_prism::SourceLineNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(_node: Option<&SourceLineNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    string(LINE)
}
