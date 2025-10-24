use crate::builder::builder::{array, string};
use crate::document::Document;
use crate::keyword::{NIL, SPLAT};
use ruby_prism::NoKeywordsParameterNode;

pub fn build_node(_node: Option<&NoKeywordsParameterNode>) -> Document {
    array(&[string(SPLAT), string(NIL)])
}
