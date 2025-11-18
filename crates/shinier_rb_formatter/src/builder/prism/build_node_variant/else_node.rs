use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::helper::location::is_end_keyword;
use ruby_prism::ElseNode;

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
        build_location(&else_keyword_loc, context),
        statements.map(|n| indent(array_opt(&[Some(hardline()), Some(build_node(&n.as_node(), context))]))),
        end_keyword_loc.and_then(|l| build_location(&l, context).map(|e| array(&[hardline(), e]))),
    ]))
}
