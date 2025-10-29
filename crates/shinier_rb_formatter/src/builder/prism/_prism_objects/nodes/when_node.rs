use crate::Build;
use crate::BuildContext;
use crate::ListBuild;
use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::WHEN;
use ruby_prism::WhenNode;

impl<'sh> Build for Option<&WhenNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&WhenNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let conditions = node.conditions();
    let statements = node.statements();
    group(array(&[
        string(WHEN),
        space(),
        conditions.build(context, &hardline()),
        indent(
            statements
                .as_ref()
                .build_with(context, Some(hardline()), None),
        ),
    ]))
}
