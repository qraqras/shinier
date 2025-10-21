use crate::utility::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Flat,
    Break,
}

#[derive(Clone)]
enum Doc {
    String(String),
    Group(Group),
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
    rest_commands: &Vec<Command>,
    width: &mut i32,
    _has_line_suffix: bool,
    _group_mode_map: &HashMap<usize, Mode>,
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
            Doc::String(string) => {
                *width -= get_string_width(string.as_str()) as i32;
            }
            Doc::Group(group) => {
                if must_be_flat && group.mode == Mode::Break {
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
        }
    }
    false
}

fn print_doc_to_string(doc: &Doc, _options: ()) {
    let mut group_mod_map: HashMap<usize, Mode> = HashMap::new();
    let width = 80;
    let mut pos = 0;
    let mut cmds = vec![doc.as_cmd(0, Mode::Break)];
    let mut out = vec![];
    let mut should_remesure = false;

    //propagate_breaks(doc);

    while !cmds.is_empty() {
        let Command { ind, doc, mode } = cmds.pop().unwrap();
        match doc {
            Doc::String(string) => {
                out.push(string);
                pos += get_string_width(string.as_str()) as i32;
            }
            Doc::Group(group) => match mode {
                Mode::Flat => {
                    if !should_remesure {
                        cmds.push(group.contents().as_cmd(ind, group.mode));
                    }
                }
                Mode::Break => {
                    should_remesure = false;

                    let next = group
                        .expanded_states
                        .last()
                        .unwrap()
                        .as_cmd(ind, Mode::Flat);
                    let mut rem = width - pos;
                    let has_line_suffix = false; // !line_suffix.is_empty();
                    if group.mode == Mode::Flat
                        && fits(
                            next,
                            &cmds,
                            &mut rem,
                            has_line_suffix,
                            &group_mod_map,
                            false,
                        )
                    {
                        cmds.push(next);
                    } else {
                        if group.expanded_states.len() > 1 {
                            let most_expanded = group.expanded_states.last().unwrap();
                            if group.mode == Mode::Break {
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
                                            &group_mod_map,
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
                    group_mod_map.insert(group.id, cmds.last().unwrap().mode);
                }
            },
        }
    }
}
