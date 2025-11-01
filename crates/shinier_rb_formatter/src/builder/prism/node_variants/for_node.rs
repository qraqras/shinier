use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::builder::prism::helper::owning_comments;
use crate::document::Document;
use crate::keyword::{DO, END, FOR, IN};
use ruby_prism::ForNode;

impl<'sh> Build for ForNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &ForNode, context: &mut BuildContext) -> Document {
    let index = node.index();
    let collection = node.collection();
    let statements = node.statements();
    group(array(&[
        string(FOR),
        space(),
        index.build(context),
        space(),
        string(IN),
        space(),
        collection.build(context),
        indent(statements.build_with(context, Some(array(&[space(), string(DO), line()])), None)),
        indent(owning_comments(&node.as_node(), context)),
        line(),
        string(END),
    ]))
}
