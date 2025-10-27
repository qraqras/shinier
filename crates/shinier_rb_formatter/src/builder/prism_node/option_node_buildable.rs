use crate::document::Document;
use crate::{BuildContext, BuildPrismNode};
use ruby_prism::*;

impl BuildPrismNode for Option<AliasGlobalVariableNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<AliasMethodNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<AlternationPatternNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<AndNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ArgumentsNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ArrayNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ArrayPatternNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<AssocNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<AssocSplatNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<BackReferenceReadNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<BeginNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<BlockArgumentNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<BlockLocalVariableNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<BlockNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<BlockParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<BlockParametersNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<BreakNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<CallAndWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<CallNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<CallOperatorWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<CallOrWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<CallTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<CapturePatternNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<CaseMatchNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<CaseNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ClassNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ClassVariableAndWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ClassVariableOperatorWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ClassVariableOrWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ClassVariableReadNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ClassVariableTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ClassVariableWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantAndWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantOperatorWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantOrWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantPathAndWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantPathNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantPathOperatorWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantPathOrWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantPathTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantPathWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantReadNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ConstantWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<DefNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<DefinedNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ElseNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<EmbeddedStatementsNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<EmbeddedVariableNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<EnsureNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<FalseNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<FindPatternNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<FlipFlopNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<FloatNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ForNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ForwardingArgumentsNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ForwardingParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ForwardingSuperNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<GlobalVariableAndWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<GlobalVariableOperatorWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<GlobalVariableOrWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<GlobalVariableReadNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<GlobalVariableTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<GlobalVariableWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<HashNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<HashPatternNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<IfNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ImaginaryNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ImplicitNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ImplicitRestNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<IndexAndWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<IndexOperatorWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<IndexOrWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<IndexTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InstanceVariableAndWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InstanceVariableOperatorWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InstanceVariableOrWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InstanceVariableReadNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InstanceVariableTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InstanceVariableWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<IntegerNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InterpolatedMatchLastLineNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InterpolatedRegularExpressionNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InterpolatedStringNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InterpolatedSymbolNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<InterpolatedXStringNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ItLocalVariableReadNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ItParametersNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<KeywordHashNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<KeywordRestParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<LambdaNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<LocalVariableAndWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<LocalVariableOperatorWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<LocalVariableOrWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<LocalVariableReadNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<LocalVariableTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<LocalVariableWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<MatchLastLineNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<MatchPredicateNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<MatchRequiredNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<MatchWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<MissingNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ModuleNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<MultiTargetNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<MultiWriteNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<NextNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<NilNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<NoKeywordsParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<NumberedParametersNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<NumberedReferenceReadNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<OptionalKeywordParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<OptionalParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<OrNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ParametersNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ParenthesesNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<PinnedExpressionNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<PinnedVariableNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<PostExecutionNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<PreExecutionNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ProgramNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RangeNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RationalNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RedoNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RegularExpressionNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RequiredKeywordParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RequiredParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RescueModifierNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RescueNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RestParameterNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<RetryNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ReturnNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<SelfNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<ShareableConstantNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<SingletonClassNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<SourceEncodingNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<SourceFileNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<SourceLineNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<SplatNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<StatementsNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<StringNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<SuperNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<SymbolNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<TrueNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<UndefNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<UnlessNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<UntilNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<WhenNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<WhileNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<XStringNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
impl BuildPrismNode for Option<YieldNode<'_>> {
    #[rustfmt::skip]
    fn _build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.as_node()._build(context),
            None => Document::None,
        }
    }
}
