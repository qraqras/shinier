use crate::ast::*;
use crate::doc::*;
use crate::layout::*;
use ruby_prism::*;

pub struct Visitor {
    pub docs: Docs,
    pub frame: Docs,
}
impl Visitor {
    pub fn new() -> Self {
        Self {
            docs: vec![],
            frame: vec![],
        }
    }
    fn pop_frame(&mut self) -> Doc {
        self.frame.pop().unwrap_or_default()
    }
    fn push_frame(&mut self, doc: Doc) {
        self.frame.push(doc);
    }
    fn clear_frame(&mut self) {
        self.frame.clear();
    }
}
impl<'pr> Visit<'pr> for Visitor {
    /// Visits a `AliasGlobalVariableNode` node.
    fn visit_alias_global_variable_node(&mut self, node: &AliasGlobalVariableNode<'pr>) {
        visit_alias_global_variable_node(self, node);
    }

    /// Visits a `AliasMethodNode` node.
    fn visit_alias_method_node(&mut self, node: &AliasMethodNode<'pr>) {
        visit_alias_method_node(self, node);
    }

    /// Visits a `AlternationPatternNode` node.
    fn visit_alternation_pattern_node(&mut self, node: &AlternationPatternNode<'pr>) {
        visit_alternation_pattern_node(self, node);
    }

    /// Visits a `AndNode` node.
    fn visit_and_node(&mut self, node: &AndNode<'pr>) {
        visit_and_node(self, node);
    }

    /// Visits a `ArgumentsNode` node.
    fn visit_arguments_node(&mut self, node: &ArgumentsNode<'pr>) {
        visit_arguments_node(self, node);
    }

    /// Visits a `ArrayNode` node.
    fn visit_array_node(&mut self, node: &ArrayNode<'pr>) {
        // visit_array_node(self, node);
        for node in node.elements().iter() {
            self.visit(&node);
        }
        self.push_frame(array(&self.frame));
    }

    /// Visits a `ArrayPatternNode` node.
    fn visit_array_pattern_node(&mut self, node: &ArrayPatternNode<'pr>) {
        visit_array_pattern_node(self, node);
    }

    /// Visits a `AssocNode` node.
    fn visit_assoc_node(&mut self, node: &AssocNode<'pr>) {
        visit_assoc_node(self, node);
    }

    /// Visits a `AssocSplatNode` node.
    fn visit_assoc_splat_node(&mut self, node: &AssocSplatNode<'pr>) {
        visit_assoc_splat_node(self, node);
    }

    /// Visits a `BackReferenceReadNode` node.
    fn visit_back_reference_read_node(&mut self, node: &BackReferenceReadNode<'pr>) {
        visit_back_reference_read_node(self, node);
    }

    /// Visits a `BeginNode` node.
    fn visit_begin_node(&mut self, node: &BeginNode<'pr>) {
        visit_begin_node(self, node);
    }

    /// Visits a `BlockArgumentNode` node.
    fn visit_block_argument_node(&mut self, node: &BlockArgumentNode<'pr>) {
        visit_block_argument_node(self, node);
    }

    /// Visits a `BlockLocalVariableNode` node.
    fn visit_block_local_variable_node(&mut self, node: &BlockLocalVariableNode<'pr>) {
        visit_block_local_variable_node(self, node);
    }

    /// Visits a `BlockNode` node.
    fn visit_block_node(&mut self, node: &BlockNode<'pr>) {
        visit_block_node(self, node);
    }

    /// Visits a `BlockParameterNode` node.
    fn visit_block_parameter_node(&mut self, node: &BlockParameterNode<'pr>) {
        visit_block_parameter_node(self, node);
    }

    /// Visits a `BlockParametersNode` node.
    fn visit_block_parameters_node(&mut self, node: &BlockParametersNode<'pr>) {
        visit_block_parameters_node(self, node);
    }

    /// Visits a `BreakNode` node.
    fn visit_break_node(&mut self, node: &BreakNode<'pr>) {
        visit_break_node(self, node);
    }

