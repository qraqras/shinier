use crate::document::Document;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Command<'sh> {
    ind: Rc<Ind>,
    doc: &'sh Document,
    mode: Mode,
    offset: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Flat,
    Break,
}

#[derive(Clone, Debug)]
struct Ind {
    value: String,
    length: usize,
    queue: Vec<IndType>,
    tab_width: usize,
    root: Option<Rc<Ind>>,
}

#[derive(Clone, Debug)]
enum IndType {
    AlignNumber(usize),
    AlignString(String),
    Dedent,
    Indent,
}

impl From<bool> for Mode {
    fn from(value: bool) -> Self {
        match value {
            true => Mode::Break,
            false => Mode::Flat,
        }
    }
}

impl From<Mode> for bool {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Break => true,
            Mode::Flat => false,
        }
    }
}

impl Ind {
    fn root_indent(tab_width: usize) -> Self {
        Self {
            value: String::new(),
            length: 0usize,
            queue: Vec::new(),
            tab_width,
            root: None,
        }
    }
    fn make_indent(&self) -> Self {
        self.generate_indent(IndType::Indent)
    }
    fn make_align_number(&self, width: i32) -> Self {
        if width == i32::MIN {
            if let Some(root) = &self.root {
                return (**root).clone();
            } else {
                return Ind::root_indent(self.tab_width);
            }
        }
        if width < 0 {
            return self.generate_indent(IndType::Dedent);
        }
        if width == 0 {
            return self.clone();
        }
        self.generate_indent(IndType::AlignNumber(width as usize))
    }
    fn make_align_string(&self, s: &String) -> Self {
        if s.is_empty() {
            return self.clone();
        }
        self.generate_indent(IndType::AlignString(s.clone()))
    }
    fn generate_indent(&self, new_part: IndType) -> Self {
        fn add_tabs(ind: &Ind, value: &mut String, length: &mut usize, count: usize) {
            *value += "\t".repeat(count).as_str();
            *length += ind.tab_width * count;
        }
        fn add_spaces(_ind: &Ind, value: &mut String, length: &mut usize, count: usize) {
            *value += " ".repeat(count).as_str();
            *length += count;
        }
        fn flush(
            ind: &Ind,
            value: &mut String,
            length: &mut usize,
            last_tabs: &mut usize,
            last_spaces: &mut usize,
        ) {
            const USE_TABS: bool = false; // TODO: オプション化
            match USE_TABS {
                true => flush_tabs(ind, value, length, last_tabs, last_spaces),
                false => flush_spaces(ind, value, length, last_tabs, last_spaces),
            }
        }
        fn flush_tabs(
            ind: &Ind,
            value: &mut String,
            length: &mut usize,
            last_tabs: &mut usize,
            last_spaces: &mut usize,
        ) {
            if *last_tabs > 0 {
                add_tabs(ind, value, length, *last_tabs);
            }
            reset_last(last_tabs, last_spaces);
        }
        fn flush_spaces(
            ind: &Ind,
            value: &mut String,
            length: &mut usize,
            last_tabs: &mut usize,
            last_spaces: &mut usize,
        ) {
            if *last_spaces > 0 {
                add_spaces(ind, value, length, *last_spaces);
            }
            reset_last(last_tabs, last_spaces);
        }
        fn reset_last(last_tabs: &mut usize, last_spaces: &mut usize) {
            *last_tabs = 0;
            *last_spaces = 0;
        }
        const USE_TABS: bool = false; // TODO: オプション化
        let mut queue = self.queue.clone();
        match new_part {
            IndType::Dedent => {
                queue.pop();
            }
            _ => {
                queue.push(new_part);
            }
        };
        let mut value = String::new();
        let mut length = 0usize;
        let mut last_tabs = 0usize;
        let mut last_spaces = 0usize;
        for part in &queue {
            match part {
                IndType::AlignNumber(n) => {
                    last_tabs += 1;
                    last_spaces += n;
                }
                IndType::AlignString(s) => {
                    flush(
                        self,
                        &mut value,
                        &mut length,
                        &mut last_tabs,
                        &mut last_spaces,
                    );
                    value += s;
                    length += s.len();
                }
                IndType::Indent => {
                    flush(
                        self,
                        &mut value,
                        &mut length,
                        &mut last_tabs,
                        &mut last_spaces,
                    );
                    match USE_TABS {
                        true => add_tabs(self, &mut value, &mut length, 1),
                        false => add_spaces(self, &mut value, &mut length, self.tab_width),
                    };
                }
                IndType::Dedent => {
                    unreachable!("Dedent should not be in the queue");
                }
            };
        }
        flush_spaces(
            self,
            &mut value,
            &mut length,
            &mut last_tabs,
            &mut last_spaces,
        );
        let root = match &self.root {
            Some(root) => Some(Rc::clone(root)),
            None => Some(Rc::new(self.clone())),
        };
        Self {
            value,
            length,
            queue,
            tab_width: self.tab_width,
            root,
        }
    }
}

