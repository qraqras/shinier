use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, UNLESS};
use ruby_prism::UnlessNode;

impl<'sh> Build for UnlessNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &UnlessNode, context: &mut BuildContext) -> Document {
    let predicate = node.predicate();
    let statements = node.statements();
    let else_clause = node.else_clause();
    group(array(&[
        string(UNLESS),
        space(),
        predicate.build(context),
        indent(statements.build_with(context, Some(hardline()), None)),
        else_clause.build_with(context, Some(hardline()), None),
        line(),
        string(END),
    ]))
}
