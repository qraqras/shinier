use crate::{build_optional, doc::*};
use ruby_prism::*;

const OPEN_DELIMITER: &str = "{";
const CLOSE_DELIMITER: &str = "}";

pub fn build_node(node: Option<&BlockNode>) -> Doc {
    match node {
        Some(node) => {
            let parameters = node.parameters();
            let body = node.body();
            group(&[
                text(OPEN_DELIMITER),
                line(),
                indent(&[
                    build_optional(parameters.as_ref()),
                    line(),
                    build_optional(body.as_ref()),
                ]),
                line(),
                text(CLOSE_DELIMITER),
            ])
        }
        None => none(),
    }
}
