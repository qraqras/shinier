use crate::doc::*;

// Generic, pure layout builders (no AST traversal).

// Join docs with a separator doc inserted between items.
pub fn join_with_sep(items: Vec<Doc>, sep: Doc) -> Docs {
    let mut out: Docs = Vec::with_capacity(items.len() * 2);
    let mut iter = items.into_iter();
    if let Some(first) = iter.next() {
        out.push(first);
        for it in iter {
            out.push(sep.clone());
            out.push(it);
        }
    }
    out
}

// A comma followed by a potential soft wrap between items: "," + softline
pub fn comma_softline_sep() -> Doc {
    sequence(vec![text(",".to_string()), softline()])
}

// Surround inner with literal left/right tokens without adding grouping by itself.
pub fn surround(left: &str, inner: Docs, right: &str) -> Doc {
    sequence(vec![
        text(left.to_string()),
        sequence(inner),
        text(right.to_string()),
    ])
}

// Parenthesize a list of items with comma+softline separation and indentation.
pub fn parens_comma_list(items: Vec<Doc>) -> Doc {
    if items.is_empty() {
        return sequence(vec![text("(".to_string()), text(")".to_string())]);
    }
    let body = sequence(join_with_sep(items, comma_softline_sep()));
    group(vec![
        text("(".to_string()),
        indent(vec![softline(), body]),
        softline(),
        text(")".to_string()),
    ])
}

// Brackets around a comma-separated list, for index access targets.
pub fn brackets_comma_list(items: Vec<Doc>) -> Doc {
    if items.is_empty() {
        return sequence(vec![text("[".to_string()), text("]".to_string())]);
    }
    let body = sequence(join_with_sep(items, comma_softline_sep()));
    group(vec![
        text("[".to_string()),
        indent(vec![softline(), body]),
        softline(),
        text("]".to_string()),
    ])
}

// Infix operator layout with soft break opportunity between operator and rhs.
pub fn infix(lhs: Doc, op: &str, rhs: Doc) -> Doc {
    group(vec![
        lhs,
        text(format!(" {}", op)),
        indent(vec![softline(), rhs]),
    ])
}

// Assignment-like layout: lhs <op> rhs
// Flat:  lhs <op> rhs
// Break: lhs <op>\n  rhs
pub fn build_assignment(lhs: Doc, op: &str, rhs: Doc) -> Doc {
    group(vec![
        lhs,
        text(format!(" {}", op)),
        indent(vec![softline(), rhs]),
    ])
}

// Method call layout: optional receiver + method name + parenthesized args
// - recv.method(args)
// - method(args) when recv is None
pub fn call_layout(recv: Option<Doc>, method: &str, args: Vec<Doc>) -> Doc {
    let name = text(method.to_string());
    let call = sequence(vec![name, parens_comma_list(args)]);
    match recv {
        Some(r) => sequence(vec![r, text(".".to_string()), call]),
        None => call,
    }
}

// Index access layout: recv[items]
pub fn index_layout(recv: Doc, items: Vec<Doc>) -> Doc {
    sequence(vec![recv, brackets_comma_list(items)])
}
