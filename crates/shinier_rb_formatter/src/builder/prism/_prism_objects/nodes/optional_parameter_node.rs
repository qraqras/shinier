use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::WRITE_OPERATOR;
use ruby_prism::OptionalParameterNode;

impl<'sh> Build for OptionalParameterNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &OptionalParameterNode, context: &mut BuildContext) -> Document {
    let name = node.name();
    let value = node.value();
    group(array(&[
        name.build(context),
        space(),
        string(WRITE_OPERATOR),
        space(),
        value.build(context),
    ]))
}
