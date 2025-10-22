use crate::builder::builder::*;
use crate::document::Doc;
use crate::keyword::{LogicalOperator, WRITE_OPERATOR};

pub fn build_write(name: Doc, value: Doc) -> Doc {
    group(array(&[
        name,
        space(),
        string(WRITE_OPERATOR),
        group(array(&[line(), value])),
    ]))
}

pub fn build_logical_write(name: Doc, value: Doc, logical_operator: LogicalOperator) -> Doc {
    group(array(&[
        name,
        space(),
        string(logical_operator.as_str()),
        string(WRITE_OPERATOR),
        group(array(&[line(), value])),
    ]))
}

pub fn build_operator_write(name: Doc, value: Doc, operator: Doc) -> Doc {
    group(array(&[
        name,
        space(),
        operator,
        string(WRITE_OPERATOR),
        group(array(&[line(), value])),
    ]))
}
