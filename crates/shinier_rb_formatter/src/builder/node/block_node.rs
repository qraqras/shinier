use crate::{build_optional, doc::*};
use ruby_prism::*;

const OPEN_DELIMITER: &str = "{";
const CLOSE_DELIMITER: &str = "}";
const PARAMETERS_SEPARATOR: &str = "|";

pub fn build_node(node: Option<&BlockNode>) -> Doc {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let body = node.body();
            sequence(&[
                text(OPEN_DELIMITER),
                build_optional(parameters.as_ref()),
                build_optional(body.as_ref()),
                text(CLOSE_DELIMITER),
            ])
        }
        None => none(),
    }
}
