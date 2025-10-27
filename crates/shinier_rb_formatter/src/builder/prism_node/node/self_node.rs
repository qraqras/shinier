use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::SELF;
use ruby_prism::SelfNode;
use std::collections::HashMap;

pub fn build_node(_node: Option<&SelfNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    string(SELF)
}
