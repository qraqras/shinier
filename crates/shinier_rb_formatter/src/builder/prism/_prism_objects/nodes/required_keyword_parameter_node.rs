use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use crate::keyword::{ASTERISK, COLON};
use ruby_prism::RequiredKeywordParameterNode;

impl<'sh> Build for Option<&RequiredKeywordParameterNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(
    node: Option<&RequiredKeywordParameterNode>,
    context: &mut BuildContext,
) -> Document {
    let node = node.unwrap();
    let is_repeated_parameter = node.is_repeated_parameter();
    let name = node.name();
    group(array(&[
        match is_repeated_parameter {
            true => string(ASTERISK),
            false => none(),
        },
        name.build(context),
        string(COLON),
    ]))
}
