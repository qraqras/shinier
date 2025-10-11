use crate::doc::{Doc, group, line, space, text};
use crate::keyword::WRITE_OPERATOR;

pub fn build_write_pattern<'sh, T: WriteNodeTrait<'sh>>(node: &T) -> Doc {
    let name = node.name();
    let value = node.value();
    group(&[name, space(), text(WRITE_OPERATOR), line(), value])
}

pub fn build_logical_write_pattern<'sh, T: LogicalWriteNodeTrait<'sh>>(node: &T) -> Doc {
    let name = node.name();
    let value = node.value();
    let operator = node.logical_operator();
    group(&[name, space(), operator, text(WRITE_OPERATOR), line(), value])
}

pub fn build_operator_write_pattern<'sh, T: OperatorWriteNodeTrait<'sh>>(node: &T) -> Doc {
    let name = node.name();
    let value = node.value();
    let operator = node.binary_operator();
    group(&[name, space(), operator, text(WRITE_OPERATOR), line(), value])
}

pub trait WriteNodeTrait<'sh> {
    fn name(&self) -> Doc;
    fn value(&self) -> Doc;
}

pub trait LogicalWriteNodeTrait<'sh>: WriteNodeTrait<'sh> {
    const AND_OPERATOR: &'sh str = "&&";
    const OR_OPERATOR: &'sh str = "||";
    fn logical_operator(&self) -> Doc;
}

pub trait OperatorWriteNodeTrait<'sh>: WriteNodeTrait<'sh> {
    fn binary_operator(&self) -> Doc;
}
