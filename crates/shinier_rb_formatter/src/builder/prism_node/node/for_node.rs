use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{DO, END, FOR, IN};
use ruby_prism::ForNode;

pub fn build_node(node: Option<&ForNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
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
        line(),
        string(END),
    ]))
}
