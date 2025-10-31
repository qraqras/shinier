use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use crate::keyword::ROCKET;
use ruby_prism::MatchRequiredNode;

impl<'sh> Build for MatchRequiredNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &MatchRequiredNode, context: &mut BuildContext) -> Document {
    let value = node.value();
    let pattern = node.pattern();
    group(array(&[
        value.build(context),
        space(),
        string(ROCKET),
        indent(array(&[line(), pattern.build(context)])),
    ]))
}
