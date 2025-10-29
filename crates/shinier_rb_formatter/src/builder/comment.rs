use crate::BuildContext;
use crate::Document;
use ruby_prism::*;

/*
AliasGlobalVariableNode
AliasMethodNode
AlternationPatternNode
AndNode
ArgumentsNode
ArrayNode
ArrayPatternNode
AssocNode
AssocSplatNode
BackReferenceReadNode
BeginNode
BlockArgumentNode
BlockLocalVariableNode
BlockNode
BlockParameterNode
BlockParametersNode
BreakNode
CallAndWriteNode
CallNode
CallOperatorWriteNode
CallOrWriteNode
CallTargetNode
CapturePatternNode
CaseMatchNode
CaseNode
ClassNode
ClassVariableAndWriteNode
ClassVariableOperatorWriteNode
ClassVariableOrWriteNode
ClassVariableReadNode
ClassVariableTargetNode
ClassVariableWriteNode
ConstantAndWriteNode
ConstantOperatorWriteNode
ConstantOrWriteNode
ConstantPathAndWriteNode
ConstantPathNode
ConstantPathOperatorWriteNode
ConstantPathOrWriteNode
ConstantPathTargetNode
ConstantPathWriteNode
ConstantReadNode
ConstantTargetNode
ConstantWriteNode
DefNode
DefinedNode
ElseNode
EmbeddedStatementsNode
EmbeddedVariableNode
EnsureNode
FalseNode
FindPatternNode
FlipFlopNode
FloatNode
ForNode
ForwardingArgumentsNode
ForwardingParameterNode
ForwardingSuperNode
GlobalVariableAndWriteNode
GlobalVariableOperatorWriteNode
GlobalVariableOrWriteNode
GlobalVariableReadNode
GlobalVariableTargetNode
GlobalVariableWriteNode
HashNode
HashPatternNode
IfNode
ImaginaryNode
ImplicitNode
ImplicitRestNode
InNode
IndexAndWriteNode
IndexOperatorWriteNode
IndexOrWriteNode
IndexTargetNode
InstanceVariableAndWriteNode
InstanceVariableOperatorWriteNode
InstanceVariableOrWriteNode
InstanceVariableReadNode
InstanceVariableTargetNode
InstanceVariableWriteNode
IntegerNode
InterpolatedMatchLastLineNode
InterpolatedRegularExpressionNode
InterpolatedStringNode
InterpolatedSymbolNode
InterpolatedXStringNode
ItLocalVariableReadNode
ItParametersNode
KeywordHashNode
KeywordRestParameterNode
LambdaNode
LocalVariableAndWriteNode
LocalVariableOperatorWriteNode
LocalVariableOrWriteNode
LocalVariableReadNode
LocalVariableTargetNode
LocalVariableWriteNode
MatchLastLineNode
MatchPredicateNode
MatchRequiredNode
MatchWriteNode
MissingNode
ModuleNode
MultiTargetNode
MultiWriteNode
NextNode
NilNode
NoKeywordsParameterNode
NumberedParametersNode
NumberedReferenceReadNode
OptionalKeywordParameterNode
OptionalParameterNode
OrNode
ParametersNode
ParenthesesNode
PinnedExpressionNode
PinnedVariableNode
PostExecutionNode
PreExecutionNode
ProgramNode
RangeNode
RationalNode
RedoNode
RegularExpressionNode
RequiredKeywordParameterNode
RequiredParameterNode
RescueModifierNode
RescueNode
RestParameterNode
RetryNode
ReturnNode
SelfNode
ShareableConstantNode
SingletonClassNode
SourceEncodingNode
SourceFileNode
SourceLineNode
SplatNode
StatementsNode
StringNode
SuperNode
SymbolNode
TrueNode
UndefNode
UnlessNode
UntilNode
WhenNode
WhileNode
XStringNode
YieldNode
*/

// pub trait NodeBuilder<T> {
//     fn _build(&self, node: Option<&T>, context: &mut BuildContext) -> Document;
// }