impl Document {
    fn as_cmd<'sh>(&'sh self, ind: Rc<Ind>, mode: Mode) -> Command<'sh> {
        Command {
            ind,
            doc: self,
            mode,
            offset: 0,
        }
    }
    fn as_cmd_with_offset<'sh>(&'sh self, ind: Rc<Ind>, mode: Mode, offset: usize) -> Command<'sh> {
        Command {
            ind,
            doc: self,
            mode,
            offset,
        }
    }
}

pub fn get_string_width(string: &String) -> usize {
    string.chars().count()
}

pub fn trim(out: &mut String) -> i32 {
    let mut trim_count: i32 = 0;

    while let Some(char) = out.chars().last() {
        if [' ', '\t'].contains(&char) {
            out.pop();
            trim_count += 1;
        } else {
            break;
        }
    }
    trim_count
}

fn fits(
    next: &Command,
    rest_commands: &[Command],
    width: &mut i32,
    has_line_suffix: &mut bool,
    group_mode_map: &HashMap<usize, Mode>,
    must_be_flat: bool,
) -> bool {
    if *width >= i32::MAX {
        return true;
    }
    let mut rest_idx = rest_commands.len();
    let mut cmds: Vec<Command<'_>> = Vec::from(&[next.clone()]);
    while *width >= 0 {
        if cmds.is_empty() {
            if rest_idx == 0 {
                return true;
            }
            rest_idx -= 1;
            cmds.push(rest_commands[rest_idx].clone());
            continue;
        }
        let Command { ind, doc, mode, .. } = cmds.pop().unwrap();
        match doc {
            Document::Align(align) => {
                cmds.push(align.contents.as_cmd(Rc::clone(&ind), mode));
            }
            Document::Array(array) => {
                for part in array.iter().rev() {
                    cmds.push(part.as_cmd(Rc::clone(&ind), mode));
                }
            }
            Document::BreakParent => {}
            Document::Fill(fill) => {
                for part in fill.parts.iter().rev() {
                    cmds.push(part.as_cmd(Rc::clone(&ind), mode));
                }
            }
            Document::Group(group) => {
                // group.r#break は書き換わらないため group_mode_map を取得する必要があるのでは？
                if must_be_flat && group.r#break {
                    return false;
                }
                let group_mode = match group.r#break {
                    true => Mode::Break,
                    false => Mode::Flat,
                };
                let contents = if let Some(expanded_states) = &group.expanded_states
                    && group_mode == Mode::Break
                {
                    expanded_states.first().unwrap()
                } else {
                    &group.contents
                };
                cmds.push(contents.as_cmd(Rc::clone(&ind), mode));
            }
            Document::IfBreak(if_break) => {
                let group_mode = match if_break.group_id {
                    Some(group_id) => group_mode_map.get(&group_id).copied().unwrap_or(Mode::Flat),
                    None => mode,
                };
                let contents = match group_mode {
                    Mode::Break => &if_break.r#break,
                    Mode::Flat => &if_break.flat,
                };
                cmds.push(contents.as_cmd(Rc::clone(&ind), mode));
            }
            Document::Indent(indent) => {
                cmds.push(indent.contents.as_cmd(Rc::clone(&ind), mode));
            }
            Document::Line(line) => {
                if mode == Mode::Break || line.hard {
                    return true;
                }
                if !line.soft {
                    *width -= 1;
                }
            }
            Document::LineSuffix(_line_suffix) => {
                *has_line_suffix = true;
            }
            Document::LineSuffixBoundary(_line_suffix_boundary) => {
                if *has_line_suffix {
                    return false;
                }
            }
            Document::None => {}
            Document::String(string) => {
                *width -= get_string_width(string) as i32;
            }
        }
    }
    false
}

