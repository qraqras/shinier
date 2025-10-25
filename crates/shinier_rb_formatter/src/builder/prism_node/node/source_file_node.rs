use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::FILE;
use ruby_prism::SourceFileNode;

pub fn build_node(_node: Option<&SourceFileNode>) -> Document {
    string(FILE)
}
