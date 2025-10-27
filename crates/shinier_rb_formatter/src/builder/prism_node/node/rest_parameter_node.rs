use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, none, string};
use crate::document::Document;
use crate::keyword::ASTERISK;
use ruby_prism::RestParameterNode;

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
