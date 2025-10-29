use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, line, none, string};
use crate::document::Document;
use crate::keyword::COMMA;
use ruby_prism::ArgumentsNode;

impl<'sh> Build for Option<&ArgumentsNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&ArgumentsNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let arguments = node.arguments();
            let separator = array(&[string(COMMA), line()]);
            group(arguments.build(context, &separator))
        }
        None => none(),
    }
}