// pub struct AliasGlobalVariableNodeBuilder {}
// pub struct AliasMethodNodeBuilder {}
// pub struct AlternationPatternNodeBuilder {}
// pub struct AndNodeBuilder {}
// pub struct ArgumentsNodeBuilder {}
// pub struct ArrayNodeBuilder {}
// pub struct ArrayPatternNodeBuilder {}
// pub struct AssocNodeBuilder {}
// pub struct AssocSplatNodeBuilder {}
// pub struct BackReferenceReadNodeBuilder {}
// pub struct BeginNodeBuilder {}
// pub struct BlockArgumentNodeBuilder {}
// pub struct BlockLocalVariableNodeBuilder {}
// pub struct BlockNodeBuilder {}
// pub struct BlockParameterNodeBuilder {}
// pub struct BlockParametersNodeBuilder {}
// pub struct BreakNodeBuilder {}
// pub struct CallAndWriteNodeBuilder {}
// pub struct CallNodeBuilder {}
// pub struct CallOperatorWriteNodeBuilder {}
// pub struct CallOrWriteNodeBuilder {}
// pub struct CallTargetNodeBuilder {}
// pub struct CapturePatternNodeBuilder {}
// pub struct CaseMatchNodeBuilder {}
// pub struct CaseNodeBuilder {}
// pub struct ClassNodeBuilder {}
// pub struct ClassVariableAndWriteNodeBuilder {}
// pub struct ClassVariableOperatorWriteNodeBuilder {}
// pub struct ClassVariableOrWriteNodeBuilder {}
// pub struct ClassVariableReadNodeBuilder {}
// pub struct ClassVariableTargetNodeBuilder {}
// pub struct ClassVariableWriteNodeBuilder {}
// pub struct ConstantAndWriteNodeBuilder {}
// pub struct ConstantOperatorWriteNodeBuilder {}
// pub struct ConstantOrWriteNodeBuilder {}
// pub struct ConstantPathAndWriteNodeBuilder {}
// pub struct ConstantPathNodeBuilder {}
// pub struct ConstantPathOperatorWriteNodeBuilder {}
// pub struct ConstantPathOrWriteNodeBuilder {}
// pub struct ConstantPathTargetNodeBuilder {}
// pub struct ConstantPathWriteNodeBuilder {}
// pub struct ConstantReadNodeBuilder {}
// pub struct ConstantTargetNodeBuilder {}
// pub struct ConstantWriteNodeBuilder {}
// pub struct DefNodeBuilder {}
// pub struct DefinedNodeBuilder {}
// pub struct ElseNodeBuilder {}
// pub struct EmbeddedStatementsNodeBuilder {}
// pub struct EmbeddedVariableNodeBuilder {}
// pub struct EnsureNodeBuilder {}
// pub struct FalseNodeBuilder {}
// pub struct FindPatternNodeBuilder {}
// pub struct FlipFlopNodeBuilder {}
// pub struct FloatNodeBuilder {}
// pub struct ForNodeBuilder {}
// pub struct ForwardingArgumentsNodeBuilder {}
// pub struct ForwardingParameterNodeBuilder {}
// pub struct ForwardingSuperNodeBuilder {}
// pub struct GlobalVariableAndWriteNodeBuilder {}
// pub struct GlobalVariableOperatorWriteNodeBuilder {}
// pub struct GlobalVariableOrWriteNodeBuilder {}
// pub struct GlobalVariableReadNodeBuilder {}
// pub struct GlobalVariableTargetNodeBuilder {}
// pub struct GlobalVariableWriteNodeBuilder {}
// pub struct HashNodeBuilder {}
// pub struct HashPatternNodeBuilder {}
// pub struct IfNodeBuilder {}
// pub struct ImaginaryNodeBuilder {}
// pub struct ImplicitNodeBuilder {}
// pub struct ImplicitRestNodeBuilder {}
// pub struct InNodeBuilder {}
// pub struct IndexAndWriteNodeBuilder {}
// pub struct IndexOperatorWriteNodeBuilder {}
// pub struct IndexOrWriteNodeBuilder {}
// pub struct IndexTargetNodeBuilder {}
// pub struct InstanceVariableAndWriteNodeBuilder {}
// pub struct InstanceVariableOperatorWriteNodeBuilder {}
// pub struct InstanceVariableOrWriteNodeBuilder {}
// pub struct InstanceVariableReadNodeBuilder {}
// pub struct InstanceVariableTargetNodeBuilder {}
// pub struct InstanceVariableWriteNodeBuilder {}
// pub struct IntegerNodeBuilder {}
// pub struct InterpolatedMatchLastLineNodeBuilder {}
// pub struct InterpolatedRegularExpressionNodeBuilder {}
// pub struct InterpolatedStringNodeBuilder {}
// pub struct InterpolatedSymbolNodeBuilder {}
// pub struct InterpolatedXStringNodeBuilder {}
// pub struct ItLocalVariableReadNodeBuilder {}
// pub struct ItParametersNodeBuilder {}
// pub struct KeywordHashNodeBuilder {}
// pub struct KeywordRestParameterNodeBuilder {}
// pub struct LambdaNodeBuilder {}
// pub struct LocalVariableAndWriteNodeBuilder {}
// pub struct LocalVariableOperatorWriteNodeBuilder {}
// pub struct LocalVariableOrWriteNodeBuilder {}
// pub struct LocalVariableReadNodeBuilder {}
// pub struct LocalVariableTargetNodeBuilder {}
// pub struct LocalVariableWriteNodeBuilder {}
// pub struct MatchLastLineNodeBuilder {}
// pub struct MatchPredicateNodeBuilder {}
// pub struct MatchRequiredNodeBuilder {}
// pub struct MatchWriteNodeBuilder {}
// pub struct MissingNodeBuilder {}
// pub struct ModuleNodeBuilder {}
// pub struct MultiTargetNodeBuilder {}
// pub struct MultiWriteNodeBuilder {}
// pub struct NextNodeBuilder {}
// pub struct NilNodeBuilder {}
// pub struct NoKeywordsParameterNodeBuilder {}
// pub struct NumberedParametersNodeBuilder {}
// pub struct NumberedReferenceReadNodeBuilder {}
// pub struct OptionalKeywordParameterNodeBuilder {}
// pub struct OptionalParameterNodeBuilder {}
// pub struct OrNodeBuilder {}
// pub struct ParametersNodeBuilder {}
// pub struct ParenthesesNodeBuilder {}
// pub struct PinnedExpressionNodeBuilder {}
// pub struct PinnedVariableNodeBuilder {}
// pub struct PostExecutionNodeBuilder {}
// pub struct PreExecutionNodeBuilder {}
// pub struct ProgramNodeBuilder {}
// pub struct RangeNodeBuilder {}
// pub struct RationalNodeBuilder {}
// pub struct RedoNodeBuilder {}
// pub struct RegularExpressionNodeBuilder {}
// pub struct RequiredKeywordParameterNodeBuilder {}
// pub struct RequiredParameterNodeBuilder {}
// pub struct RescueModifierNodeBuilder {}
// pub struct RescueNodeBuilder {}
// pub struct RestParameterNodeBuilder {}
// pub struct RetryNodeBuilder {}
// pub struct ReturnNodeBuilder {}
// pub struct SelfNodeBuilder {}
// pub struct ShareableConstantNodeBuilder {}
// pub struct SingletonClassNodeBuilder {}
// pub struct SourceEncodingNodeBuilder {}
// pub struct SourceFileNodeBuilder {}
// pub struct SourceLineNodeBuilder {}
// pub struct SplatNodeBuilder {}
// pub struct StatementsNodeBuilder {}
// pub struct StringNodeBuilder {}
// pub struct SuperNodeBuilder {}
// pub struct SymbolNodeBuilder {}
// pub struct TrueNodeBuilder {}
// pub struct UndefNodeBuilder {}
// pub struct UnlessNodeBuilder {}
// pub struct UntilNodeBuilder {}
// pub struct WhenNodeBuilder {}
// pub struct WhileNodeBuilder {}
// pub struct XStringNodeBuilder {}
// pub struct YieldNodeBuilder {}

