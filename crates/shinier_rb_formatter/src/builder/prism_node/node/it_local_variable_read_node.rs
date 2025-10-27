use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::IT;
use ruby_prism::ItLocalVariableReadNode;

pub fn build_node(
    _node: Option<&ItLocalVariableReadNode>,
    _context: &mut BuildContext,
) -> Document {
    string(IT)
}
