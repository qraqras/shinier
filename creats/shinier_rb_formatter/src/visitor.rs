use std::{backtrace, io::Read};

use crate::doc::*;
use ruby_prism::*;

pub struct Visitor {
    pub docs: Docs,
}
impl Visitor {
    pub fn new() -> Self {
        Self { docs: vec![] }
    }
    fn utf8_to_str<'a>(&self, bytes: &'a [u8]) -> &'a str {
        std::str::from_utf8(bytes).unwrap()
    }
    fn utf8_to_string(&self, bytes: &[u8]) -> String {
        self.utf8_to_str(bytes).to_string()
    }
}
impl<'pr> Visit<'pr> for Visitor {
    fn visit_branch_node_enter(&mut self, _node: Node) {
        // println!("enter-b: {:?}", _node.location());
    }
    fn visit_leaf_node_enter(&mut self, _node: Node) {
        // println!("enter-l: {:?}", _node);
    }

    fn visit_local_variable_write_node(&mut self, node: &LocalVariableWriteNode) {
        visit_local_variable_write_node(self, node);
        let left: String = self.utf8_to_string(node.name().as_slice());
        let right: String = self.utf8_to_string(node.value().location().as_slice());
        self.docs.push(raw(left));
        self.docs.push(raw(" = ".to_string()));
        self.docs.push(raw(right));
        // println!("local_variable_write: {:?}", node)
    }
    fn visit_call_node(&mut self, node: &CallNode<'pr>) {
        if let Some(r) = node.receiver() {
            println!("call-r: {:?}", r.location());
        }
        if let Some(a) = node.arguments() {
            println!("call-a: {:?}", a.location());
        }
        if let Some(b) = node.block() {
            println!("call-b: {:?}", b.location());
        }
        println!("call-n: {:?}", self.utf8_to_str(node.name().as_slice()));

        // receiver.name
        //         ^
        if let Some(_) = node.call_operator_loc() {
            self.docs.push(raw(".".to_string()));
        }

        visit_call_node(self, node);
    }
}
