use crate::Document;

pub struct LayoutParamAliasGlobalVariableNode {
    pub new_name: Document,
    pub old_name: Document,
}
pub struct LayoutParamAliasMethodNode {
    pub new_name: Document,
    pub old_name: Document,
}
pub struct LayoutParamAlternationPatternNode {
    pub left: Document,
    pub right: Document,
}
pub struct LayoutParamAndNode {
    pub left: Document,
    pub right: Document,
}
pub struct LayoutParamArgumentsNode {
    pub arguments: Vec<Document>,
}
pub struct LayoutParamArrayNode {
    pub elements: Vec<Document>,
}
pub struct LayoutParamArrayPatternNode {
    pub constant: Option<Document>,
    pub requireds: Vec<Document>,
    pub rest: Option<Document>,
    pub posts: Vec<Document>,
}
pub struct LayoutParamAssocNode {
    pub key: Document,
    pub value: Document,
}
pub struct LayoutParamAssocSplatNode {
    pub value: Option<Document>,
}
pub struct LayoutParamBackReferenceReadNode {
    pub name: Document,
}
pub struct LayoutParamBeginNode {
    pub statements: Option<Document>,
    pub rescue_clause: Option<Document>,
    pub else_clause: Option<Document>,
    pub ensure_clause: Option<Document>,
}
pub struct LayoutParamBlockArgumentNode {
    pub expression: Option<Document>,
}
pub struct LayoutParamBlockLocalVariableNode {
    pub name: Document,
}
pub struct LayoutParamBlockNode {
    pub parameters: Option<Document>,
    pub body: Option<Document>,
}
pub struct LayoutParamBlockParameterNode {
    pub name: Option<Document>,
}
pub struct LayoutParamBlockParametersNode {
    pub parameters: Option<Document>,
    pub locals: Vec<Document>,
}
pub struct LayoutParamBreakNode {
    pub arguments: Option<Document>,
}
pub struct LayoutParamCallAndWriteNode {
    pub receiver: Option<Document>,
    pub value: Document,
}
pub struct LayoutParamCallNode {
    pub receiver: Option<Document>,
    pub arguments: Option<Document>,
    pub block: Option<Document>,
}
pub struct LayoutParamCallOperatorWriteNode {
    pub receiver: Option<Document>,
    pub value: Document,
}
pub struct LayoutParamCallOrWriteNode {
    pub receiver: Option<Document>,
    pub value: Document,
}
pub struct LayoutParamCallTargetNode {
    pub receiver: Document,
}
pub struct LayoutParamCapturePatternNode {
    pub value: Document,
    pub target: Document,
}
pub struct LayoutParamCaseMatchNode {
    pub predicate: Option<Document>,
    pub conditions: Vec<Document>,
    pub else_clause: Option<Document>,
}
pub struct LayoutParamCaseNode {
    pub predicate: Option<Document>,
    pub conditions: Vec<Document>,
    pub else_clause: Option<Document>,
}
pub struct LayoutParamClassNode {
    pub superclass: Option<Document>,
    pub body: Option<Document>,
}
pub struct LayoutParamClassVariableAndWriteNode {
    pub value: Document,
}
pub struct LayoutParamClassVariableOperatorWriteNode {
    pub value: Document,
}
pub struct LayoutParamClassVariableOrWriteNode {
    pub value: Document,
}
pub struct LayoutParamClassVariableReadNode {
    pub name: Document,
}
pub struct LayoutParamClassVariableTargetNode {
    pub name: Document,
}
pub struct LayoutParamClassVariableWriteNode {
    pub value: Document,
}
pub struct LayoutParamConstantAndWriteNode {
    pub value: Document,
}
pub struct LayoutParamConstantOperatorWriteNode {
    pub value: Document,
}
pub struct LayoutParamConstantOrWriteNode {
    pub value: Document,
}
pub struct LayoutParamConstantPathAndWriteNode {
    pub target: Document,
    pub value: Document,
}
pub struct LayoutParamConstantPathNode {
    pub parent: Option<Document>,
}
pub struct LayoutParamConstantPathOperatorWriteNode {
    pub target: Document,
    pub value: Document,
}
pub struct LayoutParamConstantPathOrWriteNode {
    pub target: Document,
    pub value: Document,
}
pub struct LayoutParamConstantPathTargetNode {
    pub parent: Option<Document>,
}
pub struct LayoutParamConstantPathWriteNode {
    pub target: Document,
    pub value: Document,
}
pub struct LayoutParamConstantReadNode {
    pub name: Document,
}
pub struct LayoutParamConstantTargetNode {
    pub name: Document,
}
pub struct LayoutParamConstantWriteNode {
    pub value: Document,
}
pub struct LayoutParamDefNode {
    pub receiver: Option<Document>,
    pub parameters: Option<Document>,
    pub body: Option<Document>,
}
pub struct LayoutParamDefinedNode {
    pub value: Document,
}
pub struct LayoutParamElseNode {
    pub statements: Option<Document>,
}
pub struct LayoutParamEmbeddedStatementsNode {
    pub statements: Option<Document>,
}
pub struct LayoutParamEmbeddedVariableNode {
    pub variable: Document,
}
pub struct LayoutParamEnsureNode {
    pub statements: Option<Document>,
}
pub struct LayoutParamFalseNode {
    pub keyword: Document,
}
pub struct LayoutParamFindPatternNode {
    pub constant: Option<Document>,
    pub left: Document,
    pub requireds: Vec<Document>,
    pub right: Document,
}
pub struct LayoutParamFlipFlopNode {
    pub left: Option<Document>,
    pub right: Option<Document>,
}
pub struct LayoutParamFloatNode {
    pub value: Document,
}
pub struct LayoutParamForNode {
    pub index: Document,
    pub collection: Document,
    pub statements: Option<Document>,
}
pub struct LayoutParamForwardingArgumentsNode {
    pub keyword: Document,
}
pub struct LayoutParamForwardingParameterNode {
    pub keyword: Document,
}
pub struct LayoutParamForwardingSuperNode {
    pub block: Option<Document>,
}
pub struct LayoutParamGlobalVariableAndWriteNode {
    pub value: Document,
}
pub struct LayoutParamGlobalVariableOperatorWriteNode {
    pub value: Document,
}
pub struct LayoutParamGlobalVariableOrWriteNode {
    pub value: Document,
}
pub struct LayoutParamGlobalVariableReadNode {
    pub name: Document,
}
pub struct LayoutParamGlobalVariableTargetNode {
    pub name: Document,
}
pub struct LayoutParamGlobalVariableWriteNode {
    pub name: Document,
    pub value: Document,
}
pub struct LayoutParamHashNode {
    pub elements: Vec<Document>,
}
pub struct LayoutParamHashPatternNode {
    pub constant: Option<Document>,
    pub elements: Vec<Document>,
    pub rest: Option<Document>,
}
pub struct LayoutParamIfNode {
    pub statements: Option<Document>,
    pub subsequent: Option<Document>,
}
pub struct LayoutParamImaginaryNode {
    pub numeric: Document,
}
pub struct LayoutParamImplicitNode {
    pub value: Document,
}
pub struct LayoutParamImplicitRestNode {
    pub keyword: Document,
}
pub struct LayoutParamInNode {
    pub pattern: Document,
    pub statements: Option<Document>,
}
pub struct LayoutParamIndexAndWriteNode {
    pub receiver: Option<Document>,
    pub arguments: Option<Document>,
    pub block: Option<Document>,
    pub value: Document,
}
pub struct LayoutParamIndexOperatorWriteNode {
    pub receiver: Option<Document>,
    pub arguments: Option<Document>,
    pub block: Option<Document>,
    pub value: Document,
}
pub struct LayoutParamIndexOrWriteNode {
    pub receiver: Option<Document>,
    pub arguments: Option<Document>,
    pub block: Option<Document>,
    pub value: Document,
}
pub struct LayoutParamIndexTargetNode {
    pub arguments: Option<Document>,
    pub block: Option<Document>,
}
pub struct LayoutParamInstanceVariableAndWriteNode {
    pub value: Document,
}
pub struct LayoutParamInstanceVariableOperatorWriteNode {
    pub value: Document,
}
pub struct LayoutParamInstanceVariableOrWriteNode {
    pub value: Document,
}
pub struct LayoutParamInstanceVariableReadNode {
    pub name: Document,
}
pub struct LayoutParamInstanceVariableTargetNode {
    pub name: Document,
}
pub struct LayoutParamInstanceVariableWriteNode {
    pub name: Document,
    pub value: Document,
}
pub struct LayoutParamIntegerNode {
    pub value: Document,
}
pub struct LayoutParamInterpolatedMatchLastLineNode {
    pub parts: Vec<Document>,
}
pub struct LayoutParamInterpolatedRegularExpressionNode {
    pub parts: Vec<Document>,
}
pub struct LayoutParamInterpolatedStringNode {
    pub parts: Vec<Document>,
}
pub struct LayoutParamInterpolatedSymbolNode {
    pub parts: Vec<Document>,
}
pub struct LayoutParamInterpolatedXStringNode {
    pub parts: Vec<Document>,
}
pub struct LayoutParamItLocalVariableReadNode {
    pub keyword: Document,
}
pub struct LayoutParamItParametersNode {
    // TODO
}
pub struct LayoutParamKeywordHashNode {
    pub elements: Vec<Document>,
}
pub struct LayoutParamKeywordRestParameterNode {
    pub name: Option<Document>,
}
pub struct LayoutParamLambdaNode {
    pub parameters: Option<Document>,
    pub body: Option<Document>,
}
pub struct LayoutParamLocalVariableAndWriteNode {
    pub value: Document,
}
pub struct LayoutParamLocalVariableOperatorWriteNode {
    pub value: Document,
}
pub struct LayoutParamLocalVariableOrWriteNode {
    pub value: Document,
}
pub struct LayoutParamLocalVariableReadNode {
    pub name: Document,
}
pub struct LayoutParamLocalVariableTargetNode {
    pub name: Document,
}
pub struct LayoutParamLocalVariableWriteNode {
    pub name: Document,
    pub value: Document,
}
pub struct LayoutParamMatchLastLineNode {
    pub escaped: Document,
}
pub struct LayoutParamMatchPredicateNode {
    pub value: Document,
    pub pattern: Document,
}
pub struct LayoutParamMatchRequiredNode {
    pub value: Document,
    pub pattern: Document,
}
pub struct LayoutParamMatchWriteNode {
    pub call: Document,
    pub targets: Vec<Document>,
}
pub struct LayoutParamMissingNode {
    // Empty
}
pub struct LayoutParamModuleNode {
    pub constant_path: Document,
    pub body: Vec<Document>,
}
pub struct LayoutParamMultiTargetNode {
    pub lefts: Vec<Document>,
    pub rest: Option<Document>,
    pub rights: Vec<Document>,
}
pub struct LayoutParamMultiWriteNode {
    pub lefts: Vec<Document>,
    pub rest: Option<Document>,
    pub rights: Vec<Document>,
    pub value: Document,
}
pub struct LayoutParamNextNode {
    pub arguments: Option<Document>,
}
pub struct LayoutParamNilNode {
    pub keyword: Document,
}
pub struct LayoutParamNoKeywordsParameterNode {
    pub keyword: Document,
}
pub struct LayoutParamNumberedParametersNode {
    pub maximum: Document,
}
pub struct LayoutParamNumberedReferenceReadNode {
    pub number: Document,
}
pub struct LayoutParamOptionalKeywordParameterNode {
    pub value: Document,
}
pub struct LayoutParamOptionalParameterNode {
    pub value: Document,
}
pub struct LayoutParamOrNode {
    pub left: Document,
    pub right: Document,
}
pub struct LayoutParamParametersNode {
    pub requireds: Vec<Document>,
    pub optionals: Vec<Document>,
    pub rest: Option<Document>,
    pub posts: Vec<Document>,
    pub keywords: Vec<Document>,
    pub keyword: Option<Document>,
    pub block: Option<Document>,
}
pub struct LayoutParamParenthesesNode {
    pub body: Option<Document>,
}
pub struct LayoutParamPinnedExpressionNode {
    pub expression: Document,
}
pub struct LayoutParamPinnedVariableNode {
    pub variable: Document,
}
pub struct LayoutParamPostExecutionNode {
    pub statements: Option<Document>,
}
pub struct LayoutParamPreExecutionNode {
    pub statements: Option<Document>,
}
pub struct LayoutParamProgramNode {
    pub statements: Document,
}
pub struct LayoutParamRangeNode {
    pub left: Option<Document>,
    pub right: Option<Document>,
}
pub struct LayoutParamRationalNode {}
pub struct LayoutParamRedoNode {
    pub keyword: Document,
}
pub struct LayoutParamRegularExpressionNode {
    pub escaped: Document,
}
pub struct LayoutParamRequiredKeywordParameterNode {
    pub name: Document,
}
pub struct LayoutParamRequiredParameterNode {
    pub name: Document,
}
pub struct LayoutParamRescueModifierNode {
    pub expression: Document,
    pub rescue_expression: Document,
}
pub struct LayoutParamRescueNode {
    pub expressions: Vec<Document>,
    pub reference: Option<Document>,
    pub statements: Option<Document>,
    pub subsequent: Option<Document>,
}
pub struct LayoutParamRestParameterNode {
    pub name: Option<Document>,
}
pub struct LayoutParamRetryNode {
    pub keyword: Document,
}
pub struct LayoutParamReturnNode {
    pub arguments: Option<Document>,
}
pub struct LayoutParamSelfNode {
    pub keyword: Document,
}
pub struct LayoutParamShareableConstantNode {
    pub write: Document,
}
pub struct LayoutParamSingletonClassNode {
    pub expression: Document,
    pub body: Option<Document>,
}
pub struct LayoutParamSourceEncodingNode {
    pub keyword: Document,
}
pub struct LayoutParamSourceFileNode {
    pub keyword: Document,
}
pub struct LayoutParamSourceLineNode {
    pub keyword: Document,
}
pub struct LayoutParamSplatNode {
    pub expression: Option<Document>,
}
pub struct LayoutParamStatementsNode {
    pub body: Vec<Document>,
}
pub struct LayoutParamStringNode {
    pub escaped: Document,
}
pub struct LayoutParamSuperNode {
    pub arguments: Option<Document>,
    pub block: Option<Document>,
}
pub struct LayoutParamSymbolNode {
    pub escaped: Document,
}
pub struct LayoutParamTrueNode {
    pub keyword: Document,
}
pub struct LayoutParamUndefNode {
    pub names: Vec<Document>,
}
pub struct LayoutParamUnlessNode {
    pub predicate: Document,
    pub statements: Option<Document>,
    pub else_clause: Option<Document>,
}
pub struct LayoutParamUntilNode {
    pub predicate: Document,
    pub statements: Option<Document>,
}
pub struct LayoutParamWhenNode {
    pub conditions: Vec<Document>,
    pub statements: Option<Document>,
}
pub struct LayoutParamWhileNode {
    pub predicate: Document,
    pub statements: Option<Document>,
}
pub struct LayoutParamXStringNode {
    pub escaped: Document,
}
pub struct LayoutParamYieldNode {
    pub arguments: Option<Document>,
}