    /// Visits a `CallAndWriteNode` node.
    fn visit_call_and_write_node(&mut self, node: &CallAndWriteNode<'pr>) {
        visit_call_and_write_node(self, node);
    }

    /// Visits a `CallNode` node.
    fn visit_call_node(&mut self, node: &CallNode<'pr>) {
        /*
        if let Some(node) = node.receiver() {
            visitor.visit(&node);
        }
        if let Some(node) = node.arguments() {
            visitor.visit_arguments_node(&node);
        }
        if let Some(node) = node.block() {
            visitor.visit(&node);
        }
        */
        self.frame.clear();
        let recv = if let Some(node) = node.receiver() {
            self.visit(&node);
            self.frame.pop()
        } else {
            None
        };
        self.frame.clear();
        let args = if let Some(node) = node.arguments() {
            self.visit_arguments_node(&node);
            self.frame.pop()
        } else {
            None
        };
        self.frame.clear();
        let block = if let Some(node) = node.block() {
            self.visit(&node);
            self.frame.pop()
        } else {
            None
        };
        self.push_frame(sequence(vec![
            recv.unwrap_or_default(),
            text(String::from_utf8_lossy(node.name().as_slice()).to_string()),
            text("(".to_string()),
            args.unwrap_or_default(),
            text(")".to_string()),
            block.unwrap_or_default(),
        ]));
    }

    /// Visits a `CallOperatorWriteNode` node.
    fn visit_call_operator_write_node(&mut self, node: &CallOperatorWriteNode<'pr>) {
        visit_call_operator_write_node(self, node);
    }

    /// Visits a `CallOrWriteNode` node.
    fn visit_call_or_write_node(&mut self, node: &CallOrWriteNode<'pr>) {
        visit_call_or_write_node(self, node);
    }

    /// Visits a `CallTargetNode` node.
    fn visit_call_target_node(&mut self, node: &CallTargetNode<'pr>) {
        visit_call_target_node(self, node);
    }

    /// Visits a `CapturePatternNode` node.
    fn visit_capture_pattern_node(&mut self, node: &CapturePatternNode<'pr>) {
        visit_capture_pattern_node(self, node);
    }

    /// Visits a `CaseMatchNode` node.
    fn visit_case_match_node(&mut self, node: &CaseMatchNode<'pr>) {
        visit_case_match_node(self, node);
    }

    /// Visits a `CaseNode` node.
    fn visit_case_node(&mut self, node: &CaseNode<'pr>) {
        visit_case_node(self, node);
    }

    /// Visits a `ClassNode` node.
    fn visit_class_node(&mut self, node: &ClassNode<'pr>) {
        visit_class_node(self, node);
    }

    /// Visits a `ClassVariableAndWriteNode` node.
    fn visit_class_variable_and_write_node(&mut self, node: &ClassVariableAndWriteNode<'pr>) {
        visit_class_variable_and_write_node(self, node);
    }

    /// Visits a `ClassVariableOperatorWriteNode` node.
    fn visit_class_variable_operator_write_node(
        &mut self,
        node: &ClassVariableOperatorWriteNode<'pr>,
    ) {
        visit_class_variable_operator_write_node(self, node);
    }

    /// Visits a `ClassVariableOrWriteNode` node.
    fn visit_class_variable_or_write_node(&mut self, node: &ClassVariableOrWriteNode<'pr>) {
        visit_class_variable_or_write_node(self, node);
    }

    /// Visits a `ClassVariableReadNode` node.
    fn visit_class_variable_read_node(&mut self, node: &ClassVariableReadNode<'pr>) {
        visit_class_variable_read_node(self, node);
    }

    /// Visits a `ClassVariableTargetNode` node.
    fn visit_class_variable_target_node(&mut self, node: &ClassVariableTargetNode<'pr>) {
        visit_class_variable_target_node(self, node);
    }

    /// Visits a `ClassVariableWriteNode` node.
    fn visit_class_variable_write_node(&mut self, node: &ClassVariableWriteNode<'pr>) {
        visit_class_variable_write_node(self, node);
    }

