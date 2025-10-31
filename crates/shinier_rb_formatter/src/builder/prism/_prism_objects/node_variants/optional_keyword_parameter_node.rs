use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, none, space, string};
use crate::document::Document;
use crate::keyword::{ASTERISK, COLON};
use ruby_prism::OptionalKeywordParameterNode;

impl<'sh> Build for OptionalKeywordParameterNode<'sh> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(self, context)
    }
}

pub fn build_node(node: &OptionalKeywordParameterNode, context: &mut BuildContext) -> Document {
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    let value = node.value();
    group(array(&[
        match is_repeated_parameter {
            true => string(ASTERISK),
            false => none(),
        },
        name.build(context),
        string(COLON),
        space(),
        value.build(context),
    ]))
}
