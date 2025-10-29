use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, hardline, indent, space, string};
use crate::document::Document;
use crate::keyword::IN;
use ruby_prism::InNode;

impl<'sh> Build for Option<&InNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&InNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let pattern = node.pattern();
    let statements = node.statements();
    group(array(&[
        string(IN),
        space(),
        pattern.build(context),
        indent(
            statements
                .as_ref()
                .build_with(context, Some(hardline()), None),
        ),
    ]))
}
