use crate::Document;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_node_as_location;
use ruby_prism::BackReferenceReadNode;

pub fn build_back_reference_read_node(
    node: &BackReferenceReadNode<'_>,
    ctx: &mut BuildContext,
) -> Option<Document> {
    build_node_as_location(&node.as_node(), ctx)
}
