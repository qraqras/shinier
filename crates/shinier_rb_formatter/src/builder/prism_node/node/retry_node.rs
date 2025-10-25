use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::RETRY;
use ruby_prism::RetryNode;

pub fn build_node(_node: Option<&RetryNode>) -> Document {
    string(RETRY)
}
