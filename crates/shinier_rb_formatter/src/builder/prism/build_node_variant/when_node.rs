use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::WhenNode;

pub fn build_when_node(node: &WhenNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let keyword_loc = node.keyword_loc();
    let conditions = node.conditions();
    let then_keyword_loc = node.then_keyword_loc();
    let statements = node.statements();

    let mut conditions_docs = Vec::new();
    for (i, condition) in conditions.iter().enumerate() {
        if i > 0 {
            conditions_docs.push(comma());
            conditions_docs.push(line());
        }
        conditions_docs.push(build_node(condition, ctx));
    }

    group(array(&[
        build_location(keyword_loc, ctx),
        indent(group(array(&[line(), array(&conditions_docs)]))),
        indent(
            then_keyword_loc
                .map(|t| array(&[line(), build_location(t, ctx)]))
                .flatten(),
        ),
        indent(
            statements
                .map(|s| array(&[hardline(), build_node(s.as_node(), ctx)]))
                .flatten(),
        ),
    ]))
}
