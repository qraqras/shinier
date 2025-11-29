use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use ruby_prism::*;

// TODO: doキーワードに付与されたコメントが消えないことを確認する
pub fn build_for_node(node: &ForNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let index = node.index();
    let collection = node.collection();
    let statements = node.statements();
    let for_keyword_loc = node.for_keyword_loc();
    let in_keyword_loc = node.in_keyword_loc();
    let do_keyword_loc = node.do_keyword_loc();
    let end_keyword_loc = node.end_keyword_loc();

    group(array(&[
        build_location(for_keyword_loc, ctx),
        indent(group(array(&[line(), build_node(index, ctx)]))),
        space(),
        build_location(in_keyword_loc, ctx),
        indent(group(array(&[line(), build_node(collection, ctx)]))),
        indent(
            statements
                .map(|s| array(&[hardline(), build_node(s.as_node(), ctx)]))
                .flatten(),
        ),
        hardline(),
        build_location(end_keyword_loc, ctx),
    ]))
}
