use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use crate::keyword::ASTERISK;
use ruby_prism::RestParameterNode;

impl<'sh> Build for Option<&RestParameterNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&RestParameterNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let name = node.name();
    array(&[
        string(ASTERISK),
        match name {
            Some(name) => name.build(context),
            None => none(),
        },
    ])
}
