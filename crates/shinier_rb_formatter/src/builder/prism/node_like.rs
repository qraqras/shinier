// use crate::Build;
// use crate::BuildContext;
// use crate::Document;
// use ruby_prism::*;

// pub trait NodeLike<'sh>: Build {
//     fn location(&self) -> Location<'sh>;
//     fn build(&self, context: &mut crate::BuildContext) -> Document;
//     fn execute_build(&self, context: &mut BuildContext) -> Document {
//         let mut vec = Vec::new();
//         // Build leading line breaks
//         if context.leading_line_breaks {
//             if let Some(line_breaks) =
//                 build_leading_line_breaks(context, self.location().start_offset(), 1)
//             {
//                 vec.push(line_breaks);
//             }
//         }
//         // Build leading comments
//         if let Some(leading_comments) = build_leading_comments(&self.as_node(), context) {
//             vec.push(leading_comments);
//         }
//         let prev_is_statement = context.leading_line_breaks;
//         context.leading_line_breaks = match self.as_node() {
//             Node::StatementsNode { .. } => true,
//             Node::ProgramNode { .. } => true,
//             _ => false,
//         };
//         // Build the node itself
//         vec.push(self.__build__(context));
//         // Build trailing comments
//         if let Some(trailing_comments) = build_trailing_comments(&self.as_node(), context) {
//             vec.push(trailing_comments);
//         }
//         context.built_end = context.built_end.max(self.location().end_offset());
//         context.leading_line_breaks = prev_is_statement;
//         Document::Array(vec)
//     }
// }

// macro_rules! impl_node_like {
//     ($($typ:ident),* $(,)?) => {
//         $(
//             impl<'sh> NodeLike<'sh> for $typ<'sh> {
//                 fn location(&self) -> Location<'sh> {
//                     <$typ<'sh>>::location(self)
//                 }
//                 fn build(&self, context: &mut BuildContext) -> Document {
//                     self.execute_build(context)
//                 }
//             }
//             impl<'sh> NodeLike<'sh> for Option<$typ<'sh>> {
//                 fn location(&self) -> Location<'sh> {
//                     unimplemented!()
//                 }
//                 fn build(&self, context: &mut BuildContext) -> Document {
//                     match self {
//                         Some(node) => node.execute_build(context),
//                         None => Document::None,
//                     }
//                 }
//             }
//         )*
//     };
// }

// impl_node_like!(Comment, ConstantId, Integer);
