use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, hardline, space, string};
use crate::document::Document;
use crate::keyword::{CASE, END};
use ruby_prism::CaseNode;

impl<'sh> Build for Option<&CaseNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&CaseNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let predicate = node.predicate();
    let conditions = node.conditions();
    let else_clause = node.else_clause();
    group(array(&[
        string(CASE),
        predicate.build_with(context, Some(space()), None),
        conditions.build_with(context, &hardline(), Some(hardline()), None),
        else_clause
            .as_ref()
            .build_with(context, Some(hardline()), None),
        hardline(),
        string(END),
    ]))
}
