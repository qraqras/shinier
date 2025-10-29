use crate::document::Document;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug)]
struct Command<'a> {
    ind: i32,
    doc: &'a Document,
    mode: Mode,
    offset: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Flat,
    Break,
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

impl Document {
    fn as_cmd<'a>(&'a self, ind: i32, mode: Mode) -> Command<'a> {
        Command {
            ind,
            doc: self,
            mode,
            offset: 0,
        }
    }
    fn as_cmd_with_offset<'a>(&'a self, ind: i32, mode: Mode, offset: usize) -> Command<'a> {
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
    next: Command,
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
    let mut cmds: Vec<Command<'_>> = Vec::from(&[next]);
    while *width >= 0 {
        if cmds.is_empty() {
            if rest_idx == 0 {
                return true;
            }
            rest_idx -= 1;
            cmds.push(rest_commands[rest_idx]);
            continue;
        }
        let Command { ind, doc, mode, .. } = cmds.pop().unwrap();
        match doc {
            Document::Array(array) => {
                for part in array.iter().rev() {
                    cmds.push(part.as_cmd(ind, mode));
                }
            }
            Document::BreakParent => {}
            Document::Fill(fill) => {
                for part in fill.parts.iter().rev() {
                    cmds.push(part.as_cmd(ind, mode));
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
                cmds.push(contents.as_cmd(ind, mode));
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
                cmds.push(contents.as_cmd(ind, mode));
            }
            Document::Indent(indent) => {
                cmds.push(indent.contents.as_cmd(ind, mode));
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
    let mut cmds = Vec::from(&[doc.as_cmd(0, Mode::Break)]);
    let mut out = String::new();
    let mut should_remeasure = false;
    let mut line_suffixes = Vec::new();

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
            Document::Array(array) => {
                for doc in array.iter().rev() {
                    cmds.push(doc.as_cmd(ind, mode));
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
                let content_flat_cmd = content.as_cmd(ind, Mode::Flat);
                let content_break_cmd = content.as_cmd(ind, Mode::Break);
                let content_fits = fits(
                    content_flat_cmd,
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
                let whitespace_flat_cmd = whitespace.as_cmd(ind, Mode::Flat);
                let whitespace_break_cmd = whitespace.as_cmd(ind, Mode::Break);
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

                let remaining_cmd = doc.as_cmd_with_offset(ind, mode, offset + 2);
                let first_and_second_content_array = Document::Array(Vec::from([
                    content.clone(),
                    whitespace.clone(),
                    second_content.clone(),
                ]));
                let first_and_second_content_flat_cmd =
                    first_and_second_content_array.as_cmd(ind, Mode::Flat);
                let first_and_second_content_fits = fits(
                    first_and_second_content_flat_cmd,
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
                            cmds.push(group.contents.as_cmd(ind, effective_mode));
                        }
                    }
                    Mode::Break => {
                        should_remeasure = false;
                        let next = group.contents.as_cmd(ind, Mode::Flat);
                        let mut rem = width - pos;
                        let mut has_line_suffix = !line_suffixes.is_empty();
                        if effective_mode == Mode::Flat
                            && fits(
                                next,
                                &cmds,
                                &mut rem,
                                &mut has_line_suffix,
                                &group_mode_map,
                                false,
                            )
                        {
                            cmds.push(next);
                        } else {
                            if let Some(expanded_states) = &group.expanded_states {
                                let most_expanded = expanded_states.last().unwrap();
                                if effective_mode == Mode::Break {
                                    cmds.push(most_expanded.as_cmd(ind, Mode::Break));
                                } else {
                                    for (i, state) in expanded_states.iter().enumerate() {
                                        if i >= expanded_states.len() {
                                            cmds.push(most_expanded.as_cmd(ind, Mode::Break));
                                            break;
                                        } else {
                                            let cmd = state.as_cmd(ind, Mode::Flat);
                                            if fits(
                                                cmd,
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
                                cmds.push(group.contents.as_cmd(ind, Mode::Break));
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
                cmds.push(contents.as_cmd(ind, mode));
            }
            Document::Indent(indent) => {
                cmds.push(indent.contents.as_cmd(ind + 1, mode));
            }
            Document::Line(line) => match mode {
                Mode::Flat => {
                    if !line.hard {
                        if !line.soft {
                            out.push(' ');
                            pos += 1;
                        }
                    } else {
                        should_remeasure = true;
                        if line.literal {
                            out.push('\n');
                            pos = 0;
                        } else {
                            out.push('\n');
                            let indent_str = "  ".repeat(ind as usize);
                            out.push_str(&indent_str);
                            pos = indent_str.len() as i32;
                        }
                    }
                }
                Mode::Break => {
                    if !line_suffixes.is_empty() {
                        for line_suffix in line_suffixes.iter().rev() {
                            cmds.push(*line_suffix);
                        }
                        cmds.push(doc.as_cmd(ind, mode));
                        continue;
                    }
                    if line.literal {
                        out.push('\n');
                        pos = 0;
                    } else {
                        pos -= trim(&mut out);
                        out.push('\n');
                        let indent_str = "  ".repeat(ind as usize);
                        out.push_str(&indent_str);
                        pos = indent_str.len() as i32;
                    }
                }
            },
            Document::LineSuffix(line_suffix) => {
                line_suffixes.push(line_suffix.contents.as_cmd(ind, mode));
            }
            Document::LineSuffixBoundary(line_suffix_boundary) => {
                if !line_suffixes.is_empty() {
                    line_suffix_boundary.hardline.as_cmd(ind, mode);
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
                group_break_map.insert(group.id, Mode::from(group.r#break));
                if let Some(expanded_states) = &group.expanded_states {
                    for state in expanded_states {
                        visit(state, parent_stack, group_break_map, visited);
                    }
                } else {
                    visit(&group.contents, parent_stack, group_break_map, visited);
                }
                parent_stack.pop();
                if group.r#break {
                    if let Some(&parent_id) = parent_stack.last() {
                        group_break_map.insert(parent_id, Mode::Break);
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