pub fn print_doc_to_string(doc: &Document, _options: ()) -> String {
    let width = 40; // TODO: オプション化
    let mut pos = 0;
    let mut cmds = Vec::from(&[doc.as_cmd(Rc::new(Ind::root_indent(2)), Mode::Break)]);
    let mut out = String::new();
    let mut should_remeasure = false;
    let mut line_suffixes: Vec<Command<'_>> = Vec::new();

    let mut group_mode_map = HashMap::new();

    let effective_group_mode_map = propagate_breaks(doc);

    while let Some(Command {
        ind,
        doc,
        mode,
        offset,
    }) = cmds.pop()
    {
        match &doc {
            Document::Align(align) => {
                let n = &align.n;
                let s = &align.s;
                let next_ind = match (n, s) {
                    (Some(n), None) => ind.make_align_number(*n),
                    (None, Some(s)) => ind.make_align_string(&s),
                    (None, None) => (*ind).clone(),
                    (_, _) => panic!("Both n and s cannot be set in Align at the same time"),
                };
                cmds.push(align.contents.as_cmd(Rc::new(next_ind), mode));
            }
            Document::Array(array) => {
                for doc in array.iter().rev() {
                    cmds.push(doc.as_cmd(Rc::clone(&ind), mode));
                }
            }
            Document::BreakParent => {}
            Document::Fill(fill) => {
                let mut rem = width - pos;
                let length = fill.parts.len() - offset;
                if length == 0 {
                    continue;
                }
                let content = fill.parts.get(offset + 0).unwrap();
                let content_flat_cmd = content.as_cmd(Rc::clone(&ind), Mode::Flat);
                let content_break_cmd = content.as_cmd(Rc::clone(&ind), Mode::Break);
                let content_fits = fits(
                    &content_flat_cmd,
                    &[],
                    &mut rem,
                    &mut !line_suffixes.is_empty(),
                    &group_mode_map,
                    true,
                );
                if length == 1 {
                    if content_fits {
                        cmds.push(content_flat_cmd);
                    } else {
                        cmds.push(content_break_cmd);
                    }
                    continue;
                }
                let whitespace = fill.parts.get(offset + 1).unwrap();
                let whitespace_flat_cmd = whitespace.as_cmd(Rc::clone(&ind), Mode::Flat);
                let whitespace_break_cmd = whitespace.as_cmd(Rc::clone(&ind), Mode::Break);
                if length == 2 {
                    if content_fits {
                        cmds.push(whitespace_flat_cmd);
                        cmds.push(content_flat_cmd);
                    } else {
                        cmds.push(whitespace_break_cmd);
                        cmds.push(content_break_cmd);
                    }
                    continue;
                }
                let second_content = fill.parts.get(offset + 2).unwrap();

                let remaining_cmd = doc.as_cmd_with_offset(Rc::clone(&ind), mode, offset + 2);
                let first_and_second_content_array = Document::Array(Vec::from([
                    content.clone(),
                    whitespace.clone(),
                    second_content.clone(),
                ]));
                let first_and_second_content_flat_cmd =
                    first_and_second_content_array.as_cmd(Rc::clone(&ind), Mode::Flat);
                let first_and_second_content_fits = fits(
                    &first_and_second_content_flat_cmd,
                    &[],
                    &mut rem,
                    &mut !line_suffixes.is_empty(),
                    &group_mode_map,
                    true,
                );
                if first_and_second_content_fits {
                    cmds.push(remaining_cmd.clone());
                    cmds.push(whitespace_flat_cmd);
                    cmds.push(content_flat_cmd);
                } else if content_fits {
                    cmds.push(remaining_cmd.clone());
                    cmds.push(whitespace_break_cmd);
                    cmds.push(content_flat_cmd);
                } else {
                    cmds.push(remaining_cmd.clone());
                    cmds.push(whitespace_break_cmd);
                    cmds.push(content_break_cmd);
                }
            }
            Document::Group(group) => {
                let effective_mode = effective_group_mode_map
                    .get(&group.id)
                    .copied()
                    .unwrap_or(Mode::from(group.r#break));
                match mode {
                    Mode::Flat => {
                        if !should_remeasure {
                            cmds.push(group.contents.as_cmd(Rc::clone(&ind), effective_mode));
                        }
                    }
                    Mode::Break => {
                        should_remeasure = false;
                        let next = group.contents.as_cmd(Rc::clone(&ind), Mode::Flat);
                        let mut rem = width - pos;
                        let mut has_line_suffix = !line_suffixes.is_empty();
                        if effective_mode == Mode::Flat
                            && fits(
                                &next,
                                &cmds,
                                &mut rem,
                                &mut has_line_suffix,
                                &group_mode_map,
                                false,
                            )
                        {
                            cmds.push(next.clone());
                        } else {
                            if let Some(expanded_states) = &group.expanded_states {
                                let most_expanded = expanded_states.last().unwrap();
                                if effective_mode == Mode::Break {
                                    cmds.push(most_expanded.as_cmd(Rc::clone(&ind), Mode::Break));
                                } else {
                                    for (i, state) in expanded_states.iter().enumerate() {
                                        if i >= expanded_states.len() {
                                            cmds.push(
                                                most_expanded.as_cmd(Rc::clone(&ind), Mode::Break),
                                            );
                                            break;
                                        } else {
                                            let cmd = state.as_cmd(Rc::clone(&ind), Mode::Flat);
                                            if fits(
                                                &cmd,
                                                &cmds,
                                                &mut rem,
                                                &mut has_line_suffix,
                                                &group_mode_map,
                                                false,
                                            ) {
                                                cmds.push(cmd);
                                                break;
                                            }
                                        }
                                    }
                                }
                            } else {
                                cmds.push(group.contents.as_cmd(Rc::clone(&ind), Mode::Break));
                            }
                        }
                        group_mode_map.insert(group.id, cmds.last().unwrap().mode);
                    }
                }
            }
            Document::IfBreak(if_break) => {
                let group_mode = if_break
                    .group_id
                    .and_then(|id| group_mode_map.get(&id).copied())
                    .unwrap_or(mode);
                let contents = match group_mode {
                    Mode::Break => &if_break.r#break,
                    Mode::Flat => &if_break.flat,
                };
                cmds.push(contents.as_cmd(Rc::clone(&ind), mode));
            }
            Document::Indent(indent) => {
                cmds.push(indent.contents.as_cmd(Rc::new(ind.make_indent()), mode));
            }
            Document::Line(line) => match mode {
                Mode::Flat => {
                    if !line.hard {
                        if !line.soft {
                            out.push(' ');
                            pos += 1;
                        }
                        continue;
                    } else {
                        should_remeasure = true;
                    }
                    // TODO: Mode::Breakの処理と同じ(fallthrough)
                    if !line_suffixes.is_empty() {
                        cmds.push(doc.as_cmd(Rc::clone(&ind), mode));
                        let pending = std::mem::take(&mut line_suffixes);
                        for line_suffix in pending.iter().rev() {
                            cmds.push(line_suffix.clone());
                        }
                        continue;
                    }
                    if line.literal {
                        if let Some(root) = &ind.root {
                            out.push('\n');
                            out.push_str(&root.value);
                            pos = root.length as i32;
                        } else {
                            out.push('\n');
                            pos = 0;
                        }
                    } else {
                        pos -= trim(&mut out);
                        out.push('\n');
                        out.push_str(&ind.value);
                        pos = ind.length as i32;
                    }
                }
                Mode::Break => {
                    if !line_suffixes.is_empty() {
                        cmds.push(doc.as_cmd(Rc::clone(&ind), mode));
                        let pending = std::mem::take(&mut line_suffixes);
                        for line_suffix in pending.iter().rev() {
                            cmds.push(line_suffix.clone());
                        }
                        continue;
                    }
                    if line.literal {
                        if let Some(root) = &ind.root {
                            out.push('\n');
                            out.push_str(&root.value);
                            pos = root.length as i32;
                        } else {
                            out.push('\n');
                            pos = 0;
                        }
                    } else {
                        pos -= trim(&mut out).max(0);
                        out.push('\n');
                        out.push_str(&ind.value);
                        pos = ind.length as i32;
                    }
                }
            },
            Document::LineSuffix(line_suffix) => {
                line_suffixes.push(line_suffix.contents.as_cmd(Rc::clone(&ind), mode));
            }
            Document::LineSuffixBoundary(line_suffix_boundary) => {
                if !line_suffixes.is_empty() {
                    cmds.push(line_suffix_boundary.hardline.as_cmd(Rc::clone(&ind), mode));
                }
            }
            Document::None => {}
            Document::String(string) => {
                out.push_str(string);
                pos += get_string_width(string) as i32;
            }
        }
    }
    out
}

fn propagate_breaks(doc: &Document) -> HashMap<usize, Mode> {
    let mut group_mode_map = HashMap::new();
    fn visit(
        doc: &Document,
        parent_stack: &mut Vec<usize>,
        group_break_map: &mut HashMap<usize, Mode>,
        visited: &mut HashSet<usize>,
    ) {
        match doc {
            Document::Align(align) => {
                visit(&align.contents, parent_stack, group_break_map, visited);
            }
            Document::Array(array) => {
                for part in array {
                    visit(part, parent_stack, group_break_map, visited);
                }
            }
            Document::BreakParent => {
                if let Some(&parent) = parent_stack.last() {
                    group_break_map.insert(parent, Mode::Break);
                }
            }
            Document::Fill(fill) => {
                for part in &fill.parts {
                    visit(part, parent_stack, group_break_map, visited);
                }
            }
            Document::Group(group) => {
                if !visited.insert(group.id) {
                    return;
                }
                parent_stack.push(group.id);
                group_break_map
                    .entry(group.id)
                    .or_insert_with(|| Mode::from(group.r#break));
                if let Some(expanded_states) = &group.expanded_states {
                    for state in expanded_states {
                        visit(state, parent_stack, group_break_map, visited);
                    }
                } else {
                    visit(&group.contents, parent_stack, group_break_map, visited);
                }
                parent_stack.pop();
                if group.propagate_break {
                    if group_break_map.get(&group.id) == Some(&Mode::Break) {
                        if let Some(&parent_id) = parent_stack.last() {
                            group_break_map.insert(parent_id, Mode::Break);
                        }
                    }
                }
            }
            Document::IfBreak(if_break) => {
                visit(&if_break.r#break, parent_stack, group_break_map, visited);
                visit(&if_break.flat, parent_stack, group_break_map, visited);
            }
            Document::Indent(indent) => {
                visit(&indent.contents, parent_stack, group_break_map, visited);
            }
            Document::Line(_line) => {}
            Document::LineSuffix(line_suffix) => {
                visit(
                    &line_suffix.contents,
                    parent_stack,
                    group_break_map,
                    visited,
                );
            }
            Document::LineSuffixBoundary(_line_suffix_boundary) => {}
            Document::None => {}
            Document::String(_string) => {}
        }
    }
    visit(
        doc,
        &mut Vec::new(),
        &mut group_mode_map,
        &mut HashSet::new(),
    );
    group_mode_map
}
