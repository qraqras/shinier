use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::IN;
use ruby_prism::InNode;

impl<'sh> Build for InNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &InNode, context: &mut BuildContext) -> Document {
    let pattern = node.pattern();
    let statements = node.statements();
    group(array(&[
        string(IN),
        space(),
        pattern.build(context),
        indent(statements.build_with(context, Some(hardline()), None)),
    ]))
}
