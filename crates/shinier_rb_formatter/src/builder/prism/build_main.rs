use crate::Document;
use crate::builder::BuildContext;
use crate::builder::builder::*;
use crate::builder::prism::build_blank_lines::*;
use crate::builder::prism::build_comments::*;
use crate::builder::prism::target::Target;

/// Builds the main Document for a given node or location, including comments and blank lines.
pub fn build_main<'sh, B, P>(builder: B, target: Target<'sh>, param: &P, context: &mut BuildContext) -> Option<Document>
where
    B: Fn(&Target<'sh>, &mut BuildContext, &P) -> Option<Document>,
{
    // ** GET OFFSETS **
    let start_offset = target.start_offset();
    let end_offset = target.end_offset();

    // ** GET BLANK LINES **
    let blank_lines = match context.previous_start_offset < start_offset {
        true => leading_blank_lines(start_offset, end_offset, context),
        false => None,
    };

    // ** GET CURRENT COMMENTS **
    let current_leading_comments = context.comment_store.take_leadings(start_offset, end_offset);
    let current_trailing_comments = context.comment_store.take_trailings(start_offset, end_offset);
    let current_dangling_comments = context.comment_store.take_danglings(start_offset, end_offset);

    // ** UPDATE PREVIOUS START OFFSET **
    context.previous_start_offset = start_offset.max(context.previous_start_offset);

    // ** MAIN BUILDING PROCESS **
    let built = builder(&target, context, param);

    // ** BUILD COMMENTS AND ASSEMBLE FINAL DOCUMENT **
    let leading_comments = build_comments_as_leading(current_leading_comments, context);
    let trailing_comments = build_comments_as_trailing(current_trailing_comments, context);
    let dangling_comments = build_comments_as_dangling(current_dangling_comments, context);
    array(&[
        leading_comments,
        blank_lines,
        built,
        trailing_comments,
        dangling_comments,
    ])
}