    /// Visits a `ConstantAndWriteNode` node.
    fn visit_constant_and_write_node(&mut self, node: &ConstantAndWriteNode<'pr>) {
        visit_constant_and_write_node(self, node);
    }

    /// Visits a `ConstantOperatorWriteNode` node.
    fn visit_constant_operator_write_node(&mut self, node: &ConstantOperatorWriteNode<'pr>) {
        visit_constant_operator_write_node(self, node);
    }

    /// Visits a `ConstantOrWriteNode` node.
    fn visit_constant_or_write_node(&mut self, node: &ConstantOrWriteNode<'pr>) {
        visit_constant_or_write_node(self, node);
    }

    /// Visits a `ConstantPathAndWriteNode` node.
    fn visit_constant_path_and_write_node(&mut self, node: &ConstantPathAndWriteNode<'pr>) {
        visit_constant_path_and_write_node(self, node);
    }

    /// Visits a `ConstantPathNode` node.
    fn visit_constant_path_node(&mut self, node: &ConstantPathNode<'pr>) {
        visit_constant_path_node(self, node);
    }

    /// Visits a `ConstantPathOperatorWriteNode` node.
    fn visit_constant_path_operator_write_node(
        &mut self,
        node: &ConstantPathOperatorWriteNode<'pr>,
    ) {
        visit_constant_path_operator_write_node(self, node);
    }

    /// Visits a `ConstantPathOrWriteNode` node.
    fn visit_constant_path_or_write_node(&mut self, node: &ConstantPathOrWriteNode<'pr>) {
        visit_constant_path_or_write_node(self, node);
    }

    /// Visits a `ConstantPathTargetNode` node.
    fn visit_constant_path_target_node(&mut self, node: &ConstantPathTargetNode<'pr>) {
        visit_constant_path_target_node(self, node);
    }

    /// Visits a `ConstantPathWriteNode` node.
    fn visit_constant_path_write_node(&mut self, node: &ConstantPathWriteNode<'pr>) {
        visit_constant_path_write_node(self, node);
    }

    /// Visits a `ConstantReadNode` node.
    fn visit_constant_read_node(&mut self, node: &ConstantReadNode<'pr>) {
        visit_constant_read_node(self, node);
    }

    /// Visits a `ConstantTargetNode` node.
    fn visit_constant_target_node(&mut self, node: &ConstantTargetNode<'pr>) {
        visit_constant_target_node(self, node);
    }

    /// Visits a `ConstantWriteNode` node.
    fn visit_constant_write_node(&mut self, node: &ConstantWriteNode<'pr>) {
        visit_constant_write_node(self, node);
    }

    /// Visits a `DefNode` node.
    fn visit_def_node(&mut self, node: &DefNode<'pr>) {
        visit_def_node(self, node);
    }

    /// Visits a `DefinedNode` node.
    fn visit_defined_node(&mut self, node: &DefinedNode<'pr>) {
        visit_defined_node(self, node);
    }

    /// Visits a `ElseNode` node.
    fn visit_else_node(&mut self, node: &ElseNode<'pr>) {
        visit_else_node(self, node);
    }

    /// Visits a `EmbeddedStatementsNode` node.
    fn visit_embedded_statements_node(&mut self, node: &EmbeddedStatementsNode<'pr>) {
        visit_embedded_statements_node(self, node);
    }

    /// Visits a `EmbeddedVariableNode` node.
    fn visit_embedded_variable_node(&mut self, node: &EmbeddedVariableNode<'pr>) {
        visit_embedded_variable_node(self, node);
    }

    /// Visits a `EnsureNode` node.
    fn visit_ensure_node(&mut self, node: &EnsureNode<'pr>) {
        visit_ensure_node(self, node);
    }

    /// Visits a `FalseNode` node.
    fn visit_false_node(&mut self, node: &FalseNode<'pr>) {
        visit_false_node(self, node);
    }

