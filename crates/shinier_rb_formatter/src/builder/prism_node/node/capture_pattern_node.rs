use crate::BuildContext;
use crate::BuildPrismNode;
use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::keyword::ROCKET;
use ruby_prism::CapturePatternNode;

pub fn build_node(node: Option<&CapturePatternNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let target = node.target();
    let value = node.value();
    group(array(&[
        target.as_node().build(context),
        space(),
        string(ROCKET),
        line(),
        value.build(context),
    ]))
}
