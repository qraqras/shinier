use crate::BuildContext;
use crate::builder::builder::{array, hardline};
use crate::document::Document;

/// Builds leading line breaks before a given offset, up to a maximum number of line breaks.
/// ```ruby
///
/// foo
/// ```
pub fn leading_line_breaks(
    context: &mut BuildContext,
    start_offset: usize,
    max_line_breaks: usize,
) -> Document {
    fn is_indent_char(c: &u8) -> bool {
        matches!(c, b' ' | b'\t')
    }
    fn is_line_break_char(c: &u8) -> bool {
        matches!(c, b'\n')
    }
    let mut documents = Vec::new();
    let gap_start = context.built_end;
    let gap_end = start_offset;
    if gap_start < gap_end {
        let mut i = gap_start;
        while i < gap_end {
            if is_line_break_char(&context.source[i]) {
                let mut j = i + 1;
                while j < gap_end && is_indent_char(&context.source[j]) {
                    j += 1;
                }
                if j < gap_end && is_line_break_char(&context.source[j]) {
                    documents.push(hardline());
                    if documents.len() >= max_line_breaks {
                        break;
                    }
                    i = j;
                    continue;
                }
                i = j;
            }
            i += 1;
        }
    }
    context.built_end = gap_end;
    array(&documents)
}
