use crate::document::Align;
use crate::document::Document;
use crate::document::Group;
use crate::document::IfBreak;
use crate::document::Indent;
use crate::document::Line;
use crate::document::LineSuffix;
use crate::document::LineSuffixBoundary;
use std::cell::Cell;

fn filter_none(docs: &[Option<Document>]) -> Vec<Document> {
    docs.iter().filter_map(|opt| opt.as_ref().cloned()).collect()
}

thread_local! {
    static GROUP_ID_NEXT: Cell<usize> = Cell::new(0);
}

fn generate_group_id() -> usize {
    GROUP_ID_NEXT.with(|next| {
        let id = next.get();
        next.set(id + 1);
        id
    })
}

pub fn reset_group_id() {
    GROUP_ID_NEXT.with(|next| next.set(0));
}

pub fn align_number(n: i32, content: Option<Document>) -> Option<Document> {
    match content {
        Some(content) => Some(Document::Align(Align {
            contents: Box::new(content),
            n: Some(n),
            s: None,
        })),
        None => None,
    }
}

pub fn align_string(s: String, content: Option<Document>) -> Option<Document> {
    match content {
        Some(content) => Some(Document::Align(Align {
            contents: Box::new(content),
            n: None,
            s: Some(s),
        })),
        None => None,
    }
}

pub fn array(parts: &[Option<Document>]) -> Option<Document> {
    let parts = filter_none(parts);
    match parts.is_empty() {
        true => None,
        false => Some(Document::Array(parts)),
    }
}

pub fn break_parent() -> Option<Document> {
    Some(Document::BreakParent)
}

pub fn fill(parts: Option<Document>) -> Option<Document> {
    match parts {
        Some(Document::Array(parts)) => Some(Document::Fill(crate::document::Fill { parts })),
        None => None,
        _ => panic!("fill function expects an Array document"),
    }
}

pub fn group(contents: Option<Document>) -> Option<Document> {
    match contents {
        Some(contents) => Some(Document::Group(Group {
            id: generate_group_id(),
            contents: Box::new(contents),
            r#break: false,
            expanded_states: None,
        })),
        None => None,
    }
}

pub fn conditional_group(states: &[Option<Document>]) -> Option<Document> {
    let states = filter_none(states);
    assert!(!states.is_empty(), "conditional_group requires at least one state");
    Some(Document::Group(Group {
        id: generate_group_id(),
        contents: Box::new(states.first().unwrap().clone()),
        r#break: false,
        expanded_states: Some(Vec::from(states)),
    }))
}

pub fn if_break(r#break: Option<Document>, flat: Option<Document>, group_id: Option<usize>) -> Option<Document> {
    Some(Document::IfBreak(IfBreak {
        group_id,
        r#break: r#break.map(|b| Box::new(b)),
        flat: flat.map(|f| Box::new(f)),
    }))
}

pub fn indent(contents: Option<Document>) -> Option<Document> {
    match contents {
        Some(contents) => Some(Document::Indent(Indent {
            contents: Box::new(contents),
        })),
        None => None,
    }
}

pub fn dedent(contents: Option<Document>) -> Option<Document> {
    align_number(-1, contents)
}

pub fn dedent_to_root(contents: Option<Document>) -> Option<Document> {
    align_number(i32::MIN, contents)
}

pub fn none() -> Option<Document> {
    None
}

pub fn line() -> Option<Document> {
    Some(Document::Line(Line {
        hard: false,
        literal: false,
        soft: false,
    }))
}

pub fn hardline() -> Option<Document> {
    array(&[
        Some(Document::Line(Line {
            hard: true,
            literal: false,
            soft: false,
        })),
        break_parent(),
    ])
}

pub fn literalline() -> Option<Document> {
    array(&[
        Some(Document::Line(Line {
            hard: true,
            literal: true,
            soft: false,
        })),
        break_parent(),
    ])
}

pub fn softline() -> Option<Document> {
    Some(Document::Line(Line {
        hard: false,
        literal: false,
        soft: true,
    }))
}

pub fn line_suffix(contents: Option<Document>) -> Option<Document> {
    match contents {
        Some(contents) => Some(Document::LineSuffix(LineSuffix {
            contents: Box::new(contents),
        })),
        None => None,
    }
}

pub fn line_suffix_boundary() -> Option<Document> {
    Some(Document::LineSuffixBoundary(LineSuffixBoundary {
        hardline: Box::new(hardline().unwrap()),
    }))
}

pub fn string<T: Into<String>>(string: T) -> Option<Document> {
    let string = string.into();
    match string.is_empty() {
        true => None,
        false => Some(Document::String(string.into())),
    }
}

pub fn comma() -> Option<Document> {
    string(",")
}

pub fn space() -> Option<Document> {
    string(" ")
}





pub fn hardline_without_break() -> Option<Document> {
    array(&[
        Some(Document::Line(Line {
            hard: true,
            literal: false,
            soft: false,
        })),
    ])
}
