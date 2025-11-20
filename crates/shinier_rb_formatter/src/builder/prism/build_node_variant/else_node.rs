use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::helper::location_helper::is_end_keyword;
use ruby_prism::ElseNode;

pub fn build_else_node(node: &ElseNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let else_keyword_loc = node.else_keyword_loc();
    let statements = node.statements();
    let mut end_keyword_loc = node.end_keyword_loc();

    // The end_keyword_loc may point to the next clause keyword (e.g., "ensure")
    // when an else block is followed by ensure. We only output "end" keywords,
    // not other clause keywords, so we filter them out here.
    if !is_end_keyword(&end_keyword_loc) {
        end_keyword_loc = None;
    };

    group(array(&[
        build_location(&else_keyword_loc, ctx),
        statements
            .map(|n| indent(array(&[hardline(), build_node(&n.as_node(), ctx)])))
            .flatten(),
        end_keyword_loc
            .and_then(|l| build_location(&l, ctx).map(|e| array(&[hardline(), Some(e)])))
            .flatten(),
    ]))
}