    /// Visits a `FindPatternNode` node.
    fn visit_find_pattern_node(&mut self, node: &FindPatternNode<'pr>) {
        visit_find_pattern_node(self, node);
    }

    /// Visits a `FlipFlopNode` node.
    fn visit_flip_flop_node(&mut self, node: &FlipFlopNode<'pr>) {
        visit_flip_flop_node(self, node);
    }

    /// Visits a `FloatNode` node.
    fn visit_float_node(&mut self, node: &FloatNode<'pr>) {
        visit_float_node(self, node);
    }

    /// Visits a `ForNode` node.
    fn visit_for_node(&mut self, node: &ForNode<'pr>) {
        visit_for_node(self, node);
    }

    /// Visits a `ForwardingArgumentsNode` node.
    fn visit_forwarding_arguments_node(&mut self, node: &ForwardingArgumentsNode<'pr>) {
        visit_forwarding_arguments_node(self, node);
    }

    /// Visits a `ForwardingParameterNode` node.
    fn visit_forwarding_parameter_node(&mut self, node: &ForwardingParameterNode<'pr>) {
        visit_forwarding_parameter_node(self, node);
    }

    /// Visits a `ForwardingSuperNode` node.
    fn visit_forwarding_super_node(&mut self, node: &ForwardingSuperNode<'pr>) {
        visit_forwarding_super_node(self, node);
    }

    /// Visits a `GlobalVariableAndWriteNode` node.
    fn visit_global_variable_and_write_node(&mut self, node: &GlobalVariableAndWriteNode<'pr>) {
        visit_global_variable_and_write_node(self, node);
    }

    /// Visits a `GlobalVariableOperatorWriteNode` node.
    fn visit_global_variable_operator_write_node(
        &mut self,
        node: &GlobalVariableOperatorWriteNode<'pr>,
    ) {
        visit_global_variable_operator_write_node(self, node);
    }

    /// Visits a `GlobalVariableOrWriteNode` node.
    fn visit_global_variable_or_write_node(&mut self, node: &GlobalVariableOrWriteNode<'pr>) {
        visit_global_variable_or_write_node(self, node);
    }

    /// Visits a `GlobalVariableReadNode` node.
    fn visit_global_variable_read_node(&mut self, node: &GlobalVariableReadNode<'pr>) {
        visit_global_variable_read_node(self, node);
    }

    /// Visits a `GlobalVariableTargetNode` node.
    fn visit_global_variable_target_node(&mut self, node: &GlobalVariableTargetNode<'pr>) {
        visit_global_variable_target_node(self, node);
    }

    /// Visits a `GlobalVariableWriteNode` node.
    fn visit_global_variable_write_node(&mut self, node: &GlobalVariableWriteNode<'pr>) {
        visit_global_variable_write_node(self, node);
    }

    /// Visits a `HashNode` node.
    fn visit_hash_node(&mut self, node: &HashNode<'pr>) {
        visit_hash_node(self, node);
    }

    /// Visits a `HashPatternNode` node.
    fn visit_hash_pattern_node(&mut self, node: &HashPatternNode<'pr>) {
        visit_hash_pattern_node(self, node);
    }

    /// Visits a `IfNode` node.
    fn visit_if_node(&mut self, node: &IfNode<'pr>) {
        visit_if_node(self, node);
    }

    /// Visits a `ImaginaryNode` node.
    fn visit_imaginary_node(&mut self, node: &ImaginaryNode<'pr>) {
        visit_imaginary_node(self, node);
    }

    /// Visits a `ImplicitNode` node.
    fn visit_implicit_node(&mut self, node: &ImplicitNode<'pr>) {
        visit_implicit_node(self, node);
    }

    /// Visits a `ImplicitRestNode` node.
    fn visit_implicit_rest_node(&mut self, node: &ImplicitRestNode<'pr>) {
        visit_implicit_rest_node(self, node);
    }

    /// Visits a `InNode` node.
    fn visit_in_node(&mut self, node: &InNode<'pr>) {
        visit_in_node(self, node);
    }

