use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::comments::Target;
use crate::builder::prism::helper::build_blank_lines::*;
use crate::builder::prism::helper::build_comments::*;
use crate::builder::prism::helper::comment_helper::update_dangling_remaining;

/// Builds the main Document for a given node or location, including comments and blank lines.
pub fn build_main<B, V, P>(
    builder: B,
    value: &V,
    param: &P,
    context: &mut BuildContext,
    target: &Target,
) -> Option<Document>
where
    B: Fn(&V, &mut BuildContext, &P) -> Option<Document>,
{
    // ** GET OFFSETS **
    let start_offset = target.start_offset();
    let end_offset = target.end_offset();

    // ** GET BLANK LINES **
    let blank_lines = match context.previous_start_offset < start_offset {
        true => leading_blank_lines(start_offset, end_offset, context),
        false => None,
    };

    // ** SAVE PREVIOUS REMAINING COMMENTS **
    let previous_remaining_comments = context.remaining_comments.take();

    // ** GET CURRENT COMMENTS **
    let current_leading_comments = context.comment_store.pop_leading(start_offset, end_offset);
    let current_trailing_comments = context.comment_store.pop_trailing(start_offset, end_offset);
    let mut current_dangling_comments = context.comment_store.pop_dangling(start_offset, end_offset);
    let mut current_remaining_comments = context.comment_store.pop_remaining(start_offset, end_offset);

    // ** UPDATE PREVIOUS START OFFSET **
    context.previous_start_offset = start_offset.max(context.previous_start_offset);

    // ** MAIN BUILDING PROCESS **
    let built = builder(value, context, param);

    // ** UPDATE COMMENTS **
    update_dangling_remaining(
        &mut current_dangling_comments,
        &mut current_remaining_comments,
        start_offset,
        end_offset,
        context,
    );

    // ** MERGE CURRENT REMAINING COMMENTS INTO CONTEXT'S REMAINING COMMENTS **
    context.remaining_comments = match (context.remaining_comments.take(), current_remaining_comments) {
        (Some(prev), Some(curr)) => Some(merge_vector(prev, curr)),
        (Some(prev), None) => Some(prev),
        (None, Some(curr)) => Some(curr),
        (None, None) => None,
    }
    .filter(|merged| !merged.is_empty());

    // ** MERGE PREVIOUS REMAINING COMMENTS INTO LEADING COMMENTS **
    let current_leading_comments = match (previous_remaining_comments, current_leading_comments) {
        (Some(remaining), Some(leading)) => Some(merge_vector(remaining, leading)),
        (Some(remaining), None) => Some(remaining),
        (None, Some(leading)) => Some(leading),
        (None, None) => None,
    }
    .filter(|merged| !merged.is_empty());

    // ** BUILD COMMENTS AND ASSEMBLE FINAL DOCUMENT **
    let leading_comments = build_comments_as_leading(current_leading_comments, context);
    let trailing_comments = build_comments_as_trailing(current_trailing_comments, context);
    let dangling_comments = build_comments_as_dangling(current_dangling_comments, context);
    let dangling_comments = match target.is_location() {
        true => indent(dangling_comments),
        false => dangling_comments,
    };
    array(&[
        leading_comments,
        blank_lines,
        built,
        trailing_comments,
        dangling_comments,
    ])
}

// Merges two vectors into one.
fn merge_vector<T>(mut v1: Vec<T>, mut v2: Vec<T>) -> Vec<T> {
    v1.append(&mut v2);
    v1
}
