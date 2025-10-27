use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::IT;
use ruby_prism::ItLocalVariableReadNode;
use std::collections::HashMap;
use std::iter::Peekable;

pub fn build_node(_node: Option<&ItLocalVariableReadNode>, comments: &mut Peekable<Comments>, option: Option<&HashMap<&str, bool>>) -> Document {
    string(IT)
}
