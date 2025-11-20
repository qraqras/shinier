use crate::document::Document;
use crate::document::Group;
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
    queue: Vec<Rc<IndType>>,
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
        fn flush(ind: &Ind, value: &mut String, length: &mut usize, last_tabs: &mut usize, last_spaces: &mut usize) {
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
        let queue = match new_part {
            IndType::Dedent => {
                let mut new_queue = self.queue.clone();
                new_queue.pop();
                new_queue
            }
            _ => {
                let mut new_queue = self.queue.clone();
                new_queue.push(Rc::new(new_part));
                new_queue
            }
        };
        let estimated_size = queue.len() * self.tab_width + 32;
        let mut value = String::with_capacity(estimated_size);
        let mut length = 0usize;
        let mut last_tabs = 0usize;
        let mut last_spaces = 0usize;
        for part in queue.iter() {
            match part.as_ref() {
                IndType::AlignNumber(n) => {
                    last_tabs += 1;
                    last_spaces += n;
                }
                IndType::AlignString(s) => {
                    flush(self, &mut value, &mut length, &mut last_tabs, &mut last_spaces);
                    value.push_str(s);
                    length += s.len();
                }
                IndType::Indent => {
                    flush(self, &mut value, &mut length, &mut last_tabs, &mut last_spaces);
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
        flush_spaces(self, &mut value, &mut length, &mut last_tabs, &mut last_spaces);
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
                if must_be_flat && group.r#break {
                    return false;
                }
                let group_mode = Mode::from(group.r#break);
                let contents = if let Some(expanded_states) = &group.expanded_states
                    && group_mode == Mode::Break
                {
                    assert!(
                        !expanded_states.is_empty(),
                        "conditional_group requires at least one state"
                    );
                    expanded_states.last().unwrap()
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
                if let Some(contents) = contents {
                    cmds.push(contents.as_cmd(Rc::clone(&ind), mode));
                }
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

pub fn print_doc_to_string(doc: &mut Document, _options: ()) -> String {
    propagate_breaks(doc);
    let width = 40; // TODO: オプション化
    let mut pos = 0;
    let mut cmds = Vec::from(&[doc.as_cmd(Rc::new(Ind::root_indent(2)), Mode::Break)]);
    let mut out = String::new();
    let mut should_remeasure = false;
    let mut line_suffixes: Vec<Command<'_>> = Vec::new();

    let mut group_mode_map = HashMap::new();

    while let Some(Command { ind, doc, mode, offset }) = cmds.pop() {
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
                let first_and_second_content_array =
                    Document::Array(Vec::from([content.clone(), whitespace.clone(), second_content.clone()]));
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
                let mut fallthrough = true;
                if matches!(mode, Mode::Flat) {
                    if !should_remeasure {
                        cmds.push(group.contents.as_cmd(Rc::clone(&ind), Mode::from(group.r#break)));
                        fallthrough = false;
                    }
                }
                if matches!(mode, Mode::Break) || fallthrough {
                    should_remeasure = false;
                    let next = group.contents.as_cmd(Rc::clone(&ind), Mode::Flat);
                    let mut rem = width - pos;
                    let mut has_line_suffix = !line_suffixes.is_empty();
                    if Mode::from(group.r#break) == Mode::Flat
                        && fits(&next, &cmds, &mut rem, &mut has_line_suffix, &group_mode_map, false)
                    {
                        cmds.push(next.clone());
                    } else {
                        if let Some(expanded_states) = &group.expanded_states {
                            assert!(
                                !expanded_states.is_empty(),
                                "conditional_group requires at least one state"
                            );
                            let most_expanded = expanded_states.last().unwrap();
                            if Mode::from(group.r#break) == Mode::Break {
                                cmds.push(most_expanded.as_cmd(Rc::clone(&ind), Mode::Break));
                            } else {
                                for (i, state) in expanded_states.iter().enumerate() {
                                    if i >= expanded_states.len() - 1 {
                                        cmds.push(most_expanded.as_cmd(Rc::clone(&ind), Mode::Break));
                                        break;
                                    } else {
                                        let cmd = state.as_cmd(Rc::clone(&ind), Mode::Flat);
                                        if fits(&cmd, &cmds, &mut rem, &mut has_line_suffix, &group_mode_map, false) {
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
            Document::IfBreak(if_break) => {
                let group_mode = if_break
                    .group_id
                    .and_then(|id| group_mode_map.get(&id).copied())
                    .unwrap_or(mode);
                let contents = match group_mode {
                    Mode::Break => &if_break.r#break,
                    Mode::Flat => &if_break.flat,
                };
                if let Some(contents) = contents {
                    cmds.push(contents.as_cmd(Rc::clone(&ind), mode));
                }
            }
            Document::Indent(indent) => {
                cmds.push(indent.contents.as_cmd(Rc::new(ind.make_indent()), mode));
            }
            Document::Line(line) => {
                let mut fallthrough = true;
                if matches!(mode, Mode::Flat) {
                    if !line.hard {
                        if !line.soft {
                            out.push(' ');
                            pos += 1;
                        }
                        fallthrough = false;
                    } else {
                        should_remeasure = true;
                    }
                }
                if matches!(mode, Mode::Break) || fallthrough {
                    if !line_suffixes.is_empty() {
                        cmds.push(doc.as_cmd(Rc::clone(&ind), mode));
                        let pending = std::mem::take(&mut line_suffixes);
                        for line_suffix in pending.iter().rev() {
                            cmds.push(line_suffix.clone());
                        }
                        fallthrough = false;
                    }
                    if fallthrough {
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
                }
            }
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
        if cmds.len() == 0 && !line_suffixes.is_empty() {
            let pending = std::mem::take(&mut line_suffixes);
            for line_suffix in pending.iter().rev() {
                cmds.push(line_suffix.clone());
            }
        }
    }
    out.push('\n');
    out
}

fn break_parent_group(group_stack: &mut Vec<*mut Group>) {
    if let Some(&parent_group) = group_stack.last() {
        unsafe {
            if (*parent_group).expanded_states.is_none() && !(*parent_group).r#break {
                (*parent_group).r#break = true;
            }
        }
    }
}

fn propagate_breaks(doc: &mut Document) {
    let mut already_visited_set = HashSet::new();
    let mut group_stack = Vec::new();
    fn propagate_breaks_on_enter_fn(
        doc: &mut Document,
        already_visited_set: &mut HashSet<usize>,
        group_stack: &mut Vec<*mut Group>,
    ) -> bool {
        match doc {
            Document::BreakParent => break_parent_group(group_stack),
            Document::Group(group) => {
                let g: *mut Group = group;
                group_stack.push(g);
                if already_visited_set.contains(&group.id) {
                    return false;
                }
                already_visited_set.insert(group.id);
            }
            _ => {}
        };
        true
    }
    fn propagate_breaks_on_exit_fn(doc: &mut Document, group_stack: &mut Vec<*mut Group>) {
        match doc {
            Document::Group(_group) => {
                if let Some(group) = group_stack.pop() {
                    unsafe {
                        if (*group).r#break {
                            break_parent_group(group_stack);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    traverse_doc(
        doc,
        Some(propagate_breaks_on_enter_fn),
        Some(propagate_breaks_on_exit_fn),
        true,
        &mut already_visited_set,
        &mut group_stack,
    );
}
fn traverse_doc(
    doc: &mut Document,
    on_enter: Option<fn(&mut Document, &mut HashSet<usize>, &mut Vec<*mut Group>) -> bool>,
    on_exit: Option<fn(&mut Document, &mut Vec<*mut Group>)>,
    should_traverse_conditional_groups: bool,
    already_visited_set: &mut HashSet<usize>,
    group_stack: &mut Vec<*mut Group>,
) {
    if let Some(on_enter) = on_enter {
        if !on_enter(doc, already_visited_set, group_stack) {
            return;
        }
    }
    match doc {
        Document::Align(align) => {
            traverse_doc(
                &mut align.contents,
                on_enter,
                on_exit,
                should_traverse_conditional_groups,
                already_visited_set,
                group_stack,
            );
        }
        Document::Array(array) => {
            for part in array.iter_mut() {
                traverse_doc(
                    part,
                    on_enter,
                    on_exit,
                    should_traverse_conditional_groups,
                    already_visited_set,
                    group_stack,
                );
            }
        }
        Document::BreakParent => {}
        Document::Fill(fill) => {
            for part in fill.parts.iter_mut() {
                traverse_doc(
                    part,
                    on_enter,
                    on_exit,
                    should_traverse_conditional_groups,
                    already_visited_set,
                    group_stack,
                );
            }
        }
        Document::Group(group) => {
            if should_traverse_conditional_groups && let Some(ref mut expanded_states) = group.expanded_states {
                for state in expanded_states.iter_mut() {
                    traverse_doc(
                        state,
                        on_enter,
                        on_exit,
                        should_traverse_conditional_groups,
                        already_visited_set,
                        group_stack,
                    );
                }
            } else {
                traverse_doc(
                    &mut group.contents,
                    on_enter,
                    on_exit,
                    should_traverse_conditional_groups,
                    already_visited_set,
                    group_stack,
                );
            }
        }
        Document::IfBreak(if_break) => {
            if let Some(r#break) = &mut if_break.r#break {
                traverse_doc(
                    r#break,
                    on_enter,
                    on_exit,
                    should_traverse_conditional_groups,
                    already_visited_set,
                    group_stack,
                );
            }
            // traverse_doc(
            //     &mut if_break.r#break,
            //     on_enter,
            //     on_exit,
            //     should_traverse_conditional_groups,
            //     already_visited_set,
            //     group_stack,
            // );
            if let Some(flat) = &mut if_break.flat {
                traverse_doc(
                    flat,
                    on_enter,
                    on_exit,
                    should_traverse_conditional_groups,
                    already_visited_set,
                    group_stack,
                );
            }
            // traverse_doc(
            //     &mut if_break.flat,
            //     on_enter,
            //     on_exit,
            //     should_traverse_conditional_groups,
            //     already_visited_set,
            //     group_stack,
            // );
        }
        Document::Indent(indent) => {
            traverse_doc(
                &mut indent.contents,
                on_enter,
                on_exit,
                should_traverse_conditional_groups,
                already_visited_set,
                group_stack,
            );
        }
        Document::Line(_) => {}
        Document::LineSuffixBoundary(_) => {}
        Document::LineSuffix(line_suffix) => {
            traverse_doc(
                &mut line_suffix.contents,
                on_enter,
                on_exit,
                should_traverse_conditional_groups,
                already_visited_set,
                group_stack,
            );
        }
        Document::None => {}
        Document::String(_) => {}
    }
    if let Some(on_exit) = on_exit {
        on_exit(doc, group_stack);
    }
}
