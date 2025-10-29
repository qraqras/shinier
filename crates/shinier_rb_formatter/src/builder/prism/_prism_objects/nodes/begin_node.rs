use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, string};
use crate::document::Document;
use crate::keyword::{BEGIN, END};
use ruby_prism::BeginNode;

impl<'sh> Build for Option<&BeginNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&BeginNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let statements = node.statements();
    let rescue_clause = node.rescue_clause();
    let else_clause = node.else_clause();
    let ensure_clause = node.ensure_clause();
    group(array(&[
        string(BEGIN),
        indent(array(&[statements.as_ref().build_with(
            context,
            Some(hardline()),
            None,
        )])),
        rescue_clause
            .as_ref()
            .build_with(context, Some(hardline()), None),
        else_clause
            .as_ref()
            .build_with(context, Some(hardline()), None),
        ensure_clause
            .as_ref()
            .build_with(context, Some(hardline()), None),
        hardline(),
        string(END),
    ]))
}
