use crate::utility::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Flat,
    Break,
}

#[derive(Clone)]
enum Doc {
    BreakParent,
    Group(Group),
    String(String),
}

impl Doc {
    pub fn as_cmd(&self, ind: i32, mode: Mode) -> Command {
        Command {
            ind,
            doc: self,
            mode,
        }
    }
}

#[derive(Clone)]
struct Group {
    pub id: usize,
    pub expanded_states: Vec<Doc>,
    pub mode: Mode,
}

impl Group {
    fn contents(&self) -> &Doc {
        self.expanded_states
            .first()
            .expect("Group must have at least one expanded state")
    }
}

#[derive(Clone, Copy)]
struct Command<'a> {
    pub ind: i32,
    pub doc: &'a Doc,
    pub mode: Mode,
}

fn fits(
    next: Command,
    rest_commands: &[Command],
    width: &mut i32,
    _has_line_suffix: bool,
    group_mode_map: &HashMap<usize, Mode>,
    must_be_flat: bool,
) -> bool {
    if *width >= i32::MAX {
        return true;
    }
    let mut rest_idx = rest_commands.len();
    let mut cmds = vec![next];
    while *width >= 0 {
        if cmds.is_empty() {
            if rest_idx == 0 {
                return true;
            }
            rest_idx -= 1;
            cmds.push(rest_commands[rest_idx]);
            continue;
        }
        let Command { ind, doc, mode } = cmds.pop().unwrap();
        match doc {
            Doc::BreakParent => {}
            Doc::Group(group) => {
                let effective_mode = group_mode_map.get(&group.id).copied().unwrap_or(group.mode);
                if must_be_flat && effective_mode == Mode::Break {
                    return false;
                }
                let contents = match mode {
                    Mode::Flat => group.expanded_states.first(),
                    Mode::Break => group.expanded_states.last(),
                };
                if let Some(contents) = contents {
                    cmds.push(contents.as_cmd(ind, mode));
                }
            }
            Doc::String(string) => {
                *width -= get_string_width(string.as_str()) as i32;
            }
        }
    }
    false
}

fn print_doc_to_string(doc: &Doc, _options: ()) -> String {
    let width = 80;
    let mut pos = 0;
    let mut cmds = vec![doc.as_cmd(0, Mode::Break)];
    let mut out = String::new();
    let mut should_remeasure = false;

    let mut group_mode_map = propagate_breaks(doc);

    while let Some(Command { ind, doc, mode }) = cmds.pop() {
        match doc {
            Doc::BreakParent => {}
            Doc::Group(group) => {
                let effective_mode = group_mode_map.get(&group.id).copied().unwrap_or(group.mode);
                match mode {
                    Mode::Flat => {
                        if !should_remeasure {
                            cmds.push(group.contents().as_cmd(ind, effective_mode));
                        }
                    }
                    Mode::Break => {
                        should_remeasure = false;
                        let next = group
                            .expanded_states
                            .last()
                            .unwrap()
                            .as_cmd(ind, Mode::Flat);
                        let mut rem = width - pos;
                        let has_line_suffix = false;
                        if effective_mode == Mode::Flat
                            && fits(
                                next,
                                &cmds,
                                &mut rem,
                                has_line_suffix,
                                &group_mode_map,
                                false,
                            )
                        {
                            cmds.push(next);
                        } else {
                            if group.expanded_states.len() > 1 {
                                let most_expanded = group.expanded_states.last().unwrap();
                                if effective_mode == Mode::Break {
                                    cmds.push(most_expanded.as_cmd(ind, Mode::Break));
                                } else {
                                    for (i, state) in group.expanded_states.iter().enumerate() {
                                        if i >= group.expanded_states.len() - 1 {
                                            cmds.push(most_expanded.as_cmd(ind, Mode::Break));
                                            break;
                                        } else {
                                            let cmd = state.as_cmd(ind, Mode::Flat);
                                            if fits(
                                                cmd,
                                                &cmds,
                                                &mut rem,
                                                has_line_suffix,
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
                                cmds.push(
                                    group
                                        .expanded_states
                                        .first()
                                        .unwrap()
                                        .as_cmd(ind, Mode::Break),
                                );
                            }
                        }
                        group_mode_map.insert(group.id, cmds.last().unwrap().mode);
                    }
                }
            }
            Doc::String(string) => {
                out.push_str(string);
                pos += get_string_width(string.as_str()) as i32;
            }
        }
    }
    out
}

fn propagate_breaks(doc: &Doc) -> HashMap<usize, Mode> {
    let mut group_mode_map = HashMap::new();
    fn visit(
        doc: &Doc,
        parent_stack: &mut Vec<usize>,
        group_mode_map: &mut HashMap<usize, Mode>,
        visited: &mut HashSet<usize>,
    ) {
        match doc {
            Doc::BreakParent => {
                if let Some(&parent) = parent_stack.last() {
                    group_mode_map.insert(parent, Mode::Break);
                }
            }
            Doc::Group(group) => {
                if !visited.insert(group.id) {
                    return;
                }
                parent_stack.push(group.id);
                group_mode_map.insert(group.id, group.mode);
                for state in &group.expanded_states {
                    visit(state, parent_stack, group_mode_map, visited);
                }
                parent_stack.pop();
            }
            _ => {}
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
