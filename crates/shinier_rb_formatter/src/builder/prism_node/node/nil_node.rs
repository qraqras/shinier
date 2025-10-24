use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::NIL;
use ruby_prism::NilNode;

pub fn build_node(_node: Option<&NilNode>) -> Document {
    string(NIL)
}
