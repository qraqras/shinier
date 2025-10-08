use crate::builder::build;
use crate::doc::*;
use crate::layout::separate;
use ruby_prism::*;

const RESCUE_KEYWORD: &str = "rescue";
const REFERENCE_ARROW: &str = "=>";

pub fn build_node(node: Option<&RescueNode>) -> Doc {
    let node = node.unwrap();
    let exceptions = node.exceptions();
    let reference = node.reference();
    let statements = node.statements();
    let subsequent = node.subsequent();

    let mut vec = Vec::new();

    if let Some(statements) = statements {
        vec.push(build(&statements.as_node()));
    }
    if let Some(subsequent) = subsequent {
        vec.push(build(&subsequent.as_node()));
    }

    sequence(&[
        text(RESCUE_KEYWORD),
        space(),
        fill(&separate(&exceptions, ",")),
        if let Some(reference) = reference {
            sequence(&[space(), text(REFERENCE_ARROW), line(), build(&reference)])
        } else {
            none()
        },
        indent(&[hardline(), group(&vec)]),
        hardline(),
    ])
}
