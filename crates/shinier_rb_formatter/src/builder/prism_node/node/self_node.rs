use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::SELF;
use ruby_prism::SelfNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(_node: Option<&SelfNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    string(SELF)
}
