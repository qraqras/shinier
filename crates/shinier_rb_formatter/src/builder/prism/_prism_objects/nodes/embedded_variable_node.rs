use crate::Build;
use crate::BuildContext;
use crate::builder::builder::string;
use crate::document::Document;
use crate::keyword::HASH;
use ruby_prism::EmbeddedVariableNode;

impl<'sh> Build for Option<&EmbeddedVariableNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&EmbeddedVariableNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let variable = node.variable();
    variable.build_with(context, Some(string(HASH)), None)
}