    /// Visits a `IndexAndWriteNode` node.
    fn visit_index_and_write_node(&mut self, node: &IndexAndWriteNode<'pr>) {
        visit_index_and_write_node(self, node);
    }

    /// Visits a `IndexOperatorWriteNode` node.
    fn visit_index_operator_write_node(&mut self, node: &IndexOperatorWriteNode<'pr>) {
        visit_index_operator_write_node(self, node);
    }

    /// Visits a `IndexOrWriteNode` node.
    fn visit_index_or_write_node(&mut self, node: &IndexOrWriteNode<'pr>) {
        visit_index_or_write_node(self, node);
    }

    /// Visits a `IndexTargetNode` node.
    fn visit_index_target_node(&mut self, node: &IndexTargetNode<'pr>) {
        visit_index_target_node(self, node);
    }

    /// Visits a `InstanceVariableAndWriteNode` node.
    fn visit_instance_variable_and_write_node(&mut self, node: &InstanceVariableAndWriteNode<'pr>) {
        visit_instance_variable_and_write_node(self, node);
    }

    /// Visits a `InstanceVariableOperatorWriteNode` node.
    fn visit_instance_variable_operator_write_node(
        &mut self,
        node: &InstanceVariableOperatorWriteNode<'pr>,
    ) {
        visit_instance_variable_operator_write_node(self, node);
    }

    /// Visits a `InstanceVariableOrWriteNode` node.
    fn visit_instance_variable_or_write_node(&mut self, node: &InstanceVariableOrWriteNode<'pr>) {
        visit_instance_variable_or_write_node(self, node);
    }

    /// Visits a `InstanceVariableReadNode` node.
    fn visit_instance_variable_read_node(&mut self, node: &InstanceVariableReadNode<'pr>) {
        visit_instance_variable_read_node(self, node);
    }

    /// Visits a `InstanceVariableTargetNode` node.
    fn visit_instance_variable_target_node(&mut self, node: &InstanceVariableTargetNode<'pr>) {
        visit_instance_variable_target_node(self, node);
    }

    /// Visits a `InstanceVariableWriteNode` node.
    fn visit_instance_variable_write_node(&mut self, node: &InstanceVariableWriteNode<'pr>) {
        visit_instance_variable_write_node(self, node);
    }

    /// Visits a `IntegerNode` node.
    fn visit_integer_node(&mut self, node: &IntegerNode<'pr>) {
        let i: i32 = node.value().try_into().unwrap();
        self.frame.push(text(i.to_string()));
    }

    /// Visits a `InterpolatedMatchLastLineNode` node.
    fn visit_interpolated_match_last_line_node(
        &mut self,
        node: &InterpolatedMatchLastLineNode<'pr>,
    ) {
        visit_interpolated_match_last_line_node(self, node);
    }

    /// Visits a `InterpolatedRegularExpressionNode` node.
    fn visit_interpolated_regular_expression_node(
        &mut self,
        node: &InterpolatedRegularExpressionNode<'pr>,
    ) {
        visit_interpolated_regular_expression_node(self, node);
    }

    /// Visits a `InterpolatedStringNode` node.
    fn visit_interpolated_string_node(&mut self, node: &InterpolatedStringNode<'pr>) {
        visit_interpolated_string_node(self, node);
    }

    /// Visits a `InterpolatedSymbolNode` node.
    fn visit_interpolated_symbol_node(&mut self, node: &InterpolatedSymbolNode<'pr>) {
        visit_interpolated_symbol_node(self, node);
    }

    /// Visits a `InterpolatedXStringNode` node.
    fn visit_interpolated_x_string_node(&mut self, node: &InterpolatedXStringNode<'pr>) {
        visit_interpolated_x_string_node(self, node);
    }

    /// Visits a `ItLocalVariableReadNode` node.
    fn visit_it_local_variable_read_node(&mut self, node: &ItLocalVariableReadNode<'pr>) {
        visit_it_local_variable_read_node(self, node);
    }

    /// Visits a `ItParametersNode` node.
    fn visit_it_parameters_node(&mut self, node: &ItParametersNode<'pr>) {
        visit_it_parameters_node(self, node);
    }

