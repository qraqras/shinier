use crate::builder::builder::*;
use crate::document::Document;
use crate::keyword::{LogicalOperator, WRITE_OPERATOR};

pub fn build_write(name: Document, value: Document) -> Document {
    group(array(&[
        name,
        space(),
        string(WRITE_OPERATOR),
        space(),
        value,
    ]))
}

pub fn build_logical_write(name: Document, value: Document, logical_operator: LogicalOperator) -> Document {
    group(array(&[
        name,
        space(),
        string(logical_operator.as_str()),
        string(WRITE_OPERATOR),
        group(array(&[line(), value])),
    ]))
}

pub fn build_operator_write(name: Document, value: Document, operator: Document) -> Document {
    group(array(&[
        name,
        space(),
        operator,
        string(WRITE_OPERATOR),
        group(array(&[line(), value])),
    ]))
}
