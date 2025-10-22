use crate::doc::*;

pub struct Renderer {
    pub column: usize,
    pub is_flat: bool,
    pub output: String,
    group_stack: Vec<(usize, bool)>,
    indent_level: usize,
    indent_unit: String,
    column_max: usize,
}
impl Renderer {
    pub fn new(indent_unit: &str, column_max: usize) -> Self {
        Self {
            column: 0,
            is_flat: true,
            output: String::new(),
            group_stack: Vec::new(),
            indent_unit: indent_unit.to_string(),
            indent_level: 0,
            column_max,
        }
    }
    pub fn render(&mut self, doc: &Doc) {
        self.render_doc(doc);
    }

    fn render_doc(&mut self, doc: &Doc) {
        let previous_is_flat = self.is_flat;
        match doc {
            Doc::BeginIndent(_begin_indent) => {
                self.indent_level += 1;
            }
            Doc::EndIndent(_end_indent) => {
                self.indent_level -= 1;
            }
            Doc::Fill(fill) => {
                match &*fill.doc {
                    Doc::Sequence(sequence) => {
                        for ref doc in sequence.docs.iter() {
                            self.is_flat = self.fits(doc);
                            self.render_doc(doc);
                        }
                    }
                    _ => {
                        self.is_flat = self.fits(&fill.doc);
                        self.render_doc(&fill.doc);
                    }
                };
            }
            Doc::Group(group) => {
                // Prettier-style: check if group fits on current line
                let can_fit = self.fits(&group.doc);
                self.group_stack.push((group.id, can_fit));
                self.is_flat = can_fit;
                self.render_doc(&*group.doc);
                self.group_stack.pop();
            }
            Doc::HardLine(_hard_line) => {
                self.write_newline();
            }
            Doc::IfBreak(if_break) => {
                let is_flat = self.is_flat_group(if_break.group_id);
                if is_flat {
                    self.render_doc(&*if_break.flat);
                } else {
                    self.render_doc(&*if_break.r#break);
                }
            }
            Doc::Indent(indent) => {
                return self.render_with_indent(&*indent.doc);
            }
            Doc::IndentIfBreak(indent_if_break) => {
                let is_flat = self.is_flat_group(indent_if_break.group_id);
                if is_flat {
                    self.render_doc(&*indent_if_break.doc);
                } else {
                    self.render_with_indent(&*indent_if_break.doc);
                }
            }
            Doc::Line(_line) => {
                if self.is_flat {
                    self.write_space();
                } else {
                    self.write_newline();
                }
            }
            Doc::None(_none) => {}
            Doc::Sequence(sequence) => {
                for ref doc in sequence.docs.iter() {
                    self.render_doc(doc);
                }
            }
            Doc::Space(_space) => {
                self.write_space();
            }
            Doc::SoftLine(_soft_line) => {
                if !self.is_flat {
                    self.write_newline();
                }
            }
            Doc::Text(text) => {
                self.write_text(&text.text);
            }
        }
        self.is_flat = previous_is_flat;
    }
    fn render_with_indent(&mut self, doc: &Doc) {
        let previous_is_flat = self.is_flat;
        let previous_indent_level = self.indent_level;
        self.indent_level += 1;
        self.render_doc(doc);
        self.indent_level = previous_indent_level;
        self.is_flat = previous_is_flat;
    }
    fn write_text(&mut self, text: &str) {
        self.write_indent();
        self.output.push_str(text);
        self.column += text.len();
    }
    fn write_space(&mut self) {
        self.write_text(" ");
    }
    fn write_indent(&mut self) {
        if self.column == 0 && self.indent_level > 0 {
            let indent = self.indent_unit.repeat(self.indent_level);
            self.output.push_str(indent.as_str());
            self.column += indent.len();
        }
    }
    fn write_newline(&mut self) {
        self.output.push_str("\n");
        self.column = 0;
    }
    fn fits(&self, doc: &Doc) -> bool {
        let remaining = self.column_max.saturating_sub(self.column);
        self.fits_impl(doc, self.column, remaining)
    }

    fn fits_impl(&self, doc: &Doc, mut column: usize, mut remaining: usize) -> bool {
        if remaining == 0 {
            return false;
        }

        match doc {
            Doc::Text(text) => {
                let len = text.text.len();
                remaining >= len
            }
            Doc::Space(_) => remaining >= 1,
            Doc::Line(_) => {
                // Line becomes space in flat mode
                remaining >= 1
            }
            Doc::SoftLine(_) => {
                // SoftLine becomes empty in flat mode
                true
            }
            Doc::HardLine(_) => {
                // HardLine always forces break
                false
            }
            Doc::Group(group) => {
                // Groups are measured in flat mode
                self.fits_impl(&group.doc, column, remaining)
            }
            Doc::Sequence(sequence) => {
                for doc in &sequence.docs {
                    if !self.fits_impl(doc, column, remaining) {
                        return false;
                    }
                    // Update column and remaining for next element
                    let width = self.measure_flat(doc);
                    column = column.saturating_add(width);
                    remaining = remaining.saturating_sub(width);
                    if remaining == 0 {
                        return false;
                    }
                }
                true
            }
            Doc::Indent(inner) => self.fits_impl(&inner.doc, column, remaining),
            Doc::IndentIfBreak(inner) => self.fits_impl(&inner.doc, column, remaining),
            Doc::Fill(fill) => self.fits_impl(&fill.doc, column, remaining),
            Doc::IfBreak(if_break) => {
                // Use flat version when checking if it fits
                self.fits_impl(&if_break.flat, column, remaining)
            }
            Doc::BeginIndent(_) | Doc::EndIndent(_) | Doc::None(_) => true,
        }
    }

    fn measure_flat(&self, doc: &Doc) -> usize {
        match doc {
            Doc::Text(text) => text.text.len(),
            Doc::Space(_) => 1,
            Doc::Line(_) => 1,
            Doc::SoftLine(_) => 0,
            Doc::HardLine(_) => 0,
            Doc::Group(group) => self.measure_flat(&group.doc),
            Doc::Sequence(sequence) => {
                let mut total = 0;
                for doc in &sequence.docs {
                    total += self.measure_flat(doc);
                }
                total
            }
            Doc::Indent(inner) => self.measure_flat(&inner.doc),
            Doc::IndentIfBreak(inner) => self.measure_flat(&inner.doc),
            Doc::Fill(fill) => self.measure_flat(&fill.doc),
            Doc::IfBreak(if_break) => self.measure_flat(&if_break.flat),
            Doc::BeginIndent(_) | Doc::EndIndent(_) | Doc::None(_) => 0,
        }
    }
    fn is_flat_group(&self, group_id: Option<usize>) -> bool {
        match group_id {
            Some(group_id) => {
                for group in self.group_stack.iter() {
                    if group.0 == group_id {
                        return group.1;
                    }
                }
                true
            }
            None => {
                if let Some((_, is_flat)) = self.group_stack.last() {
                    *is_flat
                } else {
                    true
                }
            }
        }
    }
}
