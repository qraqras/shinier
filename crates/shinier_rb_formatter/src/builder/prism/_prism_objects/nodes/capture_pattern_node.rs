use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, line, space, string};
use crate::document::Document;
use crate::keyword::ROCKET;
use ruby_prism::CapturePatternNode;

impl<'sh> Build for Option<&CapturePatternNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

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
