use crate::BuildContext;
use crate::Build;
use crate::builder::builder::string;
use crate::document::Document;
use ruby_prism::NumberedParametersNode;

impl<'sh> Build for Option<&NumberedParametersNode<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        build_node(*self, context)
    }
}

pub fn build_node(node: Option<&NumberedParametersNode>, _context: &mut BuildContext) -> Document {
    let node = node.unwrap();
    let maximum = node.maximum();
    string(maximum.to_string())
}
