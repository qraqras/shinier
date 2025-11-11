use ruby_prism::*;

/// Visitor trait that visits all node variants including special nodes.
///
/// `ruby_prism::Visit` trait does not visit some special nodes like `RescueNode`,
/// `EnsureNode`, and `ElseNode`.
/// This trait ensures all nodes in the AST are visited by explicitly visiting these special nodes.
pub trait VisitAll<'sh> {
    #[rustfmt::skip]
    fn visit(&mut self, node: &Node<'sh>) {
        self.node_enter(&node);
        match node {
            Node::AliasGlobalVariableNode           { .. } => self.visit_alias_global_variable_node           (&node.as_alias_global_variable_node().unwrap()           ),
            Node::AliasMethodNode                   { .. } => self.visit_alias_method_node                    (&node.as_alias_method_node().unwrap()                    ),
            Node::AlternationPatternNode            { .. } => self.visit_alternation_pattern_node             (&node.as_alternation_pattern_node().unwrap()             ),
            Node::AndNode                           { .. } => self.visit_and_node                             (&node.as_and_node().unwrap()                             ),
            Node::ArgumentsNode                     { .. } => self.visit_arguments_node                       (&node.as_arguments_node().unwrap()                       ),
            Node::ArrayNode                         { .. } => self.visit_array_node                           (&node.as_array_node().unwrap()                           ),
            Node::ArrayPatternNode                  { .. } => self.visit_array_pattern_node                   (&node.as_array_pattern_node().unwrap()                   ),
            Node::AssocNode                         { .. } => self.visit_assoc_node                           (&node.as_assoc_node().unwrap()                           ),
            Node::AssocSplatNode                    { .. } => self.visit_assoc_splat_node                     (&node.as_assoc_splat_node().unwrap()                     ),
            Node::BackReferenceReadNode             { .. } => self.visit_back_reference_read_node             (&node.as_back_reference_read_node().unwrap()             ),
            Node::BeginNode                         { .. } => self.visit_begin_node                           (&node.as_begin_node().unwrap()                           ),
            Node::BlockArgumentNode                 { .. } => self.visit_block_argument_node                  (&node.as_block_argument_node().unwrap()                  ),
            Node::BlockLocalVariableNode            { .. } => self.visit_block_local_variable_node            (&node.as_block_local_variable_node().unwrap()            ),
            Node::BlockNode                         { .. } => self.visit_block_node                           (&node.as_block_node().unwrap()                           ),
            Node::BlockParameterNode                { .. } => self.visit_block_parameter_node                 (&node.as_block_parameter_node().unwrap()                 ),
            Node::BlockParametersNode               { .. } => self.visit_block_parameters_node                (&node.as_block_parameters_node().unwrap()                ),
            Node::BreakNode                         { .. } => self.visit_break_node                           (&node.as_break_node().unwrap()                           ),
            Node::CallAndWriteNode                  { .. } => self.visit_call_and_write_node                  (&node.as_call_and_write_node().unwrap()                  ),
            Node::CallNode                          { .. } => self.visit_call_node                            (&node.as_call_node().unwrap()                            ),
            Node::CallOperatorWriteNode             { .. } => self.visit_call_operator_write_node             (&node.as_call_operator_write_node().unwrap()             ),
            Node::CallOrWriteNode                   { .. } => self.visit_call_or_write_node                   (&node.as_call_or_write_node().unwrap()                   ),
            Node::CallTargetNode                    { .. } => self.visit_call_target_node                     (&node.as_call_target_node().unwrap()                     ),
            Node::CapturePatternNode                { .. } => self.visit_capture_pattern_node                 (&node.as_capture_pattern_node().unwrap()                 ),
            Node::CaseMatchNode                     { .. } => self.visit_case_match_node                      (&node.as_case_match_node().unwrap()                      ),
            Node::CaseNode                          { .. } => self.visit_case_node                            (&node.as_case_node().unwrap()                            ),
            Node::ClassNode                         { .. } => self.visit_class_node                           (&node.as_class_node().unwrap()                           ),
            Node::ClassVariableAndWriteNode         { .. } => self.visit_class_variable_and_write_node        (&node.as_class_variable_and_write_node().unwrap()        ),
            Node::ClassVariableOperatorWriteNode    { .. } => self.visit_class_variable_operator_write_node   (&node.as_class_variable_operator_write_node().unwrap()   ),
            Node::ClassVariableOrWriteNode          { .. } => self.visit_class_variable_or_write_node         (&node.as_class_variable_or_write_node().unwrap()         ),
            Node::ClassVariableReadNode             { .. } => self.visit_class_variable_read_node             (&node.as_class_variable_read_node().unwrap()             ),
            Node::ClassVariableTargetNode           { .. } => self.visit_class_variable_target_node           (&node.as_class_variable_target_node().unwrap()           ),
            Node::ClassVariableWriteNode            { .. } => self.visit_class_variable_write_node            (&node.as_class_variable_write_node().unwrap()            ),
            Node::ConstantAndWriteNode              { .. } => self.visit_constant_and_write_node              (&node.as_constant_and_write_node().unwrap()              ),
            Node::ConstantOperatorWriteNode         { .. } => self.visit_constant_operator_write_node         (&node.as_constant_operator_write_node().unwrap()         ),
            Node::ConstantOrWriteNode               { .. } => self.visit_constant_or_write_node               (&node.as_constant_or_write_node().unwrap()               ),
            Node::ConstantPathAndWriteNode          { .. } => self.visit_constant_path_and_write_node         (&node.as_constant_path_and_write_node().unwrap()         ),
            Node::ConstantPathNode                  { .. } => self.visit_constant_path_node                   (&node.as_constant_path_node().unwrap()                   ),
            Node::ConstantPathOperatorWriteNode     { .. } => self.visit_constant_path_operator_write_node    (&node.as_constant_path_operator_write_node().unwrap()    ),
            Node::ConstantPathOrWriteNode           { .. } => self.visit_constant_path_or_write_node          (&node.as_constant_path_or_write_node().unwrap()          ),
            Node::ConstantPathTargetNode            { .. } => self.visit_constant_path_target_node            (&node.as_constant_path_target_node().unwrap()            ),
            Node::ConstantPathWriteNode             { .. } => self.visit_constant_path_write_node             (&node.as_constant_path_write_node().unwrap()             ),
            Node::ConstantReadNode                  { .. } => self.visit_constant_read_node                   (&node.as_constant_read_node().unwrap()                   ),
            Node::ConstantTargetNode                { .. } => self.visit_constant_target_node                 (&node.as_constant_target_node().unwrap()                 ),
            Node::ConstantWriteNode                 { .. } => self.visit_constant_write_node                  (&node.as_constant_write_node().unwrap()                  ),
            Node::DefNode                           { .. } => self.visit_def_node                             (&node.as_def_node().unwrap()                             ),
            Node::DefinedNode                       { .. } => self.visit_defined_node                         (&node.as_defined_node().unwrap()                         ),
            Node::ElseNode                          { .. } => self.visit_else_node                            (&node.as_else_node().unwrap()                            ),
            Node::EmbeddedStatementsNode            { .. } => self.visit_embedded_statements_node             (&node.as_embedded_statements_node().unwrap()             ),
            Node::EmbeddedVariableNode              { .. } => self.visit_embedded_variable_node               (&node.as_embedded_variable_node().unwrap()               ),
            Node::EnsureNode                        { .. } => self.visit_ensure_node                          (&node.as_ensure_node().unwrap()                          ),
            Node::FalseNode                         { .. } => self.visit_false_node                           (&node.as_false_node().unwrap()                           ),
            Node::FindPatternNode                   { .. } => self.visit_find_pattern_node                    (&node.as_find_pattern_node().unwrap()                    ),
            Node::FlipFlopNode                      { .. } => self.visit_flip_flop_node                       (&node.as_flip_flop_node().unwrap()                       ),
            Node::FloatNode                         { .. } => self.visit_float_node                           (&node.as_float_node().unwrap()                           ),
            Node::ForNode                           { .. } => self.visit_for_node                             (&node.as_for_node().unwrap()                             ),
            Node::ForwardingArgumentsNode           { .. } => self.visit_forwarding_arguments_node            (&node.as_forwarding_arguments_node().unwrap()            ),
            Node::ForwardingParameterNode           { .. } => self.visit_forwarding_parameter_node            (&node.as_forwarding_parameter_node().unwrap()            ),
            Node::ForwardingSuperNode               { .. } => self.visit_forwarding_super_node                (&node.as_forwarding_super_node().unwrap()                ),
            Node::GlobalVariableAndWriteNode        { .. } => self.visit_global_variable_and_write_node       (&node.as_global_variable_and_write_node().unwrap()       ),
            Node::GlobalVariableOperatorWriteNode   { .. } => self.visit_global_variable_operator_write_node  (&node.as_global_variable_operator_write_node().unwrap()  ),
            Node::GlobalVariableOrWriteNode         { .. } => self.visit_global_variable_or_write_node        (&node.as_global_variable_or_write_node().unwrap()        ),
            Node::GlobalVariableReadNode            { .. } => self.visit_global_variable_read_node            (&node.as_global_variable_read_node().unwrap()            ),
            Node::GlobalVariableTargetNode          { .. } => self.visit_global_variable_target_node          (&node.as_global_variable_target_node().unwrap()          ),
            Node::GlobalVariableWriteNode           { .. } => self.visit_global_variable_write_node           (&node.as_global_variable_write_node().unwrap()           ),
            Node::HashNode                          { .. } => self.visit_hash_node                            (&node.as_hash_node().unwrap()                            ),
            Node::HashPatternNode                   { .. } => self.visit_hash_pattern_node                    (&node.as_hash_pattern_node().unwrap()                    ),
            Node::IfNode                            { .. } => self.visit_if_node                              (&node.as_if_node().unwrap()                              ),
            Node::ImaginaryNode                     { .. } => self.visit_imaginary_node                       (&node.as_imaginary_node().unwrap()                       ),
            Node::ImplicitNode                      { .. } => self.visit_implicit_node                        (&node.as_implicit_node().unwrap()                        ),
            Node::ImplicitRestNode                  { .. } => self.visit_implicit_rest_node                   (&node.as_implicit_rest_node().unwrap()                   ),
            Node::InNode                            { .. } => self.visit_in_node                              (&node.as_in_node().unwrap()                              ),
            Node::IndexAndWriteNode                 { .. } => self.visit_index_and_write_node                 (&node.as_index_and_write_node().unwrap()                 ),
            Node::IndexOperatorWriteNode            { .. } => self.visit_index_operator_write_node            (&node.as_index_operator_write_node().unwrap()            ),
            Node::IndexOrWriteNode                  { .. } => self.visit_index_or_write_node                  (&node.as_index_or_write_node().unwrap()                  ),
            Node::IndexTargetNode                   { .. } => self.visit_index_target_node                    (&node.as_index_target_node().unwrap()                    ),
            Node::InstanceVariableAndWriteNode      { .. } => self.visit_instance_variable_and_write_node     (&node.as_instance_variable_and_write_node().unwrap()     ),
            Node::InstanceVariableOperatorWriteNode { .. } => self.visit_instance_variable_operator_write_node(&node.as_instance_variable_operator_write_node().unwrap()),
            Node::InstanceVariableOrWriteNode       { .. } => self.visit_instance_variable_or_write_node      (&node.as_instance_variable_or_write_node().unwrap()      ),
            Node::InstanceVariableReadNode          { .. } => self.visit_instance_variable_read_node          (&node.as_instance_variable_read_node().unwrap()          ),
            Node::InstanceVariableTargetNode        { .. } => self.visit_instance_variable_target_node        (&node.as_instance_variable_target_node().unwrap()        ),
            Node::InstanceVariableWriteNode         { .. } => self.visit_instance_variable_write_node         (&node.as_instance_variable_write_node().unwrap()         ),
            Node::IntegerNode                       { .. } => self.visit_integer_node                         (&node.as_integer_node().unwrap()                         ),
            Node::InterpolatedMatchLastLineNode     { .. } => self.visit_interpolated_match_last_line_node    (&node.as_interpolated_match_last_line_node().unwrap()    ),
            Node::InterpolatedRegularExpressionNode { .. } => self.visit_interpolated_regular_expression_node (&node.as_interpolated_regular_expression_node().unwrap() ),
            Node::InterpolatedStringNode            { .. } => self.visit_interpolated_string_node             (&node.as_interpolated_string_node().unwrap()             ),
            Node::InterpolatedSymbolNode            { .. } => self.visit_interpolated_symbol_node             (&node.as_interpolated_symbol_node().unwrap()             ),
            Node::InterpolatedXStringNode           { .. } => self.visit_interpolated_x_string_node           (&node.as_interpolated_x_string_node().unwrap()           ),
            Node::ItLocalVariableReadNode           { .. } => self.visit_it_local_variable_read_node          (&node.as_it_local_variable_read_node().unwrap()          ),
            Node::ItParametersNode                  { .. } => self.visit_it_parameters_node                   (&node.as_it_parameters_node().unwrap()                   ),
            Node::KeywordHashNode                   { .. } => self.visit_keyword_hash_node                    (&node.as_keyword_hash_node().unwrap()                    ),
            Node::KeywordRestParameterNode          { .. } => self.visit_keyword_rest_parameter_node          (&node.as_keyword_rest_parameter_node().unwrap()          ),
            Node::LambdaNode                        { .. } => self.visit_lambda_node                          (&node.as_lambda_node().unwrap()                          ),
            Node::LocalVariableAndWriteNode         { .. } => self.visit_local_variable_and_write_node        (&node.as_local_variable_and_write_node().unwrap()        ),
            Node::LocalVariableOperatorWriteNode    { .. } => self.visit_local_variable_operator_write_node   (&node.as_local_variable_operator_write_node().unwrap()   ),
            Node::LocalVariableOrWriteNode          { .. } => self.visit_local_variable_or_write_node         (&node.as_local_variable_or_write_node().unwrap()         ),
            Node::LocalVariableReadNode             { .. } => self.visit_local_variable_read_node             (&node.as_local_variable_read_node().unwrap()             ),
            Node::LocalVariableTargetNode           { .. } => self.visit_local_variable_target_node           (&node.as_local_variable_target_node().unwrap()           ),
            Node::LocalVariableWriteNode            { .. } => self.visit_local_variable_write_node            (&node.as_local_variable_write_node().unwrap()            ),
            Node::MatchLastLineNode                 { .. } => self.visit_match_last_line_node                 (&node.as_match_last_line_node().unwrap()                 ),
            Node::MatchPredicateNode                { .. } => self.visit_match_predicate_node                 (&node.as_match_predicate_node().unwrap()                 ),
            Node::MatchRequiredNode                 { .. } => self.visit_match_required_node                  (&node.as_match_required_node().unwrap()                  ),
            Node::MatchWriteNode                    { .. } => self.visit_match_write_node                     (&node.as_match_write_node().unwrap()                     ),
            Node::MissingNode                       { .. } => self.visit_missing_node                         (&node.as_missing_node().unwrap()                         ),
            Node::ModuleNode                        { .. } => self.visit_module_node                          (&node.as_module_node().unwrap()                          ),
            Node::MultiTargetNode                   { .. } => self.visit_multi_target_node                    (&node.as_multi_target_node().unwrap()                    ),
            Node::MultiWriteNode                    { .. } => self.visit_multi_write_node                     (&node.as_multi_write_node().unwrap()                     ),
            Node::NextNode                          { .. } => self.visit_next_node                            (&node.as_next_node().unwrap()                            ),
            Node::NilNode                           { .. } => self.visit_nil_node                             (&node.as_nil_node().unwrap()                             ),
            Node::NoKeywordsParameterNode           { .. } => self.visit_no_keywords_parameter_node           (&node.as_no_keywords_parameter_node().unwrap()           ),
            Node::NumberedParametersNode            { .. } => self.visit_numbered_parameters_node             (&node.as_numbered_parameters_node().unwrap()             ),
            Node::NumberedReferenceReadNode         { .. } => self.visit_numbered_reference_read_node         (&node.as_numbered_reference_read_node().unwrap()         ),
            Node::OptionalKeywordParameterNode      { .. } => self.visit_optional_keyword_parameter_node      (&node.as_optional_keyword_parameter_node().unwrap()      ),
            Node::OptionalParameterNode             { .. } => self.visit_optional_parameter_node              (&node.as_optional_parameter_node().unwrap()              ),
            Node::OrNode                            { .. } => self.visit_or_node                              (&node.as_or_node().unwrap()                              ),
            Node::ParametersNode                    { .. } => self.visit_parameters_node                      (&node.as_parameters_node().unwrap()                      ),
            Node::ParenthesesNode                   { .. } => self.visit_parentheses_node                     (&node.as_parentheses_node().unwrap()                     ),
            Node::PinnedExpressionNode              { .. } => self.visit_pinned_expression_node               (&node.as_pinned_expression_node().unwrap()               ),
            Node::PinnedVariableNode                { .. } => self.visit_pinned_variable_node                 (&node.as_pinned_variable_node().unwrap()                 ),
            Node::PostExecutionNode                 { .. } => self.visit_post_execution_node                  (&node.as_post_execution_node().unwrap()                  ),
            Node::PreExecutionNode                  { .. } => self.visit_pre_execution_node                   (&node.as_pre_execution_node().unwrap()                   ),
            Node::ProgramNode                       { .. } => self.visit_program_node                         (&node.as_program_node().unwrap()                         ),
            Node::RangeNode                         { .. } => self.visit_range_node                           (&node.as_range_node().unwrap()                           ),
            Node::RationalNode                      { .. } => self.visit_rational_node                        (&node.as_rational_node().unwrap()                        ),
            Node::RedoNode                          { .. } => self.visit_redo_node                            (&node.as_redo_node().unwrap()                            ),
            Node::RegularExpressionNode             { .. } => self.visit_regular_expression_node              (&node.as_regular_expression_node().unwrap()              ),
            Node::RequiredKeywordParameterNode      { .. } => self.visit_required_keyword_parameter_node      (&node.as_required_keyword_parameter_node().unwrap()      ),
            Node::RequiredParameterNode             { .. } => self.visit_required_parameter_node              (&node.as_required_parameter_node().unwrap()              ),
            Node::RescueModifierNode                { .. } => self.visit_rescue_modifier_node                 (&node.as_rescue_modifier_node().unwrap()                 ),
            Node::RescueNode                        { .. } => self.visit_rescue_node                          (&node.as_rescue_node().unwrap()                          ),
            Node::RestParameterNode                 { .. } => self.visit_rest_parameter_node                  (&node.as_rest_parameter_node().unwrap()                  ),
            Node::RetryNode                         { .. } => self.visit_retry_node                           (&node.as_retry_node().unwrap()                           ),
            Node::ReturnNode                        { .. } => self.visit_return_node                          (&node.as_return_node().unwrap()                          ),
            Node::SelfNode                          { .. } => self.visit_self_node                            (&node.as_self_node().unwrap()                            ),
            Node::ShareableConstantNode             { .. } => self.visit_shareable_constant_node              (&node.as_shareable_constant_node().unwrap()              ),
            Node::SingletonClassNode                { .. } => self.visit_singleton_class_node                 (&node.as_singleton_class_node().unwrap()                 ),
            Node::SourceEncodingNode                { .. } => self.visit_source_encoding_node                 (&node.as_source_encoding_node().unwrap()                 ),
            Node::SourceFileNode                    { .. } => self.visit_source_file_node                     (&node.as_source_file_node().unwrap()                     ),
            Node::SourceLineNode                    { .. } => self.visit_source_line_node                     (&node.as_source_line_node().unwrap()                     ),
            Node::SplatNode                         { .. } => self.visit_splat_node                           (&node.as_splat_node().unwrap()                           ),
            Node::StatementsNode                    { .. } => self.visit_statements_node                      (&node.as_statements_node().unwrap()                      ),
            Node::StringNode                        { .. } => self.visit_string_node                          (&node.as_string_node().unwrap()                          ),
            Node::SuperNode                         { .. } => self.visit_super_node                           (&node.as_super_node().unwrap()                           ),
            Node::SymbolNode                        { .. } => self.visit_symbol_node                          (&node.as_symbol_node().unwrap()                          ),
            Node::TrueNode                          { .. } => self.visit_true_node                            (&node.as_true_node().unwrap()                            ),
            Node::UndefNode                         { .. } => self.visit_undef_node                           (&node.as_undef_node().unwrap()                           ),
            Node::UnlessNode                        { .. } => self.visit_unless_node                          (&node.as_unless_node().unwrap()                          ),
            Node::UntilNode                         { .. } => self.visit_until_node                           (&node.as_until_node().unwrap()                           ),
            Node::WhenNode                          { .. } => self.visit_when_node                            (&node.as_when_node().unwrap()                            ),
            Node::WhileNode                         { .. } => self.visit_while_node                           (&node.as_while_node().unwrap()                           ),
            Node::XStringNode                       { .. } => self.visit_x_string_node                        (&node.as_x_string_node().unwrap()                        ),
            Node::YieldNode                         { .. } => self.visit_yield_node                           (&node.as_yield_node().unwrap()                           ),
        }
        self.node_leave(&node);
    }
    // individual node enter/leave methods
    #[rustfmt::skip]
    fn node_enter(&mut self, _node: &Node<'sh>) {}
    #[rustfmt::skip]
    fn node_leave(&mut self, _node: &Node<'sh>) {}
    #[rustfmt::skip]
    fn alias_global_variable_node_enter(&mut self, _node: &AliasGlobalVariableNode<'sh>) {}
    #[rustfmt::skip]
    fn alias_method_node_enter(&mut self, _node: &AliasMethodNode<'sh>) {}
    #[rustfmt::skip]
    fn alternation_pattern_node_enter(&mut self, _node: &AlternationPatternNode<'sh>) {}
    #[rustfmt::skip]
    fn and_node_enter(&mut self, _node: &AndNode<'sh>) {}
    #[rustfmt::skip]
    fn arguments_node_enter(&mut self, _node: &ArgumentsNode<'sh>) {}
    #[rustfmt::skip]
    fn array_node_enter(&mut self, _node: &ArrayNode<'sh>) {}
    #[rustfmt::skip]
    fn array_pattern_node_enter(&mut self, _node: &ArrayPatternNode<'sh>) {}
    #[rustfmt::skip]
    fn assoc_node_enter(&mut self, _node: &AssocNode<'sh>) {}
    #[rustfmt::skip]
    fn assoc_splat_node_enter(&mut self, _node: &AssocSplatNode<'sh>) {}
    #[rustfmt::skip]
    fn back_reference_read_node_enter(&mut self, _node: &BackReferenceReadNode<'sh>) {}
    #[rustfmt::skip]
    fn begin_node_enter(&mut self, _node: &BeginNode<'sh>) {}
    #[rustfmt::skip]
    fn block_argument_node_enter(&mut self, _node: &BlockArgumentNode<'sh>) {}
    #[rustfmt::skip]
    fn block_local_variable_node_enter(&mut self, _node: &BlockLocalVariableNode<'sh>) {}
    #[rustfmt::skip]
    fn block_node_enter(&mut self, _node: &BlockNode<'sh>) {}
    #[rustfmt::skip]
    fn block_parameter_node_enter(&mut self, _node: &BlockParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn block_parameters_node_enter(&mut self, _node: &BlockParametersNode<'sh>) {}
    #[rustfmt::skip]
    fn break_node_enter(&mut self, _node: &BreakNode<'sh>) {}
    #[rustfmt::skip]
    fn call_and_write_node_enter(&mut self, _node: &CallAndWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn call_node_enter(&mut self, _node: &CallNode<'sh>) {}
    #[rustfmt::skip]
    fn call_operator_write_node_enter(&mut self, _node: &CallOperatorWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn call_or_write_node_enter(&mut self, _node: &CallOrWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn call_target_node_enter(&mut self, _node: &CallTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn capture_pattern_node_enter(&mut self, _node: &CapturePatternNode<'sh>) {}
    #[rustfmt::skip]
    fn case_match_node_enter(&mut self, _node: &CaseMatchNode<'sh>) {}
    #[rustfmt::skip]
    fn case_node_enter(&mut self, _node: &CaseNode<'sh>) {}
    #[rustfmt::skip]
    fn class_node_enter(&mut self, _node: &ClassNode<'sh>) {}
    #[rustfmt::skip]
    fn class_variable_and_write_node_enter(&mut self, _node: &ClassVariableAndWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn class_variable_operator_write_node_enter(&mut self, _node: &ClassVariableOperatorWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn class_variable_or_write_node_enter(&mut self, _node: &ClassVariableOrWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn class_variable_read_node_enter(&mut self, _node: &ClassVariableReadNode<'sh>) {}
    #[rustfmt::skip]
    fn class_variable_target_node_enter(&mut self, _node: &ClassVariableTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn class_variable_write_node_enter(&mut self, _node: &ClassVariableWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_and_write_node_enter(&mut self, _node: &ConstantAndWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_operator_write_node_enter(&mut self, _node: &ConstantOperatorWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_or_write_node_enter(&mut self, _node: &ConstantOrWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_path_and_write_node_enter(&mut self, _node: &ConstantPathAndWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_path_node_enter(&mut self, _node: &ConstantPathNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_path_operator_write_node_enter(&mut self, _node: &ConstantPathOperatorWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_path_or_write_node_enter(&mut self, _node: &ConstantPathOrWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_path_target_node_enter(&mut self, _node: &ConstantPathTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_path_write_node_enter(&mut self, _node: &ConstantPathWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_read_node_enter(&mut self, _node: &ConstantReadNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_target_node_enter(&mut self, _node: &ConstantTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn constant_write_node_enter(&mut self, _node: &ConstantWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn def_node_enter(&mut self, _node: &DefNode<'sh>) {}
    #[rustfmt::skip]
    fn defined_node_enter(&mut self, _node: &DefinedNode<'sh>) {}
    #[rustfmt::skip]
    fn else_node_enter(&mut self, _node: &ElseNode<'sh>) {}
    #[rustfmt::skip]
    fn embedded_statements_node_enter(&mut self, _node: &EmbeddedStatementsNode<'sh>) {}
    #[rustfmt::skip]
    fn embedded_variable_node_enter(&mut self, _node: &EmbeddedVariableNode<'sh>) {}
    #[rustfmt::skip]
    fn ensure_node_enter(&mut self, _node: &EnsureNode<'sh>) {}
    #[rustfmt::skip]
    fn false_node_enter(&mut self, _node: &FalseNode<'sh>) {}
    #[rustfmt::skip]
    fn find_pattern_node_enter(&mut self, _node: &FindPatternNode<'sh>) {}
    #[rustfmt::skip]
    fn flip_flop_node_enter(&mut self, _node: &FlipFlopNode<'sh>) {}
    #[rustfmt::skip]
    fn float_node_enter(&mut self, _node: &FloatNode<'sh>) {}
    #[rustfmt::skip]
    fn for_node_enter(&mut self, _node: &ForNode<'sh>) {}
    #[rustfmt::skip]
    fn forwarding_arguments_node_enter(&mut self, _node: &ForwardingArgumentsNode<'sh>) {}
    #[rustfmt::skip]
    fn forwarding_parameter_node_enter(&mut self, _node: &ForwardingParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn forwarding_super_node_enter(&mut self, _node: &ForwardingSuperNode<'sh>) {}
    #[rustfmt::skip]
    fn global_variable_and_write_node_enter(&mut self, _node: &GlobalVariableAndWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn global_variable_operator_write_node_enter(&mut self, _node: &GlobalVariableOperatorWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn global_variable_or_write_node_enter(&mut self, _node: &GlobalVariableOrWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn global_variable_read_node_enter(&mut self, _node: &GlobalVariableReadNode<'sh>) {}
    #[rustfmt::skip]
    fn global_variable_target_node_enter(&mut self, _node: &GlobalVariableTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn global_variable_write_node_enter(&mut self, _node: &GlobalVariableWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn hash_node_enter(&mut self, _node: &HashNode<'sh>) {}
    #[rustfmt::skip]
    fn hash_pattern_node_enter(&mut self, _node: &HashPatternNode<'sh>) {}
    #[rustfmt::skip]
    fn if_node_enter(&mut self, _node: &IfNode<'sh>) {}
    #[rustfmt::skip]
    fn imaginary_node_enter(&mut self, _node: &ImaginaryNode<'sh>) {}
    #[rustfmt::skip]
    fn implicit_node_enter(&mut self, _node: &ImplicitNode<'sh>) {}
    #[rustfmt::skip]
    fn implicit_rest_node_enter(&mut self, _node: &ImplicitRestNode<'sh>) {}
    #[rustfmt::skip]
    fn in_node_enter(&mut self, _node: &InNode<'sh>) {}
    #[rustfmt::skip]
    fn index_and_write_node_enter(&mut self, _node: &IndexAndWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn index_operator_write_node_enter(&mut self, _node: &IndexOperatorWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn index_or_write_node_enter(&mut self, _node: &IndexOrWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn index_target_node_enter(&mut self, _node: &IndexTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn instance_variable_and_write_node_enter(&mut self, _node: &InstanceVariableAndWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn instance_variable_operator_write_node_enter(&mut self, _node: &InstanceVariableOperatorWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn instance_variable_or_write_node_enter(&mut self, _node: &InstanceVariableOrWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn instance_variable_read_node_enter(&mut self, _node: &InstanceVariableReadNode<'sh>) {}
    #[rustfmt::skip]
    fn instance_variable_target_node_enter(&mut self, _node: &InstanceVariableTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn instance_variable_write_node_enter(&mut self, _node: &InstanceVariableWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn integer_node_enter(&mut self, _node: &IntegerNode<'sh>) {}
    #[rustfmt::skip]
    fn interpolated_match_last_line_node_enter(&mut self, _node: &InterpolatedMatchLastLineNode<'sh>) {}
    #[rustfmt::skip]
    fn interpolated_regular_expression_node_enter(&mut self, _node: &InterpolatedRegularExpressionNode<'sh>) {}
    #[rustfmt::skip]
    fn interpolated_string_node_enter(&mut self, _node: &InterpolatedStringNode<'sh>) {}
    #[rustfmt::skip]
    fn interpolated_symbol_node_enter(&mut self, _node: &InterpolatedSymbolNode<'sh>) {}
    #[rustfmt::skip]
    fn interpolated_x_string_node_enter(&mut self, _node: &InterpolatedXStringNode<'sh>) {}
    #[rustfmt::skip]
    fn it_local_variable_read_node_enter(&mut self, _node: &ItLocalVariableReadNode<'sh>) {}
    #[rustfmt::skip]
    fn it_parameters_node_enter(&mut self, _node: &ItParametersNode<'sh>) {}
    #[rustfmt::skip]
    fn keyword_hash_node_enter(&mut self, _node: &KeywordHashNode<'sh>) {}
    #[rustfmt::skip]
    fn keyword_rest_parameter_node_enter(&mut self, _node: &KeywordRestParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn lambda_node_enter(&mut self, _node: &LambdaNode<'sh>) {}
    #[rustfmt::skip]
    fn local_variable_and_write_node_enter(&mut self, _node: &LocalVariableAndWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn local_variable_operator_write_node_enter(&mut self, _node: &LocalVariableOperatorWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn local_variable_or_write_node_enter(&mut self, _node: &LocalVariableOrWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn local_variable_read_node_enter(&mut self, _node: &LocalVariableReadNode<'sh>) {}
    #[rustfmt::skip]
    fn local_variable_target_node_enter(&mut self, _node: &LocalVariableTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn local_variable_write_node_enter(&mut self, _node: &LocalVariableWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn match_last_line_node_enter(&mut self, _node: &MatchLastLineNode<'sh>) {}
    #[rustfmt::skip]
    fn match_predicate_node_enter(&mut self, _node: &MatchPredicateNode<'sh>) {}
    #[rustfmt::skip]
    fn match_required_node_enter(&mut self, _node: &MatchRequiredNode<'sh>) {}
    #[rustfmt::skip]
    fn match_write_node_enter(&mut self, _node: &MatchWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn missing_node_enter(&mut self, _node: &MissingNode<'sh>) {}
    #[rustfmt::skip]
    fn module_node_enter(&mut self, _node: &ModuleNode<'sh>) {}
    #[rustfmt::skip]
    fn multi_target_node_enter(&mut self, _node: &MultiTargetNode<'sh>) {}
    #[rustfmt::skip]
    fn multi_write_node_enter(&mut self, _node: &MultiWriteNode<'sh>) {}
    #[rustfmt::skip]
    fn next_node_enter(&mut self, _node: &NextNode<'sh>) {}
    #[rustfmt::skip]
    fn nil_node_enter(&mut self, _node: &NilNode<'sh>) {}
    #[rustfmt::skip]
    fn no_keywords_parameter_node_enter(&mut self, _node: &NoKeywordsParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn numbered_parameters_node_enter(&mut self, _node: &NumberedParametersNode<'sh>) {}
    #[rustfmt::skip]
    fn numbered_reference_read_node_enter(&mut self, _node: &NumberedReferenceReadNode<'sh>) {}
    #[rustfmt::skip]
    fn optional_keyword_parameter_node_enter(&mut self, _node: &OptionalKeywordParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn optional_parameter_node_enter(&mut self, _node: &OptionalParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn or_node_enter(&mut self, _node: &OrNode<'sh>) {}
    #[rustfmt::skip]
    fn parameters_node_enter(&mut self, _node: &ParametersNode<'sh>) {}
    #[rustfmt::skip]
    fn parentheses_node_enter(&mut self, _node: &ParenthesesNode<'sh>) {}
    #[rustfmt::skip]
    fn pinned_expression_node_enter(&mut self, _node: &PinnedExpressionNode<'sh>) {}
    #[rustfmt::skip]
    fn pinned_variable_node_enter(&mut self, _node: &PinnedVariableNode<'sh>) {}
    #[rustfmt::skip]
    fn post_execution_node_enter(&mut self, _node: &PostExecutionNode<'sh>) {}
    #[rustfmt::skip]
    fn pre_execution_node_enter(&mut self, _node: &PreExecutionNode<'sh>) {}
    #[rustfmt::skip]
    fn program_node_enter(&mut self, _node: &ProgramNode<'sh>) {}
    #[rustfmt::skip]
    fn range_node_enter(&mut self, _node: &RangeNode<'sh>) {}
    #[rustfmt::skip]
    fn rational_node_enter(&mut self, _node: &RationalNode<'sh>) {}
    #[rustfmt::skip]
    fn redo_node_enter(&mut self, _node: &RedoNode<'sh>) {}
    #[rustfmt::skip]
    fn regular_expression_node_enter(&mut self, _node: &RegularExpressionNode<'sh>) {}
    #[rustfmt::skip]
    fn required_keyword_parameter_node_enter(&mut self, _node: &RequiredKeywordParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn required_parameter_node_enter(&mut self, _node: &RequiredParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn rescue_modifier_node_enter(&mut self, _node: &RescueModifierNode<'sh>) {}
    #[rustfmt::skip]
    fn rescue_node_enter(&mut self, _node: &RescueNode<'sh>) {}
    #[rustfmt::skip]
    fn rest_parameter_node_enter(&mut self, _node: &RestParameterNode<'sh>) {}
    #[rustfmt::skip]
    fn retry_node_enter(&mut self, _node: &RetryNode<'sh>) {}
    #[rustfmt::skip]
    fn return_node_enter(&mut self, _node: &ReturnNode<'sh>) {}
    #[rustfmt::skip]
    fn self_node_enter(&mut self, _node: &SelfNode<'sh>) {}
    #[rustfmt::skip]
    fn shareable_constant_node_enter(&mut self, _node: &ShareableConstantNode<'sh>) {}
    #[rustfmt::skip]
    fn singleton_class_node_enter(&mut self, _node: &SingletonClassNode<'sh>) {}
    #[rustfmt::skip]
    fn source_encoding_node_enter(&mut self, _node: &SourceEncodingNode<'sh>) {}
    #[rustfmt::skip]
    fn source_file_node_enter(&mut self, _node: &SourceFileNode<'sh>) {}
    #[rustfmt::skip]
    fn source_line_node_enter(&mut self, _node: &SourceLineNode<'sh>) {}
    #[rustfmt::skip]
    fn splat_node_enter(&mut self, _node: &SplatNode<'sh>) {}
    #[rustfmt::skip]
    fn statements_node_enter(&mut self, _node: &StatementsNode<'sh>) {}
    #[rustfmt::skip]
    fn string_node_enter(&mut self, _node: &StringNode<'sh>) {}
    #[rustfmt::skip]
    fn super_node_enter(&mut self, _node: &SuperNode<'sh>) {}
    #[rustfmt::skip]
    fn symbol_node_enter(&mut self, _node: &SymbolNode<'sh>) {}
    #[rustfmt::skip]
    fn true_node_enter(&mut self, _node: &TrueNode<'sh>) {}
    #[rustfmt::skip]
    fn undef_node_enter(&mut self, _node: &UndefNode<'sh>) {}
    #[rustfmt::skip]
    fn unless_node_enter(&mut self, _node: &UnlessNode<'sh>) {}
    #[rustfmt::skip]
    fn until_node_enter(&mut self, _node: &UntilNode<'sh>) {}
    #[rustfmt::skip]
    fn when_node_enter(&mut self, _node: &WhenNode<'sh>) {}
    #[rustfmt::skip]
    fn while_node_enter(&mut self, _node: &WhileNode<'sh>) {}
    #[rustfmt::skip]
    fn x_string_node_enter(&mut self, _node: &XStringNode<'sh>) {}
    #[rustfmt::skip]
    fn yield_node_enter(&mut self, _node: &YieldNode<'sh>) {}
    // visit each node methods
    #[rustfmt::skip]
    fn visit_alias_global_variable_node(&mut self, node: &AliasGlobalVariableNode<'sh>) {
        self.alias_global_variable_node_enter(node);
        self.visit(&node.new_name());
        self.visit(&node.old_name());
    }
    #[rustfmt::skip]
    fn visit_alias_method_node(&mut self, node: &AliasMethodNode<'sh>) {
        self.alias_method_node_enter(node);
        self.visit(&node.new_name());
        self.visit(&node.old_name());
    }
    #[rustfmt::skip]
    fn visit_alternation_pattern_node(&mut self, node: &AlternationPatternNode<'sh>) {
        self.alternation_pattern_node_enter(node);
        self.visit(&node.left());
        self.visit(&node.right());
    }
    #[rustfmt::skip]
    fn visit_and_node(&mut self, node: &AndNode<'sh>) {
        self.and_node_enter(node);
        self.visit(&node.left());
        self.visit(&node.right());
    }
    #[rustfmt::skip]
    fn visit_arguments_node(&mut self, node: &ArgumentsNode<'sh>) {
        self.arguments_node_enter(node);
        for node in &node.arguments() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_array_node(&mut self, node: &ArrayNode<'sh>) {
        self.array_node_enter(node);
        for node in &node.elements() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_array_pattern_node(&mut self, node: &ArrayPatternNode<'sh>) {
        self.array_pattern_node_enter(node);
        if let Some(node) = &node.constant() {
            self.visit(&node);
        }
        for node in &node.requireds() {
            self.visit(&node);
        }
        if let Some(node) = &node.rest() {
            self.visit(&node);
        }
        for node in &node.posts() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_assoc_node(&mut self, node: &AssocNode<'sh>) {
        self.assoc_node_enter(node);
        self.visit(&node.key());
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_assoc_splat_node(&mut self, node: &AssocSplatNode<'sh>) {
        self.assoc_splat_node_enter(node);
        if let Some(node) = &node.value() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_back_reference_read_node(&mut self, node: &BackReferenceReadNode<'sh>) {
        self.back_reference_read_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_begin_node(&mut self, node: &BeginNode<'sh>) {
        self.begin_node_enter(node);
        if let Some(node) = &node.statements() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.rescue_clause() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.else_clause() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.ensure_clause() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_block_argument_node(&mut self, node: &BlockArgumentNode<'sh>) {
        self.block_argument_node_enter(node);
        if let Some(node) = &node.expression() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_block_local_variable_node(&mut self, node: &BlockLocalVariableNode<'sh>) {
        self.block_local_variable_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_block_node(&mut self, node: &BlockNode<'sh>) {
        self.block_node_enter(node);
        if let Some(node) = &node.parameters() {
            self.visit(&node);
        }
        if let Some(node) = &node.body() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_block_parameter_node(&mut self, node: &BlockParameterNode<'sh>) {
        self.block_parameter_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_block_parameters_node(&mut self, node: &BlockParametersNode<'sh>) {
        self.block_parameters_node_enter(node);
        if let Some(node) = &node.parameters() {
            self.visit(&node.as_node());
        }
        for node in &node.locals() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_break_node(&mut self, node: &BreakNode<'sh>) {
        self.break_node_enter(node);
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_call_and_write_node(&mut self, node: &CallAndWriteNode<'sh>) {
        self.call_and_write_node_enter(node);
        if let Some(node) = &node.receiver() {
            self.visit(&node);
        }
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_call_node(&mut self, node: &CallNode<'sh>) {
        self.call_node_enter(node);
        if let Some(node) = &node.receiver() {
            self.visit(&node);
        }
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
        if let Some(node) = node.block() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_call_operator_write_node(&mut self, node: &CallOperatorWriteNode<'sh>) {
        self.call_operator_write_node_enter(node);
        if let Some(node) = &node.receiver() {
            self.visit(&node);
        }
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_call_or_write_node(&mut self, node: &CallOrWriteNode<'sh>) {
        self.call_or_write_node_enter(node);
        if let Some(node) = &node.receiver() {
            self.visit(&node);
        }
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_call_target_node(&mut self, node: &CallTargetNode<'sh>) {
        self.call_target_node_enter(node);
        self.visit(&node.receiver());
    }
    #[rustfmt::skip]
    fn visit_capture_pattern_node(&mut self, node: &CapturePatternNode<'sh>) {
        self.capture_pattern_node_enter(node);
        self.visit(&node.value());
        self.visit(&node.target().as_node());
    }
    #[rustfmt::skip]
    fn visit_case_match_node(&mut self, node: &CaseMatchNode<'sh>) {
        self.case_match_node_enter(node);
        if let Some(node) = &node.predicate() {
            self.visit(&node);
        }
        for node in &node.conditions() {
            self.visit(&node);
        }
        if let Some(node) = &node.else_clause() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_case_node(&mut self, node: &CaseNode<'sh>) {
        self.case_node_enter(node);
        if let Some(node) = &node.predicate() {
            self.visit(&node);
        }
        for node in &node.conditions() {
            self.visit(&node);
        }
        if let Some(node) = &node.else_clause() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_class_node(&mut self, node: &ClassNode<'sh>) {
        self.class_node_enter(node);
        self.visit(&node.constant_path());
        if let Some(node) = &node.superclass() {
            self.visit(&node);
        }
        if let Some(node) = &node.body() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_class_variable_and_write_node(&mut self, node: &ClassVariableAndWriteNode<'sh>) {
        self.class_variable_and_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_class_variable_operator_write_node(&mut self, node: &ClassVariableOperatorWriteNode<'sh>) {
        self.class_variable_operator_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_class_variable_or_write_node(&mut self, node: &ClassVariableOrWriteNode<'sh>) {
        self.class_variable_or_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_class_variable_read_node(&mut self, node: &ClassVariableReadNode<'sh>) {
        self.class_variable_read_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_class_variable_target_node(&mut self, node: &ClassVariableTargetNode<'sh>) {
        self.class_variable_target_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_class_variable_write_node(&mut self, node: &ClassVariableWriteNode<'sh>) {
        self.class_variable_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_constant_and_write_node(&mut self, node: &ConstantAndWriteNode<'sh>) {
        self.constant_and_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_constant_operator_write_node(&mut self, node: &ConstantOperatorWriteNode<'sh>) {
        self.constant_operator_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_constant_or_write_node(&mut self, node: &ConstantOrWriteNode<'sh>) {
        self.constant_or_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_constant_path_and_write_node(&mut self, node: &ConstantPathAndWriteNode<'sh>) {
        self.constant_path_and_write_node_enter(node);
        self.visit(&node.target().as_node());
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_constant_path_node(&mut self, node: &ConstantPathNode<'sh>) {
        self.constant_path_node_enter(node);
        if let Some(node) = &node.parent() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_constant_path_operator_write_node(&mut self, node: &ConstantPathOperatorWriteNode<'sh>) {
        self.constant_path_operator_write_node_enter(node);
        self.visit(&node.target().as_node());
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_constant_path_or_write_node(&mut self, node: &ConstantPathOrWriteNode<'sh>) {
        self.constant_path_or_write_node_enter(node);
        self.visit(&node.target().as_node());
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_constant_path_target_node(&mut self, node: &ConstantPathTargetNode<'sh>) {
        self.constant_path_target_node_enter(node);
        if let Some(node) = &node.parent() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_constant_path_write_node(&mut self, node: &ConstantPathWriteNode<'sh>) {
        self.constant_path_write_node_enter(node);
        self.visit(&node.target().as_node());
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_constant_read_node(&mut self, node: &ConstantReadNode<'sh>) {
        self.constant_read_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_constant_target_node(&mut self, node: &ConstantTargetNode<'sh>) {
        self.constant_target_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_constant_write_node(&mut self, node: &ConstantWriteNode<'sh>) {
        self.constant_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_def_node(&mut self, node: &DefNode<'sh>) {
        self.def_node_enter(node);
        if let Some(node) = &node.receiver() {
            self.visit(&node);
        }
        if let Some(node) = &node.parameters() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.body() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_defined_node(&mut self, node: &DefinedNode<'sh>) {
        self.defined_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_else_node(&mut self, node: &ElseNode<'sh>) {
        self.else_node_enter(node);
        if let Some(node) = &node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_embedded_statements_node(&mut self, node: &EmbeddedStatementsNode<'sh>) {
        self.embedded_statements_node_enter(node);
        if let Some(node) = &node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_embedded_variable_node(&mut self, node: &EmbeddedVariableNode<'sh>) {
        self.embedded_variable_node_enter(node);
        self.visit(&node.variable());
    }
    #[rustfmt::skip]
    fn visit_ensure_node(&mut self, node: &EnsureNode<'sh>) {
        self.ensure_node_enter(node);
        if let Some(node) = &node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_false_node(&mut self, node: &FalseNode<'sh>) {
        self.false_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_find_pattern_node(&mut self, node: &FindPatternNode<'sh>) {
        self.find_pattern_node_enter(node);
        if let Some(node) = &node.constant() {
            self.visit(&node);
        }
        self.visit(&node.left().as_node());
        for node in &node.requireds() {
            self.visit(&node);
        }
        self.visit(&node.right());
    }
    #[rustfmt::skip]
    fn visit_flip_flop_node(&mut self, node: &FlipFlopNode<'sh>) {
        self.flip_flop_node_enter(node);
        if let Some(node) = &node.left() {
            self.visit(&node);
        }
        if let Some(node) = &node.right() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_float_node(&mut self, node: &FloatNode<'sh>) {
        self.float_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_for_node(&mut self, node: &ForNode<'sh>) {
        self.for_node_enter(node);
        self.visit(&node.index());
        self.visit(&node.collection());
        if let Some(node) = node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_forwarding_arguments_node(&mut self, node: &ForwardingArgumentsNode<'sh>) {
        self.forwarding_arguments_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_forwarding_parameter_node(&mut self, node: &ForwardingParameterNode<'sh>) {
        self.forwarding_parameter_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_forwarding_super_node(&mut self, node: &ForwardingSuperNode<'sh>) {
        self.forwarding_super_node_enter(node);
        if let Some(node) = node.block() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_global_variable_and_write_node(&mut self, node: &GlobalVariableAndWriteNode<'sh>) {
        self.global_variable_and_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_global_variable_operator_write_node(&mut self, node: &GlobalVariableOperatorWriteNode<'sh>) {
        self.global_variable_operator_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_global_variable_or_write_node(&mut self, node: &GlobalVariableOrWriteNode<'sh>) {
        self.global_variable_or_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_global_variable_read_node(&mut self, node: &GlobalVariableReadNode<'sh>) {
        self.global_variable_read_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_global_variable_target_node(&mut self, node: &GlobalVariableTargetNode<'sh>) {
        self.global_variable_target_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_global_variable_write_node(&mut self, node: &GlobalVariableWriteNode<'sh>) {
        self.global_variable_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_hash_node(&mut self, node: &HashNode<'sh>) {
        self.hash_node_enter(node);
        for node in &node.elements() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_hash_pattern_node(&mut self, node: &HashPatternNode<'sh>) {
        self.hash_pattern_node_enter(node);
        if let Some(node) = node.constant() {
            self.visit(&node);
        }
        for node in &node.elements() {
            self.visit(&node);
        }
        if let Some(node) = node.rest() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_if_node(&mut self, node: &IfNode<'sh>) {
        self.if_node_enter(node);
        self.visit(&node.predicate());
        if let Some(node) = &node.statements() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.subsequent() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_imaginary_node(&mut self, node: &ImaginaryNode<'sh>) {
        self.imaginary_node_enter(node);
        self.visit(&node.numeric());
    }
    #[rustfmt::skip]
    fn visit_implicit_node(&mut self, node: &ImplicitNode<'sh>) {
        self.implicit_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_implicit_rest_node(&mut self, node: &ImplicitRestNode<'sh>) {
        self.implicit_rest_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_in_node(&mut self, node: &InNode<'sh>) {
        self.in_node_enter(node);
        self.visit(&node.pattern());
        if let Some(node) = node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_index_and_write_node(&mut self, node: &IndexAndWriteNode<'sh>) {
        self.index_and_write_node_enter(node);
        if let Some(node) = &node.receiver() {
            self.visit(&node);
        }
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.block() {
            self.visit(&node.as_node());
        }
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_index_operator_write_node(&mut self, node: &IndexOperatorWriteNode<'sh>) {
        self.index_operator_write_node_enter(node);
        if let Some(node) = &node.receiver() {
            self.visit(&node);
        }
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.block() {
            self.visit(&node.as_node());
        }
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_index_or_write_node(&mut self, node: &IndexOrWriteNode<'sh>) {
        self.index_or_write_node_enter(node);
        if let Some(node) = &node.receiver() {
            self.visit(&node);
        }
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.block() {
            self.visit(&node.as_node());
        }
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_index_target_node(&mut self, node: &IndexTargetNode<'sh>) {
        self.index_target_node_enter(node);
        self.visit(&node.receiver());
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.block() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_instance_variable_and_write_node(&mut self, node: &InstanceVariableAndWriteNode<'sh>) {
        self.instance_variable_and_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_instance_variable_operator_write_node(&mut self, node: &InstanceVariableOperatorWriteNode<'sh>) {
        self.instance_variable_operator_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_instance_variable_or_write_node(&mut self, node: &InstanceVariableOrWriteNode<'sh>) {
        self.instance_variable_or_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_instance_variable_read_node(&mut self, node: &InstanceVariableReadNode<'sh>) {
        self.instance_variable_read_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_instance_variable_target_node(&mut self, node: &InstanceVariableTargetNode<'sh>) {
        self.instance_variable_target_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_instance_variable_write_node(&mut self, node: &InstanceVariableWriteNode<'sh>) {
        self.instance_variable_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_integer_node(&mut self, node: &IntegerNode<'sh>) {
        self.integer_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_interpolated_match_last_line_node(&mut self, node: &InterpolatedMatchLastLineNode<'sh>) {
        self.interpolated_match_last_line_node_enter(node);
        for node in &node.parts() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_interpolated_regular_expression_node(&mut self, node: &InterpolatedRegularExpressionNode<'sh>) {
        self.interpolated_regular_expression_node_enter(node);
        for node in &node.parts() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_interpolated_string_node(&mut self, node: &InterpolatedStringNode<'sh>) {
        self.interpolated_string_node_enter(node);
        for node in &node.parts() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_interpolated_symbol_node(&mut self, node: &InterpolatedSymbolNode<'sh>) {
        self.interpolated_symbol_node_enter(node);
        for node in &node.parts() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_interpolated_x_string_node(&mut self, node: &InterpolatedXStringNode<'sh>) {
        self.interpolated_x_string_node_enter(node);
        for node in &node.parts() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_it_local_variable_read_node(&mut self, node: &ItLocalVariableReadNode<'sh>) {
        self.it_local_variable_read_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_it_parameters_node(&mut self, node: &ItParametersNode<'sh>) {
        self.it_parameters_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_keyword_hash_node(&mut self, node: &KeywordHashNode<'sh>) {
        self.keyword_hash_node_enter(node);
        for node in &node.elements() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_keyword_rest_parameter_node(&mut self, node: &KeywordRestParameterNode<'sh>) {
        self.keyword_rest_parameter_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_lambda_node(&mut self, node: &LambdaNode<'sh>) {
        self.lambda_node_enter(node);
        if let Some(node) = &node.parameters() {
            self.visit(&node);
        }
        if let Some(node) = &node.body() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_local_variable_and_write_node(&mut self, node: &LocalVariableAndWriteNode<'sh>) {
        self.local_variable_and_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_local_variable_operator_write_node(&mut self, node: &LocalVariableOperatorWriteNode<'sh>) {
        self.local_variable_operator_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_local_variable_or_write_node(&mut self, node: &LocalVariableOrWriteNode<'sh>) {
        self.local_variable_or_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_local_variable_read_node(&mut self, node: &LocalVariableReadNode<'sh>) {
        self.local_variable_read_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_local_variable_target_node(&mut self, node: &LocalVariableTargetNode<'sh>) {
        self.local_variable_target_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_local_variable_write_node(&mut self, node: &LocalVariableWriteNode<'sh>) {
        self.local_variable_write_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_match_last_line_node(&mut self, node: &MatchLastLineNode<'sh>) {
        self.match_last_line_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_match_predicate_node(&mut self, node: &MatchPredicateNode<'sh>) {
        self.match_predicate_node_enter(node);
        self.visit(&node.value());
        self.visit(&node.pattern());
    }
    #[rustfmt::skip]
    fn visit_match_required_node(&mut self, node: &MatchRequiredNode<'sh>) {
        self.match_required_node_enter(node);
        self.visit(&node.value());
        self.visit(&node.pattern());
    }
    #[rustfmt::skip]
    fn visit_match_write_node(&mut self, node: &MatchWriteNode<'sh>) {
        self.match_write_node_enter(node);
        self.visit(&node.call().as_node());
        for node in &node.targets() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_missing_node(&mut self, node: &MissingNode<'sh>) {
        self.missing_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_module_node(&mut self, node: &ModuleNode<'sh>) {
        self.module_node_enter(node);
        self.visit(&node.constant_path());
        if let Some(node) = &node.body() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_multi_target_node(&mut self, node: &MultiTargetNode<'sh>) {
        self.multi_target_node_enter(node);
        for node in &node.lefts() {
            self.visit(&node);
        }
        if let Some(node) = &node.rest() {
            self.visit(&node);
        }
        for node in &node.rights() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_multi_write_node(&mut self, node: &MultiWriteNode<'sh>) {
        self.multi_write_node_enter(node);
        for node in &node.lefts() {
            self.visit(&node);
        }
        if let Some(node) = &node.rest() {
            self.visit(&node);
        }
        for node in &node.rights() {
            self.visit(&node);
        }
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_next_node(&mut self, node: &NextNode<'sh>) {
        self.next_node_enter(node);
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_nil_node(&mut self, node: &NilNode<'sh>) {
        self.nil_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_no_keywords_parameter_node(&mut self, node: &NoKeywordsParameterNode<'sh>) {
        self.no_keywords_parameter_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_numbered_parameters_node(&mut self, node: &NumberedParametersNode<'sh>) {
        self.numbered_parameters_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_numbered_reference_read_node(&mut self, node: &NumberedReferenceReadNode<'sh>) {
        self.numbered_reference_read_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_optional_keyword_parameter_node(&mut self, node: &OptionalKeywordParameterNode<'sh>) {
        self.optional_keyword_parameter_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_optional_parameter_node(&mut self, node: &OptionalParameterNode<'sh>) {
        self.optional_parameter_node_enter(node);
        self.visit(&node.value());
    }
    #[rustfmt::skip]
    fn visit_or_node(&mut self, node: &OrNode<'sh>) {
        self.or_node_enter(node);
        self.visit(&node.left());
        self.visit(&node.right());
    }
    #[rustfmt::skip]
    fn visit_parameters_node(&mut self, node: &ParametersNode<'sh>) {
        self.parameters_node_enter(node);
        for node in &node.requireds() {
            self.visit(&node);
        }
        for node in &node.optionals() {
            self.visit(&node);
        }
        if let Some(node) = &node.rest() {
            self.visit(&node);
        }
        for node in &node.posts() {
            self.visit(&node);
        }
        for node in &node.keywords() {
            self.visit(&node);
        }
        if let Some(node) = &node.keyword_rest() {
            self.visit(&node);
        }
        if let Some(node) = &node.block() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_parentheses_node(&mut self, node: &ParenthesesNode<'sh>) {
        self.parentheses_node_enter(node);
        if let Some(node) = &node.body() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_pinned_expression_node(&mut self, node: &PinnedExpressionNode<'sh>) {
        self.pinned_expression_node_enter(node);
        self.visit(&node.expression());
    }
    #[rustfmt::skip]
    fn visit_pinned_variable_node(&mut self, node: &PinnedVariableNode<'sh>) {
        self.pinned_variable_node_enter(node);
        self.visit(&node.variable());
    }
    #[rustfmt::skip]
    fn visit_post_execution_node(&mut self, node: &PostExecutionNode<'sh>) {
        self.post_execution_node_enter(node);
        if let Some(node) = node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_pre_execution_node(&mut self, node: &PreExecutionNode<'sh>) {
        self.pre_execution_node_enter(node);
        if let Some(node) = node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_program_node(&mut self, node: &ProgramNode<'sh>) {
        self.program_node_enter(node);
        self.visit(&node.statements().as_node());
    }
    #[rustfmt::skip]
    fn visit_range_node(&mut self, node: &RangeNode<'sh>) {
        self.range_node_enter(node);
        if let Some(node) = &node.left() {
            self.visit(&node);
        }
        if let Some(node) = &node.right() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_rational_node(&mut self, node: &RationalNode<'sh>) {
        self.rational_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_redo_node(&mut self, node: &RedoNode<'sh>) {
        self.redo_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_regular_expression_node(&mut self, node: &RegularExpressionNode<'sh>) {
        self.regular_expression_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_required_keyword_parameter_node(&mut self, node: &RequiredKeywordParameterNode<'sh>) {
        self.required_keyword_parameter_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_required_parameter_node(&mut self, node: &RequiredParameterNode<'sh>) {
        self.required_parameter_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_rescue_modifier_node(&mut self, node: &RescueModifierNode<'sh>) {
        self.rescue_modifier_node_enter(node);
        self.visit(&node.expression());
        self.visit(&node.rescue_expression());
    }
    #[rustfmt::skip]
    fn visit_rescue_node(&mut self, node: &RescueNode<'sh>) {
        self.rescue_node_enter(node);
        for node in &node.exceptions() {
            self.visit(&node);
        }
        if let Some(node) = &node.reference() {
            self.visit(&node);
        }
        if let Some(node) = &node.statements() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.subsequent() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_rest_parameter_node(&mut self, node: &RestParameterNode<'sh>) {
        self.rest_parameter_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_retry_node(&mut self, node: &RetryNode<'sh>) {
        self.retry_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_return_node(&mut self, node: &ReturnNode<'sh>) {
        self.return_node_enter(node);
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_self_node(&mut self, node: &SelfNode<'sh>) {
        self.self_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_shareable_constant_node(&mut self, node: &ShareableConstantNode<'sh>) {
        self.shareable_constant_node_enter(node);
        self.visit(&node.write());
    }
    #[rustfmt::skip]
    fn visit_singleton_class_node(&mut self, node: &SingletonClassNode<'sh>) {
        self.singleton_class_node_enter(node);
        self.visit(&node.expression());
        if let Some(node) = &node.body() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_source_encoding_node(&mut self, node: &SourceEncodingNode<'sh>) {
        self.source_encoding_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_source_file_node(&mut self, node: &SourceFileNode<'sh>) {
        self.source_file_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_source_line_node(&mut self, node: &SourceLineNode<'sh>) {
        self.source_line_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_splat_node(&mut self, node: &SplatNode<'sh>) {
        self.splat_node_enter(node);
        if let Some(node) = &node.expression() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_statements_node(&mut self, node: &StatementsNode<'sh>) {
        self.statements_node_enter(node);
        for node in &node.body() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_string_node(&mut self, node: &StringNode<'sh>) {
        self.string_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_super_node(&mut self, node: &SuperNode<'sh>) {
        self.super_node_enter(node);
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
        if let Some(node) = node.block() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_symbol_node(&mut self, node: &SymbolNode<'sh>) {
        self.symbol_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_true_node(&mut self, node: &TrueNode<'sh>) {
        self.true_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_undef_node(&mut self, node: &UndefNode<'sh>) {
        self.undef_node_enter(node);
        for node in &node.names() {
            self.visit(&node);
        }
    }
    #[rustfmt::skip]
    fn visit_unless_node(&mut self, node: &UnlessNode<'sh>) {
        self.unless_node_enter(node);
        self.visit(&node.predicate());
        if let Some(node) = &node.statements() {
            self.visit(&node.as_node());
        }
        if let Some(node) = &node.else_clause() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_until_node(&mut self, node: &UntilNode<'sh>) {
        self.until_node_enter(node);
        self.visit(&node.predicate());
        if let Some(node) = node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_when_node(&mut self, node: &WhenNode<'sh>) {
        self.when_node_enter(node);
        for node in &node.conditions() {
            self.visit(&node);
        }
        if let Some(node) = &node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_while_node(&mut self, node: &WhileNode<'sh>) {
        self.while_node_enter(node);
        self.visit(&node.predicate());
        if let Some(node) = node.statements() {
            self.visit(&node.as_node());
        }
    }
    #[rustfmt::skip]
    fn visit_x_string_node(&mut self, node: &XStringNode<'sh>) {
        self.x_string_node_enter(node);
    }
    #[rustfmt::skip]
    fn visit_yield_node(&mut self, node: &YieldNode<'sh>) {
        self.yield_node_enter(node);
        if let Some(node) = &node.arguments() {
            self.visit(&node.as_node());
        }
    }
}
