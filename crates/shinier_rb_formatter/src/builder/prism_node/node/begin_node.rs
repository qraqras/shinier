use crate::builder::builder::{array, group, hardline, indent, string};
use crate::document::Document;
use crate::keyword::{BEGIN, END};
use crate::{BuildContext, BuildPrismNode};
use ruby_prism::BeginNode;

pub fn build_node(node: Option<&BeginNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    let rescue_clause = node.rescue_clause();
    let else_clause = node.else_clause();
    let ensure_clause = node.ensure_clause();
    group(array(&[
        string(BEGIN),
        indent(array(&[statements.build_with(
            context,
            Some(hardline()),
            None,
        )])),
        rescue_clause.build_with(context, Some(hardline()), None),
        else_clause.build_with(context, Some(hardline()), None),
        ensure_clause.build_with(context, Some(hardline()), None),
        hardline(),
        string(END),
    ]))
}
