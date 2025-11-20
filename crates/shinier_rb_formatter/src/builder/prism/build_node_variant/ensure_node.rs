use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::EnsureNode;

pub fn build_ensure_node(node: &EnsureNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let ensure_keyword_loc = node.ensure_keyword_loc();
    let statements = node.statements();
    let end_keyword_loc = node.end_keyword_loc();

    group(array(&[
        build_location(&ensure_keyword_loc, ctx),
        statements
            .map(|n| indent(array(&[hardline(), build_node(&n.as_node(), ctx)])))
            .flatten(),
        build_location(&end_keyword_loc, ctx)
            .map(|e| array(&[hardline(), Some(e)]))
            .flatten(),
    ]))
}
