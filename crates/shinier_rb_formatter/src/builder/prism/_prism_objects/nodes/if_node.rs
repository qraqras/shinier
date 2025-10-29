use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, line, space, string};
use crate::document::Document;
use crate::keyword::{END, IF};
use ruby_prism::IfNode;

impl<'sh> Build for Option<&IfNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&IfNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let predicate = node.predicate();
    let statements = node.statements();
    let subsequent = node.subsequent();
    group(array(&[
        string(IF),
        space(),
        predicate.build(context),
        indent(
            statements
                .as_ref()
                .build_with(context, Some(hardline()), None),
        ),
        subsequent.build_with(context, Some(hardline()), None),
        line(),
        string(END),
    ]))
}
