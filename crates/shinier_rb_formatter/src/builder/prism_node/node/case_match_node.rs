use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, space, string};
use crate::document::Document;
use crate::keyword::{CASE, END};
use crate::{BuildPrismNode, BuildPrismNodeList};
use ruby_prism::CaseMatchNode;

pub fn build_node(node: Option<&CaseMatchNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let predicate = node.predicate();
    let conditions = node.conditions();
    let else_clause = node.else_clause();
    group(array(&[
        string(CASE),
        predicate.build_with(context, Some(space()), None),
        conditions.build_with(context, &hardline(), Some(hardline()), None),
        else_clause.build_with(context, Some(hardline()), None),
        hardline(),
        string(END),
    ]))
}
