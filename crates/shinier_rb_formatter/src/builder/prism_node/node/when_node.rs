use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::WHEN;
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::WhenNode;

pub fn build_node(node: Option<&WhenNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let conditions = node.conditions();
    let statements = node.statements();
    group(array(&[
        string(WHEN),
        space(),
        conditions.build(context, &hardline()),
        indent(statements.build_with(context, Some(hardline()), None)),
    ]))
}
