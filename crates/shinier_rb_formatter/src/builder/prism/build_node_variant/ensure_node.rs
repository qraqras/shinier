use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::EnsureNode;

pub fn build_ensure_node(node: &EnsureNode<'_>, context: &mut BuildContext) -> Document {
    let ensure_keyword_loc = node.ensure_keyword_loc();
    let statements = node.statements();
    let end_keyword_loc = node.end_keyword_loc();

    group(array_opt(&[
        build_location(&ensure_keyword_loc, context),
        statements.map(|n| indent(array(&[hardline(), build_node(&n.as_node(), context)]))),
        build_location(&end_keyword_loc, context).map(|e| array(&[hardline(), e])),
    ]))
}