    /// Visits a `KeywordHashNode` node.
    fn visit_keyword_hash_node(&mut self, node: &KeywordHashNode<'pr>) {
        visit_keyword_hash_node(self, node);
    }

    /// Visits a `KeywordRestParameterNode` node.
    fn visit_keyword_rest_parameter_node(&mut self, node: &KeywordRestParameterNode<'pr>) {
        visit_keyword_rest_parameter_node(self, node);
    }

    /// Visits a `LambdaNode` node.
    fn visit_lambda_node(&mut self, node: &LambdaNode<'pr>) {
        visit_lambda_node(self, node);
    }

    /// Visits a `LocalVariableAndWriteNode` node.
    fn visit_local_variable_and_write_node(&mut self, node: &LocalVariableAndWriteNode<'pr>) {
        visit_local_variable_and_write_node(self, node);
    }

    /// Visits a `LocalVariableOperatorWriteNode` node.
    fn visit_local_variable_operator_write_node(
        &mut self,
        node: &LocalVariableOperatorWriteNode<'pr>,
    ) {
        visit_local_variable_operator_write_node(self, node);
    }

    /// Visits a `LocalVariableOrWriteNode` node.
    fn visit_local_variable_or_write_node(&mut self, node: &LocalVariableOrWriteNode<'pr>) {
        visit_local_variable_or_write_node(self, node);
    }

    /// Visits a `LocalVariableReadNode` node.
    fn visit_local_variable_read_node(&mut self, node: &LocalVariableReadNode<'pr>) {
        visit_local_variable_read_node(self, node);
    }

    /// Visits a `LocalVariableTargetNode` node.
    fn visit_local_variable_target_node(&mut self, node: &LocalVariableTargetNode<'pr>) {
        visit_local_variable_target_node(self, node);
    }

    /// Visits a `LocalVariableWriteNode` node.
    fn visit_local_variable_write_node(&mut self, node: &LocalVariableWriteNode<'pr>) {
        visit_local_variable_write_node(self, node);
    }

    /// Visits a `MatchLastLineNode` node.
    fn visit_match_last_line_node(&mut self, node: &MatchLastLineNode<'pr>) {
        visit_match_last_line_node(self, node);
    }

    /// Visits a `MatchPredicateNode` node.
    fn visit_match_predicate_node(&mut self, node: &MatchPredicateNode<'pr>) {
        visit_match_predicate_node(self, node);
    }

    /// Visits a `MatchRequiredNode` node.
    fn visit_match_required_node(&mut self, node: &MatchRequiredNode<'pr>) {
        visit_match_required_node(self, node);
    }

    /// Visits a `MatchWriteNode` node.
    fn visit_match_write_node(&mut self, node: &MatchWriteNode<'pr>) {
        visit_match_write_node(self, node);
    }

    /// Visits a `MissingNode` node.
    fn visit_missing_node(&mut self, node: &MissingNode<'pr>) {
        visit_missing_node(self, node);
    }

    /// Visits a `ModuleNode` node.
    fn visit_module_node(&mut self, node: &ModuleNode<'pr>) {
        visit_module_node(self, node);
    }

    /// Visits a `MultiTargetNode` node.
    fn visit_multi_target_node(&mut self, node: &MultiTargetNode<'pr>) {
        visit_multi_target_node(self, node);
    }

    /// Visits a `MultiWriteNode` node.
    fn visit_multi_write_node(&mut self, node: &MultiWriteNode<'pr>) {
        visit_multi_write_node(self, node);
    }

    /// Visits a `NextNode` node.
    fn visit_next_node(&mut self, node: &NextNode<'pr>) {
        visit_next_node(self, node);
    }

    /// Visits a `NilNode` node.
    fn visit_nil_node(&mut self, node: &NilNode<'pr>) {
        visit_nil_node(self, node);
    }

