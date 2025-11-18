use crate::Document;
use crate::builder::COMMA;
use crate::builder::builder::*;
use crate::builder::prism::BuildContext;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::RescueNode;

pub fn build_rescue_node(node: &RescueNode<'_>, context: &mut BuildContext) -> Document {
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
            exceptions_document.push(array(&[string(COMMA), line()]));
        }
        exceptions_document.push(build_node(&exception, context));
    }

    group(array_opt(&[
        Some(build_location(&keyword_loc, context)),
        Some(array(&exceptions_document)),
        operator_loc.map(|l| array(&[space(), build_location(&l, context)])),
        reference.map(|n| array(&[space(), build_node(&n, context)])),
        then_keyword_loc.map(|l| array(&[space(), build_location(&l, context)])),
        statements.map(|n| indent(array_opt(&[Some(hardline()), Some(build_node(&n.as_node(), context))]))),
        subsequent.map(|n| array(&[hardline(), build_node(&n.as_node(), context)])),
    ]))
}
