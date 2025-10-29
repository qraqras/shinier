use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::IT;
use ruby_prism::ItLocalVariableReadNode;

impl<'sh> Build for Option<&ItLocalVariableReadNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(
    _node: Option<&ItLocalVariableReadNode>,
    _context: &mut BuildContext,
) -> Document {
    string(IT)
}
