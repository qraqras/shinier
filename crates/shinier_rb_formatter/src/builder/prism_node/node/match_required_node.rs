use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, indent, line, space, string};
use crate::document::Document;
use crate::keyword::ROCKET;
use ruby_prism::MatchRequiredNode;

pub fn build_node(node: Option<&MatchRequiredNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let value = node.value();
    let pattern = node.pattern();
    group(array(&[
        value.build(context),
        space(),
        string(ROCKET),
        indent(array(&[line(), pattern.build(context)])),
    ]))
}