    /// Visits a `NoKeywordsParameterNode` node.
    fn visit_no_keywords_parameter_node(&mut self, node: &NoKeywordsParameterNode<'pr>) {
        visit_no_keywords_parameter_node(self, node);
    }

    /// Visits a `NumberedParametersNode` node.
    fn visit_numbered_parameters_node(&mut self, node: &NumberedParametersNode<'pr>) {
        visit_numbered_parameters_node(self, node);
    }

    /// Visits a `NumberedReferenceReadNode` node.
    fn visit_numbered_reference_read_node(&mut self, node: &NumberedReferenceReadNode<'pr>) {
        visit_numbered_reference_read_node(self, node);
    }

    /// Visits a `OptionalKeywordParameterNode` node.
    fn visit_optional_keyword_parameter_node(&mut self, node: &OptionalKeywordParameterNode<'pr>) {
        visit_optional_keyword_parameter_node(self, node);
    }

    /// Visits a `OptionalParameterNode` node.
    fn visit_optional_parameter_node(&mut self, node: &OptionalParameterNode<'pr>) {
        visit_optional_parameter_node(self, node);
    }

    /// Visits a `OrNode` node.
    fn visit_or_node(&mut self, node: &OrNode<'pr>) {
        visit_or_node(self, node);
    }

    /// Visits a `ParametersNode` node.
    fn visit_parameters_node(&mut self, node: &ParametersNode<'pr>) {
        visit_parameters_node(self, node);
    }

    /// Visits a `ParenthesesNode` node.
    fn visit_parentheses_node(&mut self, node: &ParenthesesNode<'pr>) {
        visit_parentheses_node(self, node);
    }

    /// Visits a `PinnedExpressionNode` node.
    fn visit_pinned_expression_node(&mut self, node: &PinnedExpressionNode<'pr>) {
        visit_pinned_expression_node(self, node);
    }

    /// Visits a `PinnedVariableNode` node.
    fn visit_pinned_variable_node(&mut self, node: &PinnedVariableNode<'pr>) {
        visit_pinned_variable_node(self, node);
    }

    /// Visits a `PostExecutionNode` node.
    fn visit_post_execution_node(&mut self, node: &PostExecutionNode<'pr>) {
        visit_post_execution_node(self, node);
    }

    /// Visits a `PreExecutionNode` node.
    fn visit_pre_execution_node(&mut self, node: &PreExecutionNode<'pr>) {
        visit_pre_execution_node(self, node);
    }

    /// Visits a `ProgramNode` node.
    fn visit_program_node(&mut self, node: &ProgramNode<'pr>) {
        visit_program_node(self, node);
    }

    /// Visits a `RangeNode` node.
    fn visit_range_node(&mut self, node: &RangeNode<'pr>) {
        visit_range_node(self, node);
    }

    /// Visits a `RationalNode` node.
    fn visit_rational_node(&mut self, node: &RationalNode<'pr>) {
        visit_rational_node(self, node);
    }

    /// Visits a `RedoNode` node.
    fn visit_redo_node(&mut self, node: &RedoNode<'pr>) {
        visit_redo_node(self, node);
    }

    /// Visits a `RegularExpressionNode` node.
    fn visit_regular_expression_node(&mut self, node: &RegularExpressionNode<'pr>) {
        visit_regular_expression_node(self, node);
    }

    /// Visits a `RequiredKeywordParameterNode` node.
    fn visit_required_keyword_parameter_node(&mut self, node: &RequiredKeywordParameterNode<'pr>) {
        visit_required_keyword_parameter_node(self, node);
    }

    /// Visits a `RequiredParameterNode` node.
    fn visit_required_parameter_node(&mut self, node: &RequiredParameterNode<'pr>) {
        visit_required_parameter_node(self, node);
    }

    /// Visits a `RescueModifierNode` node.
    fn visit_rescue_modifier_node(&mut self, node: &RescueModifierNode<'pr>) {
        visit_rescue_modifier_node(self, node);
    }

    /// Visits a `RescueNode` node.
    fn visit_rescue_node(&mut self, node: &RescueNode<'pr>) {
        visit_rescue_node(self, node);
    }

