use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::ElseNode;
use ruby_prism::Location;

pub fn build_else_node(node: &ElseNode<'_>, context: &mut BuildContext) -> Document {
    let else_keyword_loc = node.else_keyword_loc();
    let statements = node.statements();
    let mut end_keyword_loc = node.end_keyword_loc();

    // The end_keyword_loc may point to the next clause keyword (e.g., "ensure")
    // when an else block is followed by ensure. We only output "end" keywords,
    // not other clause keywords, so we filter them out here.
    if !is_end_keyword(&end_keyword_loc) {
        end_keyword_loc = None;
    };

    group(array_opt(&[
        Some(build_location(&else_keyword_loc, context)),
        statements.map(|n| indent(array_opt(&[Some(hardline()), Some(build_node(&n.as_node(), context))]))),
        end_keyword_loc.map(|l| array(&[hardline(), build_location(&l, context)])),
    ]))
}

/// Check if the given location corresponds to the "end" keyword.
fn is_end_keyword(location: &Option<Location>) -> bool {
    location
        .as_ref()
        .and_then(|l| std::str::from_utf8(l.as_slice()).ok())
        .map(|s| s == "end")
        .unwrap_or(false)
}
