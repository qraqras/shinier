use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::IT;
use ruby_prism::ItLocalVariableReadNode;

pub fn build_node(_node: Option<&ItLocalVariableReadNode>) -> Document {
    string(IT)
}
