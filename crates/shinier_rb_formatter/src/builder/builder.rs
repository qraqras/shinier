use crate::document::{Doc, Group, IfBreak, Indent, Line};

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
        id: None,
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
