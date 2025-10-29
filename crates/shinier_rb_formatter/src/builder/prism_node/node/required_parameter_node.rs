use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, none, string};
use crate::document::Document;
use crate::keyword::ASTERISK;
use ruby_prism::RequiredParameterNode;

impl<'sh> Build for Option<&RequiredParameterNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&RequiredParameterNode>, context: &mut BuildContext) -> Document {
    match node {
        Some(node) => {
            let is_repeated_parameter = node.is_repeated_parameter();
            let name = node.name();
            group(array(&[
                match is_repeated_parameter {
                    true => string(ASTERISK),
                    false => none(),
                },
                name.build(context),
            ]))
        }
        None => none(),
    }
}
