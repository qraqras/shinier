use crate::Build;
use crate::BuildContext;
use crate::Document;
use crate::builder::prism::helper::leading_comments;
use crate::builder::prism::helper::leading_line_breaks;
use crate::builder::prism::helper::trailing_comments;
use ruby_prism::*;

pub trait NodeVariant<'sh>: Build {
    fn as_node(&self) -> Node<'sh>;
    fn location(&self) -> Location<'sh>;
    fn build(&self, context: &mut BuildContext) -> Document;
    fn execute_build(&self, context: &mut BuildContext) -> Document {
        let mut vec = Vec::new();
        // Build leading line breaks
        if context.leading_line_breaks {
            vec.push(leading_line_breaks(
                context,
                self.location().start_offset(),
                1,
            ));
        }
        // Build leading comments
        vec.push(leading_comments(&self.as_node(), context));
        let prev_is_statement = context.leading_line_breaks;
        context.leading_line_breaks = match self.as_node() {
            Node::StatementsNode { .. } => true,
            Node::ProgramNode { .. } => true,
            _ => false,
        };
        // Build the node itself
        vec.push(self.__build__(context));
        // Build trailing comments
        vec.push(trailing_comments(&self.as_node(), context));
        context.built_end = context.built_end.max(self.location().end_offset());
        context.leading_line_breaks = prev_is_statement;
        Document::Array(vec)
    }
}

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
            impl<'sh> NodeVariant<'sh> for Option<$typ<'sh>> {
                fn as_node(&self) -> Node<'sh> {
                    unimplemented!()
                }
                fn location(&self) -> Location<'sh> {
                    unimplemented!()
                }
                fn build(&self, context: &mut BuildContext) -> Document {
                    match self {
                        Some(node) => node.execute_build(context),
                        None => Document::None,
                    }
                }
            }
        )*
    };
}

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
