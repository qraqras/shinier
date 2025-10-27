use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, space, string};
use crate::document::Document;
use crate::keyword::WRITE_OPERATOR;
use ruby_prism::OptionalParameterNode;

pub fn build_node(node: Option<&OptionalParameterNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
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
