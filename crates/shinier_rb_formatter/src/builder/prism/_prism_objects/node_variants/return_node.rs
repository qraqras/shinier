use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, space, string};
use crate::document::Document;
use crate::keyword::RETURN;
use ruby_prism::ReturnNode;

impl<'sh> Build for ReturnNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ReturnNode, context: &mut BuildContext) -> Document {
    let arguments = node.arguments();
    group(array(&[
        string(RETURN),
        space(),
        indent(arguments.build(context)),
    ]))
}
