use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::InNode;

pub fn build_in_node(node: &InNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let pattern = node.pattern();
    let statements = node.statements();
    let in_loc = node.in_loc();
    let _then_loc = node.then_loc();

    let break_if_comments = line_if_has_comments(in_loc.end_offset(), pattern.location().start_offset(), ctx);

    group(array(&[
        build_location(in_loc, ctx),
        indent(array(&[break_if_comments, build_node(pattern, ctx)])),
        statements
            .map(|s| indent(array(&[hardline(), build_node(s.as_node(), ctx)])))
            .flatten(),
    ]))
}
