use crate::BuildContext;
use crate::builder::builder::{array, hardline};
use crate::document::Document;

/// Builds leading line breaks before a given offset, up to a maximum number of line breaks.
/// ```ruby
///
/// foo
/// ```
pub fn blank_lines(
    context: &mut BuildContext,
    gap_start_offset: usize,
    gap_end_offset: usize,
    max_line_breaks: usize,
) -> Option<Document> {
    // Helper functions to identify indent and line break characters
    fn is_indent_char(c: &u8) -> bool {
        matches!(c, b' ' | b'\t')
    }
    // Helper function to identify line break characters
    fn is_line_break_char(c: &u8) -> bool {
        matches!(c, b'\n')
    }
    // Early return if no line breaks are allowed
    if max_line_breaks == usize::MIN {
        return None;
    }
    let mut documents = Vec::new();
    if gap_start_offset < gap_end_offset {
        let mut i = gap_start_offset;
        while i < gap_end_offset {
            if is_line_break_char(&context.source[i]) {
                let mut j = i + 1;
                while j < gap_end_offset && is_indent_char(&context.source[j]) {
                    j += 1;
                }
                if j < gap_end_offset && is_line_break_char(&context.source[j]) {
                    if documents.len() + 1 > max_line_breaks {
                        break;
                    }
                    documents.push(hardline());
                    i = j;
                    continue;
                }
                i = j;
            }
            i += 1;
        }
    }
    context.built_end = gap_end_offset;
    match documents.is_empty() {
        true => None,
        false => Some(array(&documents)),
    }
}
