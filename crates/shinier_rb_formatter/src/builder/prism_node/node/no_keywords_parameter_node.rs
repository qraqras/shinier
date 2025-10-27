use crate::builder::builder::{array, string};
use crate::document::Document;
use ruby_prism::Comments;
use crate::keyword::{NIL, SPLAT};
use ruby_prism::NoKeywordsParameterNode;
use std::collections::HashMap;

pub fn build_node(_node: Option<&NoKeywordsParameterNode>, comments: &mut Comments, option: Option<&HashMap<&str, bool>>) -> Document {
    array(&[string(SPLAT), string(NIL)])
}
