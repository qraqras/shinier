use crate::document::{Doc, Group, IfBreak, Indent, Line};
use std::cell::Cell;

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

pub fn string<T: Into<String>>(string: T) -> Doc {
    Doc::String(string.into())
}

pub fn array(contents: &[Doc]) -> Doc {
    Doc::Array(contents.to_vec())
}

pub fn break_parent() -> Doc {
    Doc::BreakParent
}

pub fn group(contents: Doc) -> Doc {
    Doc::Group(Group {
        id: generate_group_id(),
        contents: Box::new(contents),
        expanded_states: None,
        r#break: false,
    })
}

pub fn if_break(r#break: Doc, flat: Doc, group_id: Option<usize>) -> Doc {
    Doc::IfBreak(IfBreak {
        group_id,
        r#break: Box::new(r#break),
        flat: Box::new(flat),
    })
}

pub fn indent(contents: Doc) -> Doc {
    Doc::Indent(Indent {
        contents: Box::new(contents),
    })
}

pub fn line() -> Doc {
    Doc::Line(Line {
        hard: false,
        literal: false,
        soft: false,
    })
}

pub fn hardline() -> Doc {
    Doc::Line(Line {
        hard: true,
        literal: false,
        soft: false,
    })
}

pub fn literalline() -> Doc {
    Doc::Line(Line {
        hard: true,
        literal: true,
        soft: false,
    })
}

pub fn softline() -> Doc {
    Doc::Line(Line {
        hard: false,
        literal: false,
        soft: true,
    })
}
