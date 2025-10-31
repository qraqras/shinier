use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, hardline, space, string};
use crate::document::Document;
use crate::keyword::{CASE, END};
use ruby_prism::CaseMatchNode;

impl<'sh> Build for CaseMatchNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &CaseMatchNode, context: &mut BuildContext) -> Document {
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
