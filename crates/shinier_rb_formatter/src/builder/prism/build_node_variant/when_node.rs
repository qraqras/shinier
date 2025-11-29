use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_comments::*;
use crate::builder::prism::build_location::build_location;
use crate::builder::prism::build_node::build_node;
use crate::builder::prism::builder_helper::*;
use ruby_prism::WhenNode;

pub fn build_when_node(node: &WhenNode<'_>, ctx: &mut BuildContext) -> Option<Document> {
    let keyword_loc = node.keyword_loc();
    let conditions = node.conditions();
    let _then_keyword_loc = node.then_keyword_loc();
    let statements = node.statements();

    let key_word_loc_end_offset = keyword_loc.end_offset();
    let conditions_start_offset = conditions.iter().next().map(|c| c.location().start_offset());
    let break_if_comments = match conditions_start_offset {
        Some(offset) => line_if_has_comments(key_word_loc_end_offset, offset, ctx),
        None => unreachable!(),
    };

    // 末尾の条件のEndOfLineはBreakParentしてしまうとフォーマットが崩れるので取り出す
    let mut conditions_trailings = None;
    conditions.iter().last().map(|last_condition| {
        let start_offset = last_condition.location().start_offset();
        let end_offset = last_condition.location().end_offset();
        conditions_trailings = ctx.comment_store.take_trailings(start_offset, end_offset);
    });

    let mut conditions_doc = Vec::new();
    //let mut break_conditions = Vec::new();
    for (i, condition) in conditions.iter().enumerate() {
        if i > 0 {
            conditions_doc.push(comma());
            conditions_doc.push(line());
        }
        conditions_doc.push(build_node(condition, ctx));
    }

    group(array(&[
        build_location(keyword_loc, ctx),
        indent(group(array(&[break_if_comments, array(&conditions_doc)]))),
        indent(build_comments_as_trailing(conditions_trailings, ctx)),
        indent(
            statements
                .map(|s| array(&[hardline(), build_node(s.as_node(), ctx)]))
                .flatten(),
        ),
    ]))
}