    /// Visits a `RestParameterNode` node.
    fn visit_rest_parameter_node(&mut self, node: &RestParameterNode<'pr>) {
        visit_rest_parameter_node(self, node);
    }

    /// Visits a `RetryNode` node.
    fn visit_retry_node(&mut self, node: &RetryNode<'pr>) {
        visit_retry_node(self, node);
    }

    /// Visits a `ReturnNode` node.
    fn visit_return_node(&mut self, node: &ReturnNode<'pr>) {
        visit_return_node(self, node);
    }

    /// Visits a `SelfNode` node.
    fn visit_self_node(&mut self, node: &SelfNode<'pr>) {
        visit_self_node(self, node);
    }

    /// Visits a `ShareableConstantNode` node.
    fn visit_shareable_constant_node(&mut self, node: &ShareableConstantNode<'pr>) {
        visit_shareable_constant_node(self, node);
    }

    /// Visits a `SingletonClassNode` node.
    fn visit_singleton_class_node(&mut self, node: &SingletonClassNode<'pr>) {
        visit_singleton_class_node(self, node);
    }

    /// Visits a `SourceEncodingNode` node.
    fn visit_source_encoding_node(&mut self, node: &SourceEncodingNode<'pr>) {
        visit_source_encoding_node(self, node);
    }

    /// Visits a `SourceFileNode` node.
    fn visit_source_file_node(&mut self, node: &SourceFileNode<'pr>) {
        visit_source_file_node(self, node);
    }

    /// Visits a `SourceLineNode` node.
    fn visit_source_line_node(&mut self, node: &SourceLineNode<'pr>) {
        visit_source_line_node(self, node);
    }

    /// Visits a `SplatNode` node.
    fn visit_splat_node(&mut self, node: &SplatNode<'pr>) {
        visit_splat_node(self, node);
    }

    /// Visits a `StatementsNode` node.
    fn visit_statements_node(&mut self, node: &StatementsNode<'pr>) {
        // visit_statements_node(self, node);
        for node in node.body().iter() {
            self.clear_frame();
            self.visit(&node);
            let stmt = self.pop_frame();
            self.docs.push(stmt);
        }
    }

    /// Visits a `StringNode` node.
    fn visit_string_node(&mut self, node: &StringNode<'pr>) {
        visit_string_node(self, node);
    }

    /// Visits a `SuperNode` node.
    fn visit_super_node(&mut self, node: &SuperNode<'pr>) {
        visit_super_node(self, node);
    }

    /// Visits a `SymbolNode` node.
    fn visit_symbol_node(&mut self, node: &SymbolNode<'pr>) {
        visit_symbol_node(self, node);
    }

    /// Visits a `TrueNode` node.
    fn visit_true_node(&mut self, node: &TrueNode<'pr>) {
        visit_true_node(self, node);
    }

    /// Visits a `UndefNode` node.
    fn visit_undef_node(&mut self, node: &UndefNode<'pr>) {
        visit_undef_node(self, node);
    }

    /// Visits a `UnlessNode` node.
    fn visit_unless_node(&mut self, node: &UnlessNode<'pr>) {
        visit_unless_node(self, node);
    }

    /// Visits a `UntilNode` node.
    fn visit_until_node(&mut self, node: &UntilNode<'pr>) {
        visit_until_node(self, node);
    }

    /// Visits a `WhenNode` node.
    fn visit_when_node(&mut self, node: &WhenNode<'pr>) {
        visit_when_node(self, node);
    }

    /// Visits a `WhileNode` node.
    fn visit_while_node(&mut self, node: &WhileNode<'pr>) {
        visit_while_node(self, node);
    }

    /// Visits a `XStringNode` node.
    fn visit_x_string_node(&mut self, node: &XStringNode<'pr>) {
        visit_x_string_node(self, node);
    }

    /// Visits a `YieldNode` node.
    fn visit_yield_node(&mut self, node: &YieldNode<'pr>) {
        visit_yield_node(self, node);
    }
}
