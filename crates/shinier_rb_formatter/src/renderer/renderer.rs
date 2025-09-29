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
        let previous_is_flat = self.is_flat;
        match doc {
            Doc::Fill(fill) => {
                for ref doc in fill.docs.iter() {
                    self.is_flat = self.can_fit_in_line_doc(doc);
                    self.render(doc);
                }
            }
            Doc::Group(group) => {
                let can_fit = self.can_fit_in_line_docs(&group.docs);
                self.group_stack.push((group.id, can_fit));
                self.is_flat = can_fit;
                for ref doc in group.docs.iter() {
                    self.render(doc);
                }
                self.group_stack.pop();
            }
            Doc::HardLine(_hard_line) => {
                self.write_newline();
            }
            Doc::IfBreak(if_break) => {
                let groups: Vec<&(usize, bool)> = self
                    .group_stack
                    .iter()
                    .filter(|(id, _)| *id == if_break.group_id)
                    .collect();
                if let Some((_, flat)) = groups.last() {
                    if *flat {
                        self.render(&*if_break.flat);
                    } else {
                        self.render(&*if_break.r#break);
                    }
                } else {
                    self.render(&*if_break.flat);
                }
            }
            Doc::Indent(indent) => {
                return self.render_with_indent(&*indent.doc);
            }
            Doc::IndentIfBreak(indent_if_break) => {
                if self.is_flat {
                    self.render(&*indent_if_break.doc);
                } else {
                    self.render_with_indent(&*indent_if_break.doc);
                }
            }
            Doc::Line(_line) => {
                if self.is_flat {
                    self.write_text(" ");
                } else {
                    self.write_newline();
                }
            }
            Doc::None(_none) => {}
            Doc::Sequence(sequence) => {
                for ref doc in sequence.docs.iter() {
                    self.render(doc);
                }
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
        self.render(doc);
        self.indent_level = previous_indent_level;
        self.is_flat = previous_is_flat;
    }
    fn write_text(&mut self, text: &str) {
        self.write_indent();
        self.output.push_str(text);
        self.column += text.len();
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
    fn can_fit_in_line_doc(&self, doc: &Doc) -> bool {
        let column = self.column + self.measure_doc(doc);
        // 改行がちょうど収まってはいけない(次が必ずはみ出てしまうため)
        match doc {
            Doc::HardLine(_) => column < self.column_max,
            Doc::Line(_) => column < self.column_max,
            Doc::SoftLine(_) => column < self.column_max,
            _ => column <= self.column_max,
        }
    }
    fn can_fit_in_line_docs(&self, docs: &Docs) -> bool {
        let column = self.column + self.measure_docs(docs);
        match docs.iter().last() {
            Some(Doc::HardLine(_)) => column < self.column_max,
            Some(Doc::Line(_)) => column < self.column_max,
            Some(Doc::SoftLine(_)) => column < self.column_max,
            _ => column <= self.column_max,
        }
    }
    fn measure_doc(&self, doc: &Doc) -> usize {
        match doc {
            Doc::Fill(fill) => self.measure_docs(&fill.docs),
            Doc::Group(group) => self.measure_docs(&group.docs),
            Doc::HardLine(_hard_line) => 0,
            Doc::IfBreak(if_break) => self.measure_doc(&if_break.flat),
            Doc::Indent(indent) => self.measure_doc(&indent.doc),
            Doc::IndentIfBreak(indent_if_break) => self.measure_doc(&indent_if_break.doc),
            Doc::Line(_line) => 1,
            Doc::None(_none) => 0,
            Doc::Sequence(sequence) => self.measure_docs(&sequence.docs),
            Doc::SoftLine(_soft_line) => 0,
            Doc::Text(text) => text.text.len(),
        }
    }
    fn measure_docs(&self, docs: &Docs) -> usize {
        let mut total = 0;
        for doc in docs {
            total += self.measure_doc(doc);
        }
        total
    }
}
