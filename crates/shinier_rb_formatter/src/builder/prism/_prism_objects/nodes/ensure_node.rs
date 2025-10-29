use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, none, string};
use crate::document::Document;
use crate::keyword::ENSURE;
use ruby_prism::EnsureNode;

impl<'sh> Build for Option<&EnsureNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&EnsureNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let statements = node.statements();
            group(array(&[
                string(ENSURE),
                indent(array(&[statements.as_ref().build_with(
                    context,
                    Some(hardline()),
                    None,
                )])),
            ]))
        }
        None => none(),
    }
}
