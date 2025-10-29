use crate::Build;
use crate::BuildContext;
use crate::builder::builder::{array, group, string};
use crate::document::Document;
use crate::keyword::SPLAT;
use ruby_prism::AssocSplatNode;

impl<'sh> Build for Option<&AssocSplatNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&AssocSplatNode>, context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let value = node.value();
    group(array(&[string(SPLAT), value.build(context)]))
}
