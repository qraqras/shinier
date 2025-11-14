use crate::BuildContext;
use crate::builder::builder::array;
use crate::builder::builder::hardline;
use crate::document::Document;
use ruby_prism::Node;

/// line break index for efficient line break queries
pub struct LineBreakIndex {
    pub positions: Vec<usize>,
    source: Vec<u8>,
}

impl LineBreakIndex {
    pub fn new(source: &[u8]) -> Self {
        let positions = source
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b == b'\n' { Some(i) } else { None })
            .collect();
        Self {
            positions,
            source: source.to_vec(),
        }
    }
    pub fn has_line_break_in_range(&self, start_offset: usize, end_offset: usize) -> bool {
        if start_offset >= end_offset {
            return false;
        }
        let idx = self
            .positions
            .partition_point(|&position| position < start_offset);
        self.positions
            .get(idx)
            .map_or(false, |&pos| pos < end_offset)
    }
    pub fn line_number_at_offset(&self, start_offset: usize, end_offset: usize) -> (usize, usize) {
        let start_line = self
            .positions
            .partition_point(|&position| position < start_offset)
            + 1;
        let end_line = self
            .positions
            .partition_point(|&position| position < end_offset)
            + 1;
        (start_line, end_line)
    }
    pub fn get_line_start_offset(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }
        let idx = self.positions.partition_point(|&pos| pos < offset);
        if idx == 0 {
            0
        } else {
            self.positions[idx - 1] + 1
        }
    }
    pub fn col_at_offset(&self, offset: usize) -> usize {
        let line_start_idx = self
            .positions
            .partition_point(|&position| position < offset);
        let line_start_offset = if line_start_idx == 0 {
            0
        } else {
            self.positions[line_start_idx - 1] + 1
        };
        offset - line_start_offset + 1
    }
    pub fn count_leading_blank_lines(&self, offset: usize) -> usize {
        let idx = self.positions.partition_point(|&pos| pos < offset);
        if idx == 0 {
            return 0;
        }
        let mut blank_count = 0;
        let mut current_idx = idx - 1; // end of the previous line
        loop {
            let line_end = self.positions[current_idx];
            let line_start = if current_idx > 0 {
                self.positions[current_idx - 1] + 1
            } else {
                0
            };
            let line_content = &self.source[line_start..line_end];
            if line_content.iter().all(|&b| b == b' ' || b == b'\t') {
                blank_count += 1;
            } else {
                break;
            }
            if current_idx == 0 {
                break;
            }
            current_idx -= 1;
        }
        blank_count
    }
}

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

pub fn leading_blank_lines(node: &Node, context: &mut BuildContext) -> Option<Document> {
    blank_lines(
        context,
        context.built_end,
        node.location().start_offset(),
        context.max_blank_lines,
    )
}
