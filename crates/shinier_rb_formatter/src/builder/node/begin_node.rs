use crate::builder::build;
use crate::doc::*;
use ruby_prism::*;

const BEGIN_KEYWORD: &str = "begin";
const END_KEYWORD: &str = "end";

pub fn build_node(node: Option<&BeginNode>) -> Doc {
    let node = node.unwrap();
    let statements = node.statements();
    let rescue_clause = node.rescue_clause();
    let else_clause = node.else_clause();
    let ensure_clause = node.ensure_clause();

    let mut vec = Vec::new();
    if let Some(rescue_clause) = rescue_clause {
        vec.push(build(&rescue_clause.as_node()));
    }
    if let Some(else_clause) = else_clause {
        vec.push(build(&else_clause.as_node()));
    }
    if let Some(ensure_clause) = ensure_clause {
        vec.push(build(&ensure_clause.as_node()));
    }

    none() // TODO
}
