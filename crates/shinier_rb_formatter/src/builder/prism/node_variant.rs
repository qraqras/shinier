use crate::Build;
use crate::BuildContext;
use crate::Document;
use crate::builder::builder::{array, group, none};
use crate::builder::prism::helper::leading_comments;
use crate::builder::prism::helper::leading_line_breaks;
use crate::builder::prism::helper::trailing_comments;
use ruby_prism::*;

/// Trait for building node variants.
pub trait NodeVariant<'sh>: Build {
    fn as_node(&self) -> Node<'sh>;
    fn location(&self) -> Location<'sh>;
    fn build(&self, context: &mut BuildContext) -> Document;
    fn execute_build(&self, context: &mut BuildContext) -> Document {
        let mut vec = Vec::new();
        // Build leading comments
        if let Some(leading_comments) = leading_comments(&self.as_node(), context) {
            vec.push(leading_comments);
        }
        // Build leading line breaks
        if let Some(leading_line_breaks) = leading_line_breaks(
            context,
            self.location().start_offset(),
            context.max_leading_line_breaks,
        ) {
            vec.push(leading_line_breaks);
        }
        // propagate max leading line breaks for statements and program nodes
        let prev_max_leading_line_breaks = context.max_leading_line_breaks;
        context.max_leading_line_breaks = match self.as_node() {
            Node::StatementsNode { .. } => 1usize,
            Node::ProgramNode { .. } => 1usize,
            _ => 0usize,
        };
        // Build the node itself
        let node = self.__build__(context);
        // Build trailing comments
        if let Some(trailing_comments) = trailing_comments(&self.as_node(), context) {
            vec.push(group(array(&[node, trailing_comments])));
        } else {
            vec.push(node);
        }
        context.built_end = context.built_end.max(self.location().end_offset());
        context.max_leading_line_breaks = prev_max_leading_line_breaks;
        array(&vec)
    }
}

/// Macro to implement NodeVariant for multiple node types.
macro_rules! impl_node_variant {
    ($($typ:ident),* $(,)?) => {
        $(
            impl<'sh> NodeVariant<'sh> for $typ<'sh> {
                fn as_node(&self) -> Node<'sh> {
                    <$typ<'sh>>::as_node(self)
                }
                fn location(&self) -> Location<'sh> {
                    <$typ<'sh>>::location(self)
                }
                fn build(&self, context: &mut BuildContext) -> Document {
                    self.execute_build(context)
                }
            }
        )*
    };
}

// Implement NodeVariant for multiple node types
impl_node_variant!(
    AliasGlobalVariableNode,
    AliasMethodNode,
    AlternationPatternNode,
    AndNode,
    ArgumentsNode,
    ArrayNode,
    ArrayPatternNode,
    AssocNode,
    AssocSplatNode,
    BackReferenceReadNode,
    BeginNode,
    BlockArgumentNode,
    BlockLocalVariableNode,
    BlockNode,
    BlockParameterNode,
    BlockParametersNode,
    BreakNode,
    CallAndWriteNode,
    CallNode,
    CallOperatorWriteNode,
    CallOrWriteNode,
    CallTargetNode,
    CapturePatternNode,
    CaseMatchNode,
    CaseNode,
    ClassNode,
    ClassVariableAndWriteNode,
    ClassVariableOperatorWriteNode,
    ClassVariableOrWriteNode,
    ClassVariableReadNode,
    ClassVariableTargetNode,
    ClassVariableWriteNode,
    ConstantAndWriteNode,
    ConstantOperatorWriteNode,
    ConstantOrWriteNode,
    ConstantPathAndWriteNode,
    ConstantPathNode,
    ConstantPathOperatorWriteNode,
    ConstantPathOrWriteNode,
    ConstantPathTargetNode,
    ConstantPathWriteNode,
    ConstantReadNode,
    ConstantTargetNode,
    ConstantWriteNode,
    DefNode,
    DefinedNode,
    ElseNode,
    EmbeddedStatementsNode,
    EmbeddedVariableNode,
    EnsureNode,
    FalseNode,
    FindPatternNode,
    FlipFlopNode,
    FloatNode,
    ForNode,
    ForwardingArgumentsNode,
    ForwardingParameterNode,
    ForwardingSuperNode,
    GlobalVariableAndWriteNode,
    GlobalVariableOperatorWriteNode,
    GlobalVariableOrWriteNode,
    GlobalVariableReadNode,
    GlobalVariableTargetNode,
    GlobalVariableWriteNode,
    HashNode,
    HashPatternNode,
    IfNode,
    ImaginaryNode,
    ImplicitNode,
    ImplicitRestNode,
    InNode,
    IndexAndWriteNode,
    IndexOperatorWriteNode,
    IndexOrWriteNode,
    IndexTargetNode,
    InstanceVariableAndWriteNode,
    InstanceVariableOperatorWriteNode,
    InstanceVariableOrWriteNode,
    InstanceVariableReadNode,
    InstanceVariableTargetNode,
    InstanceVariableWriteNode,
    IntegerNode,
    InterpolatedMatchLastLineNode,
    InterpolatedRegularExpressionNode,
    InterpolatedStringNode,
    InterpolatedSymbolNode,
    InterpolatedXStringNode,
    ItLocalVariableReadNode,
    ItParametersNode,
    KeywordHashNode,
    KeywordRestParameterNode,
    LambdaNode,
    LocalVariableAndWriteNode,
    LocalVariableOperatorWriteNode,
    LocalVariableOrWriteNode,
    LocalVariableReadNode,
    LocalVariableTargetNode,
    LocalVariableWriteNode,
    MatchLastLineNode,
    MatchPredicateNode,
    MatchRequiredNode,
    MatchWriteNode,
    MissingNode,
    ModuleNode,
    MultiTargetNode,
    MultiWriteNode,
    NextNode,
    NilNode,
    NoKeywordsParameterNode,
    NumberedParametersNode,
    NumberedReferenceReadNode,
    OptionalKeywordParameterNode,
    OptionalParameterNode,
    OrNode,
    ParametersNode,
    ParenthesesNode,
    PinnedExpressionNode,
    PinnedVariableNode,
    PostExecutionNode,
    PreExecutionNode,
    ProgramNode,
    RangeNode,
    RationalNode,
    RedoNode,
    RegularExpressionNode,
    RequiredKeywordParameterNode,
    RequiredParameterNode,
    RescueModifierNode,
    RescueNode,
    RestParameterNode,
    RetryNode,
    ReturnNode,
    SelfNode,
    ShareableConstantNode,
    SingletonClassNode,
    SourceEncodingNode,
    SourceFileNode,
    SourceLineNode,
    SplatNode,
    StatementsNode,
    StringNode,
    SuperNode,
    SymbolNode,
    TrueNode,
    UndefNode,
    UnlessNode,
    UntilNode,
    WhenNode,
    WhileNode,
    XStringNode,
    YieldNode,
);

// Implement NodeVariant for Option<T>
impl<'sh, T: NodeVariant<'sh>> NodeVariant<'sh> for Option<T> {
    fn as_node(&self) -> Node<'sh> {
        unimplemented!("as_node is not implemented for Option<NodeVariant>")
    }
    fn location(&self) -> Location<'sh> {
        unimplemented!("location is not implemented for Option<NodeVariant>")
    }
    fn build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.execute_build(context),
            None => none(),
        }
    }
}
