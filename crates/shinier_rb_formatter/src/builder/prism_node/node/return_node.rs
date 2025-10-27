use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, space, string};
use crate::document::Document;
use crate::keyword::RETURN;
use ruby_prism::ReturnNode;

pub fn build_node(node: Option<&ReturnNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let arguments = node.arguments();
    group(array(&[
        string(RETURN),
        space(),
        indent(arguments.build(context)),
    ]))
}
