use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, space, string};
use crate::document::Document;
use crate::keyword::RETURN;
use ruby_prism::ReturnNode;

impl<'sh> Build for Option<&ReturnNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ReturnNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[
        string(RETURN),
        space(),
        indent(arguments.as_ref().build(context)),
    ]))
}
