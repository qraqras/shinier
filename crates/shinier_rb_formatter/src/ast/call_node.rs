use ruby_prism::{CallNode, Visit};

pub fn s_visit_call_node<'pr, V>(visitor: &mut V, node: &CallNode<'pr>)
where
    V: Visit<'pr> + ?Sized,
{
    // self.frame.clear();
    // let recv = if let Some(node) = node.receiver() {
    //     self.visit(&node);
    //     self.frame.pop()
    // } else {
    //     None
    // };
    // self.frame.clear();
    // let args = if let Some(node) = node.arguments() {
    //     self.visit_arguments_node(&node);
    //     self.frame.pop()
    // } else {
    //     None
    // };
    // self.frame.clear();
    // let block = if let Some(node) = node.block() {
    //     self.visit(&node);
    //     self.frame.pop()
    // } else {
    //     None
    // };
    // self.push_frame(sequence(vec![
    //     recv.unwrap_or_default(),
    //     text(String::from_utf8_lossy(node.name().as_slice()).to_string()),
    //     text("(".to_string()),
    //     args.unwrap_or_default(),
    //     text(")".to_string()),
    //     block.unwrap_or_default(),
    // ]));
}
