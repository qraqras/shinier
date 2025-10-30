use crate::BuildContext;
use crate::builder::builder::hardline;
use crate::document::Document;

pub fn build_leading_line_breaks(
    context: &mut BuildContext,
    start_offset: usize,
    max_line_breaks: usize,
) -> Option<Document> {
    fn is_indent_char(c: &u8) -> bool {
        matches!(c, b' ' | b'\t')
    }
    fn is_line_break_char(c: &u8) -> bool {
        matches!(c, b'\n')
    }
    let gap_start = context.built_end;
    let gap_end = start_offset;
    if gap_start < gap_end {
        let mut i = gap_start;
        let mut break_lines = 0usize;
        while i < gap_end {
            if is_line_break_char(&context.source[i]) {
                let mut j = i + 1;
                while j < gap_end && is_indent_char(&context.source[j]) {
                    j += 1;
                }
                if j < gap_end && is_line_break_char(&context.source[j]) {
                    break_lines += 1;
                    if break_lines >= max_line_breaks {
                        break;
                    }
                    i = j;
                    continue;
                }
                i = j;
            }
            i += 1;
        }
        if break_lines > 0 {
            context.built_end = gap_end.min(context.source.len());
            return Some(Document::Array(vec![hardline(); break_lines]));
        }
    }
    None
}
