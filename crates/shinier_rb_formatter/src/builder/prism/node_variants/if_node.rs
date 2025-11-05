use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, line, none, space, string};
use crate::builder::prism::helper::build_comments::owning_comments;
use crate::document::Document;
use crate::keyword::{END, IF};
use ruby_prism::IfNode;

impl<'sh> Build for IfNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &IfNode, context: &mut BuildContext) -> Document {
    let predicate = node.predicate();
    let statements = node.statements();
    let subsequent = node.subsequent();
    group(array(&[
        string(IF),
        space(),
        predicate.build(context),
        indent(statements.build_with(context, Some(hardline()), None)),
        subsequent.build_with(context, Some(hardline()), None),
        indent(match owning_comments(&node.as_node(), context) {
            Some(comments) => array(&[hardline(), comments]),
            None => none(),
        }),
        line(),
        string(END),
    ]))
}
