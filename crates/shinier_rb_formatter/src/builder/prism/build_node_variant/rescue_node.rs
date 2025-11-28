use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_comments::build_dangling;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::RescueNode;

pub fn build_rescue_node(node: &RescueNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let keyword_loc = node.keyword_loc();
    let exceptions = node.exceptions();
    let operator_loc = node.operator_loc();
    let reference = node.reference();
    let then_keyword_loc = node.then_keyword_loc();
    let statements = node.statements();
    let subsequent = node.subsequent();

    let mut exceptions_document = Vec::new();
    for (i, exception) in exceptions.iter().enumerate() {
        if i == 0 {
            // First exception does not need a preceding space
            exceptions_document.push(space());
        }
        if i > 0 {
            exceptions_document.push(comma());
            exceptions_document.push(line());
        }
        exceptions_document.push(build_node(exception, ctx));
    }

    let dangling = build_dangling(&node.as_node(), ctx);

    group(array(&[
        build_location(keyword_loc, ctx),
        array(&exceptions_document),
        operator_loc
            .map(|l| array(&[space(), build_location(l, ctx)]))
            .flatten(),
        reference.map(|n| array(&[space(), build_node(n, ctx)])).flatten(),
        then_keyword_loc
            .map(|l| array(&[space(), build_location(l, ctx)]))
            .flatten(),
        indent(array(&[
            statements
                .map(|n| array(&[hardline(), build_node(n.as_node(), ctx)]))
                .flatten(),
            hardline(),
            dangling,
        ])),
        subsequent
            .map(|n| array(&[hardline(), build_node(n.as_node(), ctx)]))
            .flatten(),
    ]))
}
