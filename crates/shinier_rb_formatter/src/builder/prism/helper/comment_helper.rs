use crate::builder::BuildContext;
use ruby_prism::Comment;

pub fn update_dangling_remaining<'sh>(
    dangling: &mut Option<Vec<Comment<'sh>>>,
    remaining: &mut Option<Vec<Comment<'sh>>>,
    start_offset: usize,
    _end_offset: usize,
    context: &mut BuildContext<'sh>,
) {
    let index = &context.line_break_index;
    let node_col = index.col_at_offset(start_offset);

    match remaining {
        Some(remaining) => {
            let mut idx = 0;
            for (i, remaining_comment) in remaining.iter().enumerate() {
                let comment_col = index.col_at_offset(remaining_comment.location().start_offset());
                if node_col <= comment_col {
                    idx = i + 1;
                    continue;
                }
                break;
            }
            let additional_dangling = remaining.drain(0..idx).collect();
            match dangling {
                Some(d) => d.extend(additional_dangling),
                None => *dangling = Some(additional_dangling),
            }
        }
        None => {}
    };
}
