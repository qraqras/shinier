use crate::doc::{Doc, group, line, space, text};
use crate::keyword::{LogicalOperator, WRITE_OPERATOR};

pub fn build_write(name: Doc, value: Doc) -> Doc {
    group(&[name, space(), text(WRITE_OPERATOR), group(&[line(), value])])
}

pub fn build_logical_write(name: Doc, value: Doc, logical_operator: LogicalOperator) -> Doc {
    group(&[
        name,
        space(),
        text(logical_operator.as_str()),
        text(WRITE_OPERATOR),
        group(&[line(), value]),
    ])
}

pub fn build_operator_write(name: Doc, value: Doc, operator: Doc) -> Doc {
    group(&[
        name,
        space(),
        operator,
        text(WRITE_OPERATOR),
        group(&[line(), value]),
    ])
}