// #[rustfmt::skip]
// impl<'sh> NodeBuilder<AliasGlobalVariableNode<'sh>> for AliasGlobalVariableNodeBuilder {}
// impl<'sh> NodeBuilder<AliasMethodNode<'sh>> for AliasMethodNodeBuilder {}
// impl<'sh> NodeBuilder<AlternationPatternNode<'sh>> for AlternationPatternNodeBuilder {}
// impl<'sh> NodeBuilder<AndNode<'sh>> for AndNodeBuilder {}
// impl<'sh> NodeBuilder<ArgumentsNode<'sh>> for ArgumentsNodeBuilder {}
// impl<'sh> NodeBuilder<ArrayNode<'sh>> for ArrayNodeBuilder {}
// impl<'sh> NodeBuilder<ArrayPatternNode<'sh>> for ArrayPatternNodeBuilder {}
// impl<'sh> NodeBuilder<AssocNode<'sh>> for AssocNodeBuilder {}
// impl<'sh> NodeBuilder<AssocSplatNode<'sh>> for AssocSplatNodeBuilder {}
// impl<'sh> NodeBuilder<BackReferenceReadNode<'sh>> for BackReferenceReadNodeBuilder {}
// impl<'sh> NodeBuilder<BeginNode<'sh>> for BeginNodeBuilder {}
// impl<'sh> NodeBuilder<BlockArgumentNode<'sh>> for BlockArgumentNodeBuilder {}
// impl<'sh> NodeBuilder<BlockLocalVariableNode<'sh>> for BlockLocalVariableNodeBuilder {}
// impl<'sh> NodeBuilder<BlockNode<'sh>> for BlockNodeBuilder {}
// impl<'sh> NodeBuilder<BlockParameterNode<'sh>> for BlockParameterNodeBuilder {}
// impl<'sh> NodeBuilder<BlockParametersNode<'sh>> for BlockParametersNodeBuilder {}
// impl<'sh> NodeBuilder<BreakNode<'sh>> for BreakNodeBuilder {}
// impl<'sh> NodeBuilder<CallAndWriteNode<'sh>> for CallAndWriteNodeBuilder {}
// impl<'sh> NodeBuilder<CallNode<'sh>> for CallNodeBuilder {}
// impl<'sh> NodeBuilder<CallOperatorWriteNode<'sh>> for CallOperatorWriteNodeBuilder {}
// impl<'sh> NodeBuilder<CallOrWriteNode<'sh>> for CallOrWriteNodeBuilder {}
// impl<'sh> NodeBuilder<CallTargetNode<'sh>> for CallTargetNodeBuilder {}
// impl<'sh> NodeBuilder<CapturePatternNode<'sh>> for CapturePatternNodeBuilder {}
// impl<'sh> NodeBuilder<CaseMatchNode<'sh>> for CaseMatchNodeBuilder {}
// impl<'sh> NodeBuilder<CaseNode<'sh>> for CaseNodeBuilder {}
// impl<'sh> NodeBuilder<ClassNode<'sh>> for ClassNodeBuilder {}
// impl<'sh> NodeBuilder<ClassVariableAndWriteNode<'sh>> for ClassVariableAndWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ClassVariableOperatorWriteNode<'sh>>
//     for ClassVariableOperatorWriteNodeBuilder
// {
// }
// impl<'sh> NodeBuilder<ClassVariableOrWriteNode<'sh>> for ClassVariableOrWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ClassVariableReadNode<'sh>> for ClassVariableReadNodeBuilder {}
// impl<'sh> NodeBuilder<ClassVariableTargetNode<'sh>> for ClassVariableTargetNodeBuilder {}
// impl<'sh> NodeBuilder<ClassVariableWriteNode<'sh>> for ClassVariableWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantAndWriteNode<'sh>> for ConstantAndWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantOperatorWriteNode<'sh>> for ConstantOperatorWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantOrWriteNode<'sh>> for ConstantOrWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantPathAndWriteNode<'sh>> for ConstantPathAndWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantPathNode<'sh>> for ConstantPathNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantPathOperatorWriteNode<'sh>> for ConstantPathOperatorWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantPathOrWriteNode<'sh>> for ConstantPathOrWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantPathTargetNode<'sh>> for ConstantPathTargetNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantPathWriteNode<'sh>> for ConstantPathWriteNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantReadNode<'sh>> for ConstantReadNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantTargetNode<'sh>> for ConstantTargetNodeBuilder {}
// impl<'sh> NodeBuilder<ConstantWriteNode<'sh>> for ConstantWriteNodeBuilder {}
// impl<'sh> NodeBuilder<DefNode<'sh>> for DefNodeBuilder {}
// impl<'sh> NodeBuilder<DefinedNode<'sh>> for DefinedNodeBuilder {}
// impl<'sh> NodeBuilder<ElseNode<'sh>> for ElseNodeBuilder {}
// impl<'sh> NodeBuilder<EmbeddedStatementsNode<'sh>> for EmbeddedStatementsNodeBuilder {}
// impl<'sh> NodeBuilder<EmbeddedVariableNode<'sh>> for EmbeddedVariableNodeBuilder {}
// impl<'sh> NodeBuilder<EnsureNode<'sh>> for EnsureNodeBuilder {}
// impl<'sh> NodeBuilder<FalseNode<'sh>> for FalseNodeBuilder {}
// impl<'sh> NodeBuilder<FindPatternNode<'sh>> for FindPatternNodeBuilder {}
// impl<'sh> NodeBuilder<FlipFlopNode<'sh>> for FlipFlopNodeBuilder {}
// impl<'sh> NodeBuilder<FloatNode<'sh>> for FloatNodeBuilder {}
// impl<'sh> NodeBuilder<ForNode<'sh>> for ForNodeBuilder {}
// impl<'sh> NodeBuilder<ForwardingArgumentsNode<'sh>> for ForwardingArgumentsNodeBuilder {}
// impl<'sh> NodeBuilder<ForwardingParameterNode<'sh>> for ForwardingParameterNodeBuilder {}
// impl<'sh> NodeBuilder<ForwardingSuperNode<'sh>> for ForwardingSuperNodeBuilder {}
// impl<'sh> NodeBuilder<GlobalVariableAndWriteNode<'sh>> for GlobalVariableAndWriteNodeBuilder {}
// impl<'sh> NodeBuilder<GlobalVariableOperatorWriteNode<'sh>>
//     for GlobalVariableOperatorWriteNodeBuilder
// {
// }
// impl<'sh> NodeBuilder<GlobalVariableOrWriteNode<'sh>> for GlobalVariableOrWriteNodeBuilder {}
// impl<'sh> NodeBuilder<GlobalVariableReadNode<'sh>> for GlobalVariableReadNodeBuilder {}
// impl<'sh> NodeBuilder<GlobalVariableTargetNode<'sh>> for GlobalVariableTargetNodeBuilder {}
// impl<'sh> NodeBuilder<GlobalVariableWriteNode<'sh>> for GlobalVariableWriteNodeBuilder {}
// impl<'sh> NodeBuilder<HashNode<'sh>> for HashNodeBuilder {}
// impl<'sh> NodeBuilder<HashPatternNode<'sh>> for HashPatternNodeBuilder {}
// impl<'sh> NodeBuilder<IfNode<'sh>> for IfNodeBuilder {}
// impl<'sh> NodeBuilder<ImaginaryNode<'sh>> for ImaginaryNodeBuilder {}
// impl<'sh> NodeBuilder<ImplicitNode<'sh>> for ImplicitNodeBuilder {}
// impl<'sh> NodeBuilder<ImplicitRestNode<'sh>> for ImplicitRestNodeBuilder {}
// impl<'sh> NodeBuilder<InNode<'sh>> for InNodeBuilder {}
// impl<'sh> NodeBuilder<IndexAndWriteNode<'sh>> for IndexAndWriteNodeBuilder {}
// impl<'sh> NodeBuilder<IndexOperatorWriteNode<'sh>> for IndexOperatorWriteNodeBuilder {}
// impl<'sh> NodeBuilder<IndexOrWriteNode<'sh>> for IndexOrWriteNodeBuilder {}
// impl<'sh> NodeBuilder<IndexTargetNode<'sh>> for IndexTargetNodeBuilder {}
// impl<'sh> NodeBuilder<InstanceVariableAndWriteNode<'sh>> for InstanceVariableAndWriteNodeBuilder {}
// impl<'sh> NodeBuilder<InstanceVariableOperatorWriteNode<'sh>>
//     for InstanceVariableOperatorWriteNodeBuilder
// {
// }
// impl<'sh> NodeBuilder<InstanceVariableOrWriteNode<'sh>> for InstanceVariableOrWriteNodeBuilder {}
// impl<'sh> NodeBuilder<InstanceVariableReadNode<'sh>> for InstanceVariableReadNodeBuilder {}
// impl<'sh> NodeBuilder<InstanceVariableTargetNode<'sh>> for InstanceVariableTargetNodeBuilder {}
// impl<'sh> NodeBuilder<InstanceVariableWriteNode<'sh>> for InstanceVariableWriteNodeBuilder {}
// impl<'sh> NodeBuilder<IntegerNode<'sh>> for IntegerNodeBuilder {}
// impl<'sh> NodeBuilder<InterpolatedMatchLastLineNode<'sh>> for InterpolatedMatchLastLineNodeBuilder {}
// impl<'sh> NodeBuilder<InterpolatedRegularExpressionNode<'sh>>
//     for InterpolatedRegularExpressionNodeBuilder
// {
// }
// impl<'sh> NodeBuilder<InterpolatedStringNode<'sh>> for InterpolatedStringNodeBuilder {}
// impl<'sh> NodeBuilder<InterpolatedSymbolNode<'sh>> for InterpolatedSymbolNodeBuilder {}
// impl<'sh> NodeBuilder<InterpolatedXStringNode<'sh>> for InterpolatedXStringNodeBuilder {}
// impl<'sh> NodeBuilder<ItLocalVariableReadNode<'sh>> for ItLocalVariableReadNodeBuilder {}
// impl<'sh> NodeBuilder<ItParametersNode<'sh>> for ItParametersNodeBuilder {}
// impl<'sh> NodeBuilder<KeywordHashNode<'sh>> for KeywordHashNodeBuilder {}
// impl<'sh> NodeBuilder<KeywordRestParameterNode<'sh>> for KeywordRestParameterNodeBuilder {}
// impl<'sh> NodeBuilder<LambdaNode<'sh>> for LambdaNodeBuilder {}
// impl<'sh> NodeBuilder<LocalVariableAndWriteNode<'sh>> for LocalVariableAndWriteNodeBuilder {}
// impl<'sh> NodeBuilder<LocalVariableOperatorWriteNode<'sh>>
//     for LocalVariableOperatorWriteNodeBuilder
// {
// }
// impl<'sh> NodeBuilder<LocalVariableOrWriteNode<'sh>> for LocalVariableOrWriteNodeBuilder {}
// impl<'sh> NodeBuilder<LocalVariableReadNode<'sh>> for LocalVariableReadNodeBuilder {}
// impl<'sh> NodeBuilder<LocalVariableTargetNode<'sh>> for LocalVariableTargetNodeBuilder {}
// impl<'sh> NodeBuilder<LocalVariableWriteNode<'sh>> for LocalVariableWriteNodeBuilder {}
// impl<'sh> NodeBuilder<MatchLastLineNode<'sh>> for MatchLastLineNodeBuilder {}
// impl<'sh> NodeBuilder<MatchPredicateNode<'sh>> for MatchPredicateNodeBuilder {}
// impl<'sh> NodeBuilder<MatchRequiredNode<'sh>> for MatchRequiredNodeBuilder {}
// impl<'sh> NodeBuilder<MatchWriteNode<'sh>> for MatchWriteNodeBuilder {}
// impl<'sh> NodeBuilder<MissingNode<'sh>> for MissingNodeBuilder {}
// impl<'sh> NodeBuilder<ModuleNode<'sh>> for ModuleNodeBuilder {}
// impl<'sh> NodeBuilder<MultiTargetNode<'sh>> for MultiTargetNodeBuilder {}
// impl<'sh> NodeBuilder<MultiWriteNode<'sh>> for MultiWriteNodeBuilder {}
// impl<'sh> NodeBuilder<NextNode<'sh>> for NextNodeBuilder {}
// impl<'sh> NodeBuilder<NilNode<'sh>> for NilNodeBuilder {}
// impl<'sh> NodeBuilder<NoKeywordsParameterNode<'sh>> for NoKeywordsParameterNodeBuilder {}
// impl<'sh> NodeBuilder<NumberedParametersNode<'sh>> for NumberedParametersNodeBuilder {}
// impl<'sh> NodeBuilder<NumberedReferenceReadNode<'sh>> for NumberedReferenceReadNodeBuilder {}
// impl<'sh> NodeBuilder<OptionalKeywordParameterNode<'sh>> for OptionalKeywordParameterNodeBuilder {}
// impl<'sh> NodeBuilder<OptionalParameterNode<'sh>> for OptionalParameterNodeBuilder {}
// impl<'sh> NodeBuilder<OrNode<'sh>> for OrNodeBuilder {}
// impl<'sh> NodeBuilder<ParametersNode<'sh>> for ParametersNodeBuilder {}
// impl<'sh> NodeBuilder<ParenthesesNode<'sh>> for ParenthesesNodeBuilder {}
// impl<'sh> NodeBuilder<PinnedExpressionNode<'sh>> for PinnedExpressionNodeBuilder {}
// impl<'sh> NodeBuilder<PinnedVariableNode<'sh>> for PinnedVariableNodeBuilder {}
// impl<'sh> NodeBuilder<PostExecutionNode<'sh>> for PostExecutionNodeBuilder {}
// impl<'sh> NodeBuilder<PreExecutionNode<'sh>> for PreExecutionNodeBuilder {}
// impl<'sh> NodeBuilder<ProgramNode<'sh>> for ProgramNodeBuilder {}
// impl<'sh> NodeBuilder<RangeNode<'sh>> for RangeNodeBuilder {}
// impl<'sh> NodeBuilder<RationalNode<'sh>> for RationalNodeBuilder {}
// impl<'sh> NodeBuilder<RedoNode<'sh>> for RedoNodeBuilder {}
// impl<'sh> NodeBuilder<RegularExpressionNode<'sh>> for RegularExpressionNodeBuilder {}
// impl<'sh> NodeBuilder<RequiredKeywordParameterNode<'sh>> for RequiredKeywordParameterNodeBuilder {}
// impl<'sh> NodeBuilder<RequiredParameterNode<'sh>> for RequiredParameterNodeBuilder {}
// impl<'sh> NodeBuilder<RescueModifierNode<'sh>> for RescueModifierNodeBuilder {}
// impl<'sh> NodeBuilder<RescueNode<'sh>> for RescueNodeBuilder {}
// impl<'sh> NodeBuilder<RestParameterNode<'sh>> for RestParameterNodeBuilder {}
// impl<'sh> NodeBuilder<RetryNode<'sh>> for RetryNodeBuilder {}
// impl<'sh> NodeBuilder<ReturnNode<'sh>> for ReturnNodeBuilder {}
// impl<'sh> NodeBuilder<SelfNode<'sh>> for SelfNodeBuilder {}
// impl<'sh> NodeBuilder<ShareableConstantNode<'sh>> for ShareableConstantNodeBuilder {}
// impl<'sh> NodeBuilder<SingletonClassNode<'sh>> for SingletonClassNodeBuilder {}
// impl<'sh> NodeBuilder<SourceEncodingNode<'sh>> for SourceEncodingNodeBuilder {}
// impl<'sh> NodeBuilder<SourceFileNode<'sh>> for SourceFileNodeBuilder {}
// impl<'sh> NodeBuilder<SourceLineNode<'sh>> for SourceLineNodeBuilder {}
// impl<'sh> NodeBuilder<SplatNode<'sh>> for SplatNodeBuilder {}
// impl<'sh> NodeBuilder<StatementsNode<'sh>> for StatementsNodeBuilder {}
// impl<'sh> NodeBuilder<StringNode<'sh>> for StringNodeBuilder {}
// impl<'sh> NodeBuilder<SuperNode<'sh>> for SuperNodeBuilder {}
// impl<'sh> NodeBuilder<SymbolNode<'sh>> for SymbolNodeBuilder {}
// impl<'sh> NodeBuilder<TrueNode<'sh>> for TrueNodeBuilder {}
// impl<'sh> NodeBuilder<UndefNode<'sh>> for UndefNodeBuilder {}
// impl<'sh> NodeBuilder<UnlessNode<'sh>> for UnlessNodeBuilder {}
// impl<'sh> NodeBuilder<UntilNode<'sh>> for UntilNodeBuilder {}
// impl<'sh> NodeBuilder<WhenNode<'sh>> for WhenNodeBuilder {}
// impl<'sh> NodeBuilder<WhileNode<'sh>> for WhileNodeBuilder {}
// impl<'sh> NodeBuilder<XStringNode<'sh>> for XStringNodeBuilder {}
// impl<'sh> NodeBuilder<YieldNode<'sh>> for YieldNodeBuilder {}
