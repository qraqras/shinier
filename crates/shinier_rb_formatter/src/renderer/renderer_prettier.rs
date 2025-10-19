use crate::utility::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum Doc {
    String(String),
    Fill(Fill),
    Indent(Indent),
    IndentIfBreak(IndentIfBreak),
    Group(Group),
    IfBreak(IfBreak),
    Line(Line),
    BreakParent,
    TraverseDocOnExitStackMarker,
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
    r#break: GroupBreakState,
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
enum GroupBreakState {
    True,
    False,
    Propagate,
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
                if must_be_flat && matches!(group.r#break, GroupBreakState::True) {
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
            _ => {
                return false;
            }
        }
    }

    false
}

fn print_doc_to_string(doc: &Doc, _options: ()) {
    let mut group_mod_map: HashMap<usize, Mode> = HashMap::new();
    let width = 80;
    let new_line = "\n";
    let mut pos = 0;
    let mut cmds = vec![Command {
        ind: 0,
        mode: Mode::Break,
        doc: doc.clone(),
    }];
    // const out = vec![];
    // let should_mesuere = false;
    // const mut line_suffixes = vec![];
    // let printed_cursor_count = 0;

    propagate_breaks(doc);

    while !cmds.is_empty() {
        let Command { ind, doc, mode } = cmds.pop().unwrap();
        match doc {
            // TODO:
            _ => (),
        }
    }
}

fn break_parent_group(group_stack: &RefCell<Vec<Group>>) {
    let mut group_stack = group_stack.borrow_mut();
    if !group_stack.is_empty() {
        let parent_group = group_stack.last_mut().unwrap();
        if !matches!(parent_group.r#break, GroupBreakState::False) {
            parent_group.r#break = GroupBreakState::Propagate;
        }
    }
}

fn propagate_breaks(doc: &Doc) {
    let mut already_visited_set: HashSet<usize> = HashSet::new();
    let group_stack = RefCell::new(Vec::<Group>::new());
    let propagate_breaks_on_enter_fn = |doc: &Doc| {
        if matches!(doc, Doc::BreakParent) {
            break_parent_group(&group_stack);
        }
        if let Doc::Group(group) = doc {
            group_stack.borrow_mut().push(group.clone());
            if already_visited_set.contains(&group.id) {
                return false;
            }
            already_visited_set.insert(group.id);
        }
        true
    };
    let propagate_breaks_on_exit_fn = |doc: &Doc| {
        if let Doc::Group(_group) = doc {
            group_stack.borrow_mut().pop();
        }
        true
    };
    traverse_docs(
        doc,
        Some(propagate_breaks_on_enter_fn),
        Some(propagate_breaks_on_exit_fn),
        true,
    );
}

fn traverse_docs(
    doc: &Doc,
    mut on_enter: Option<impl FnMut(&Doc) -> bool>,
    mut on_exit: Option<impl FnMut(&Doc) -> bool>,
    should_traverse_conditional_groups: bool,
) {
    let mut docs_stack = vec![doc];
    while !docs_stack.is_empty() {
        let doc = docs_stack.pop().unwrap();
        if let Some(on_exit_fn) = on_exit.as_mut() {
            if matches!(doc, Doc::TraverseDocOnExitStackMarker) {
                on_exit_fn(docs_stack.pop().unwrap());
                continue;
            }
            docs_stack.push(doc);
            docs_stack.push(&Doc::TraverseDocOnExitStackMarker);
        }
        if let Some(on_enter) = on_enter.as_mut() {
            if !on_enter(doc) {
                continue;
            }
        }
        match doc {
            Doc::Fill(fill) => {
                for part in fill.parts.iter().rev() {
                    docs_stack.push(part);
                }
                break;
            }
            Doc::IfBreak(if_break) => {
                docs_stack.push(&if_break.flat_contents);
                docs_stack.push(&if_break.break_contents);
            }
            Doc::Group(group) => {
                if should_traverse_conditional_groups {
                    for state in group.expanded_states.iter().rev() {
                        docs_stack.push(state);
                    }
                } else {
                    docs_stack.push(&group.expanded_states.first().unwrap());
                }
                break;
            }
            Doc::Indent(indent) => {
                docs_stack.push(&indent.contents);
            }
            Doc::IndentIfBreak(indent_if_break) => {
                docs_stack.push(&indent_if_break.contents);
            }
            _ => {
                break;
            }
        }
    }
}
