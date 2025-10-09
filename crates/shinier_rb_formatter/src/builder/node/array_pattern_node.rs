use crate::builder::build_optional;
use crate::doc::*;
use crate::layout::separate_nodelist;
use ruby_prism::ArrayPatternNode;

const OPEN_DELIMITER: &str = "[";
const CLOSE_DELIMITER: &str = "]";
const SEPARATOR: &str = ",";

pub fn build_node(node: Option<&ArrayPatternNode>) -> Doc {
    let node = node.unwrap();
    let constant = node.constant();
    let requireds = node.requireds();
    let rest = node.rest();
    let posts = node.posts();

    let separated_requireds = separate_nodelist(&requireds, SEPARATOR);
    let separated_posts = separate_nodelist(&posts, SEPARATOR);

    sequence(&[
        build_optional(constant.as_ref()),
        text(OPEN_DELIMITER),
        indent(&[
            softline(),
            sequence(&separated_requireds),
            none_if_false(
                rest.is_some() && separated_requireds.len() > 0,
                sequence(&[text(SEPARATOR), line()]),
            ),
            build_optional(rest.as_ref()),
            none_if_false(
                separated_posts.len() > 0 && rest.is_some(),
                sequence(&[text(SEPARATOR), line()]),
            ),
            sequence(&separated_posts),
        ]),
        text(CLOSE_DELIMITER),
    ])
}
