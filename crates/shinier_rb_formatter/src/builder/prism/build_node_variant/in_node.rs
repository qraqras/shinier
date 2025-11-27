use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::InNode;

pub fn build_in_node(node: &InNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let pattern = node.pattern();
    let statements = node.statements();
    let in_loc = node.in_loc();
    let then_loc = node.then_loc();

    group(array(&[
        build_location(in_loc, ctx),
        indent(array(&[line(), build_node(pattern, ctx)])),
        statements
            .map(|s| indent(array(&[hardline(), build_node(s.as_node(), ctx)])))
            .flatten(),
        then_loc.map(|t| array(&[hardline(), build_location(t, ctx)])).flatten(),
    ]))
}
