use crate::utility::*;
use std::collections::HashMap;

#[derive(Clone)]
enum Doc {
    String(String),
    Fill(Fill),
    Indent(Indent),
    IndentIfBreak(IndentIfBreak),
    Group(Group),
    IfBreak(IfBreak),
    Line(Line),
}
#[derive(Clone)]
struct Fill {
    parts: Vec<Doc>,
}
#[derive(Clone)]
struct Indent {
    contents: Box<Doc>,
}
#[derive(Clone)]
struct IndentIfBreak {
    contents: Box<Doc>,
}
#[derive(Clone)]
struct Group {
    id: usize,
    r#break: bool,
    expanded_states: Vec<Doc>, // 少ない折り畳み方から試すためのリスト
}
#[derive(Clone)]
struct IfBreak {
    group_id: Option<usize>,
    flat_contents: Box<Doc>,
    break_contents: Box<Doc>,
}
#[derive(Clone)]
struct Line {
    hard: bool,
    soft: bool,
}

#[derive(Clone)]
enum Mode {
    Break,
    Flat,
}

struct Command {
    ind: usize,
    doc: Doc,
    mode: Mode,
}

fn fits(
    next: Command,
    rest_commands: &mut Vec<Command>,
    width: &mut usize,
    has_line_suffix: bool,
    group_mode_map: &HashMap<usize, Mode>,
    must_be_flat: bool,
) -> bool {
    if *width == usize::MAX {
        return true;
    }
    let mut cmds = vec![next];
    let mut out = vec![];
    while *width >= 0 {
        if cmds.is_empty() {
            if rest_commands.is_empty() {
                return true;
            }
            cmds.push(rest_commands.pop().unwrap());
            continue;
        }
        let Command { ind, doc, mode } = cmds.pop().unwrap();
        match doc {
            Doc::String(string) => {
                out.push(string.clone());
                *width -= get_string_width(string.as_str());
            }
            Doc::Fill(fill) => {
                // TODO: DOC_FILL_PRINTED_LENGTH
                for element in fill.parts.into_iter().rev() {
                    cmds.push(Command {
                        ind,
                        doc: element,
                        mode: mode.clone(),
                    });
                }
            }
            Doc::Indent(indent) => {
                cmds.push(Command {
                    ind: ind,
                    doc: *indent.contents,
                    mode: mode.clone(),
                });
            }
            Doc::IndentIfBreak(indent_if_break) => {
                cmds.push(Command {
                    ind: ind,
                    doc: *indent_if_break.contents,
                    mode: mode.clone(),
                });
            }
            Doc::Group(group) => {
                if must_be_flat && group.r#break {
                    return false;
                }
                let contents = match mode {
                    Mode::Flat => group.expanded_states.first().unwrap().clone(),
                    Mode::Break => group.expanded_states.last().unwrap().clone(),
                };
                cmds.push(Command {
                    ind,
                    doc: contents,
                    mode: mode.clone(),
                });
            }
            Doc::IfBreak(if_break) => {
                let group_mode = match if_break.group_id {
                    Some(group_id) => group_mode_map.get(&group_id).or(Some(&Mode::Flat)).unwrap(),
                    None => &mode,
                };
                let contents = match group_mode {
                    Mode::Flat => *if_break.flat_contents.clone(),
                    Mode::Break => *if_break.break_contents.clone(),
                };
                cmds.push(Command {
                    ind,
                    doc: contents,
                    mode: mode.clone(),
                });
            }
            Doc::Line(line) => {
                if matches!(mode, Mode::Break) || line.hard {
                    return true;
                }
                if !line.soft {
                    out.push(" ".to_string());
                    *width -= 1;
                }
            }
        }
    }

    false
}

fn print_doc_to_string(doc: &Doc, _options: ()) {
    let mut group_mod_map: HashMap<usize, Mode> = HashMap::new();
}
