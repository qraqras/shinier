use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::NIL;
use ruby_prism::NilNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(_node: Option<&NilNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    string(NIL)
}
