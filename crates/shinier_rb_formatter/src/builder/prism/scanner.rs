use ruby_prism::*;

#[rustfmt::skip]
pub fn scan(node: &Node, func: &mut impl FnMut(&Node)) {
    func(node);
    match node {
        Node::AliasGlobalVariableNode           { .. } => scan_alias_global_variable_node           (&node.as_alias_global_variable_node().unwrap()           , func),
        Node::AliasMethodNode                   { .. } => scan_alias_method_node                    (&node.as_alias_method_node().unwrap()                    , func),
        Node::AlternationPatternNode            { .. } => scan_alternation_pattern_node             (&node.as_alternation_pattern_node().unwrap()             , func),
        Node::AndNode                           { .. } => scan_and_node                             (&node.as_and_node().unwrap()                             , func),
        Node::ArgumentsNode                     { .. } => scan_arguments_node                       (&node.as_arguments_node().unwrap()                       , func),
        Node::ArrayNode                         { .. } => scan_array_node                           (&node.as_array_node().unwrap()                           , func),
        Node::ArrayPatternNode                  { .. } => scan_array_pattern_node                   (&node.as_array_pattern_node().unwrap()                   , func),
        Node::AssocNode                         { .. } => scan_assoc_node                           (&node.as_assoc_node().unwrap()                           , func),
        Node::AssocSplatNode                    { .. } => scan_assoc_splat_node                     (&node.as_assoc_splat_node().unwrap()                     , func),
        Node::BackReferenceReadNode             { .. } => scan_back_reference_read_node             (&node.as_back_reference_read_node().unwrap()             , func),
        Node::BeginNode                         { .. } => scan_begin_node                           (&node.as_begin_node().unwrap()                           , func),
        Node::BlockArgumentNode                 { .. } => scan_block_argument_node                  (&node.as_block_argument_node().unwrap()                  , func),
        Node::BlockLocalVariableNode            { .. } => scan_block_local_variable_node            (&node.as_block_local_variable_node().unwrap()            , func),
        Node::BlockNode                         { .. } => scan_block_node                           (&node.as_block_node().unwrap()                           , func),
        Node::BlockParameterNode                { .. } => scan_block_parameter_node                 (&node.as_block_parameter_node().unwrap()                 , func),
        Node::BlockParametersNode               { .. } => scan_block_parameters_node                (&node.as_block_parameters_node().unwrap()                , func),
        Node::BreakNode                         { .. } => scan_break_node                           (&node.as_break_node().unwrap()                           , func),
        Node::CallAndWriteNode                  { .. } => scan_call_and_write_node                  (&node.as_call_and_write_node().unwrap()                  , func),
        Node::CallNode                          { .. } => scan_call_node                            (&node.as_call_node().unwrap()                            , func),
        Node::CallOperatorWriteNode             { .. } => scan_call_operator_write_node             (&node.as_call_operator_write_node().unwrap()             , func),
        Node::CallOrWriteNode                   { .. } => scan_call_or_write_node                   (&node.as_call_or_write_node().unwrap()                   , func),
        Node::CallTargetNode                    { .. } => scan_call_target_node                     (&node.as_call_target_node().unwrap()                     , func),
        Node::CapturePatternNode                { .. } => scan_capture_pattern_node                 (&node.as_capture_pattern_node().unwrap()                 , func),
        Node::CaseMatchNode                     { .. } => scan_case_match_node                      (&node.as_case_match_node().unwrap()                      , func),
        Node::CaseNode                          { .. } => scan_case_node                            (&node.as_case_node().unwrap()                            , func),
        Node::ClassNode                         { .. } => scan_class_node                           (&node.as_class_node().unwrap()                           , func),
        Node::ClassVariableAndWriteNode         { .. } => scan_class_variable_and_write_node        (&node.as_class_variable_and_write_node().unwrap()        , func),
        Node::ClassVariableOperatorWriteNode    { .. } => scan_class_variable_operator_write_node   (&node.as_class_variable_operator_write_node().unwrap()   , func),
        Node::ClassVariableOrWriteNode          { .. } => scan_class_variable_or_write_node         (&node.as_class_variable_or_write_node().unwrap()         , func),
        Node::ClassVariableReadNode             { .. } => scan_class_variable_read_node             (&node.as_class_variable_read_node().unwrap()             , func),
        Node::ClassVariableTargetNode           { .. } => scan_class_variable_target_node           (&node.as_class_variable_target_node().unwrap()           , func),
        Node::ClassVariableWriteNode            { .. } => scan_class_variable_write_node            (&node.as_class_variable_write_node().unwrap()            , func),
        Node::ConstantAndWriteNode              { .. } => scan_constant_and_write_node              (&node.as_constant_and_write_node().unwrap()              , func),
        Node::ConstantOperatorWriteNode         { .. } => scan_constant_operator_write_node         (&node.as_constant_operator_write_node().unwrap()         , func),
        Node::ConstantOrWriteNode               { .. } => scan_constant_or_write_node               (&node.as_constant_or_write_node().unwrap()               , func),
        Node::ConstantPathAndWriteNode          { .. } => scan_constant_path_and_write_node         (&node.as_constant_path_and_write_node().unwrap()         , func),
        Node::ConstantPathNode                  { .. } => scan_constant_path_node                   (&node.as_constant_path_node().unwrap()                   , func),
        Node::ConstantPathOperatorWriteNode     { .. } => scan_constant_path_operator_write_node    (&node.as_constant_path_operator_write_node().unwrap()    , func),
        Node::ConstantPathOrWriteNode           { .. } => scan_constant_path_or_write_node          (&node.as_constant_path_or_write_node().unwrap()          , func),
        Node::ConstantPathTargetNode            { .. } => scan_constant_path_target_node            (&node.as_constant_path_target_node().unwrap()            , func),
        Node::ConstantPathWriteNode             { .. } => scan_constant_path_write_node             (&node.as_constant_path_write_node().unwrap()             , func),
        Node::ConstantReadNode                  { .. } => scan_constant_read_node                   (&node.as_constant_read_node().unwrap()                   , func),
        Node::ConstantTargetNode                { .. } => scan_constant_target_node                 (&node.as_constant_target_node().unwrap()                 , func),
        Node::ConstantWriteNode                 { .. } => scan_constant_write_node                  (&node.as_constant_write_node().unwrap()                  , func),
        Node::DefNode                           { .. } => scan_def_node                             (&node.as_def_node().unwrap()                             , func),
        Node::DefinedNode                       { .. } => scan_defined_node                         (&node.as_defined_node().unwrap()                         , func),
        Node::ElseNode                          { .. } => scan_else_node                            (&node.as_else_node().unwrap()                            , func),
        Node::EmbeddedStatementsNode            { .. } => scan_embedded_statements_node             (&node.as_embedded_statements_node().unwrap()             , func),
        Node::EmbeddedVariableNode              { .. } => scan_embedded_variable_node               (&node.as_embedded_variable_node().unwrap()               , func),
        Node::EnsureNode                        { .. } => scan_ensure_node                          (&node.as_ensure_node().unwrap()                          , func),
        Node::FalseNode                         { .. } => scan_false_node                           (&node.as_false_node().unwrap()                           , func),
        Node::FindPatternNode                   { .. } => scan_find_pattern_node                    (&node.as_find_pattern_node().unwrap()                    , func),
        Node::FlipFlopNode                      { .. } => scan_flip_flop_node                       (&node.as_flip_flop_node().unwrap()                       , func),
        Node::FloatNode                         { .. } => scan_float_node                           (&node.as_float_node().unwrap()                           , func),
        Node::ForNode                           { .. } => scan_for_node                             (&node.as_for_node().unwrap()                             , func),
        Node::ForwardingArgumentsNode           { .. } => scan_forwarding_arguments_node            (&node.as_forwarding_arguments_node().unwrap()            , func),
        Node::ForwardingParameterNode           { .. } => scan_forwarding_parameter_node            (&node.as_forwarding_parameter_node().unwrap()            , func),
        Node::ForwardingSuperNode               { .. } => scan_forwarding_super_node                (&node.as_forwarding_super_node().unwrap()                , func),
        Node::GlobalVariableAndWriteNode        { .. } => scan_global_variable_and_write_node       (&node.as_global_variable_and_write_node().unwrap()       , func),
        Node::GlobalVariableOperatorWriteNode   { .. } => scan_global_variable_operator_write_node  (&node.as_global_variable_operator_write_node().unwrap()  , func),
        Node::GlobalVariableOrWriteNode         { .. } => scan_global_variable_or_write_node        (&node.as_global_variable_or_write_node().unwrap()        , func),
        Node::GlobalVariableReadNode            { .. } => scan_global_variable_read_node            (&node.as_global_variable_read_node().unwrap()            , func),
        Node::GlobalVariableTargetNode          { .. } => scan_global_variable_target_node          (&node.as_global_variable_target_node().unwrap()          , func),
        Node::GlobalVariableWriteNode           { .. } => scan_global_variable_write_node           (&node.as_global_variable_write_node().unwrap()           , func),
        Node::HashNode                          { .. } => scan_hash_node                            (&node.as_hash_node().unwrap()                            , func),
        Node::HashPatternNode                   { .. } => scan_hash_pattern_node                    (&node.as_hash_pattern_node().unwrap()                    , func),
        Node::IfNode                            { .. } => scan_if_node                              (&node.as_if_node().unwrap()                              , func),
        Node::ImaginaryNode                     { .. } => scan_imaginary_node                       (&node.as_imaginary_node().unwrap()                       , func),
        Node::ImplicitNode                      { .. } => scan_implicit_node                        (&node.as_implicit_node().unwrap()                        , func),
        Node::ImplicitRestNode                  { .. } => scan_implicit_rest_node                   (&node.as_implicit_rest_node().unwrap()                   , func),
        Node::InNode                            { .. } => scan_in_node                              (&node.as_in_node().unwrap()                              , func),
        Node::IndexAndWriteNode                 { .. } => scan_index_and_write_node                 (&node.as_index_and_write_node().unwrap()                 , func),
        Node::IndexOperatorWriteNode            { .. } => scan_index_operator_write_node            (&node.as_index_operator_write_node().unwrap()            , func),
        Node::IndexOrWriteNode                  { .. } => scan_index_or_write_node                  (&node.as_index_or_write_node().unwrap()                  , func),
        Node::IndexTargetNode                   { .. } => scan_index_target_node                    (&node.as_index_target_node().unwrap()                    , func),
        Node::InstanceVariableAndWriteNode      { .. } => scan_instance_variable_and_write_node     (&node.as_instance_variable_and_write_node().unwrap()     , func),
        Node::InstanceVariableOperatorWriteNode { .. } => scan_instance_variable_operator_write_node(&node.as_instance_variable_operator_write_node().unwrap(), func),
        Node::InstanceVariableOrWriteNode       { .. } => scan_instance_variable_or_write_node      (&node.as_instance_variable_or_write_node().unwrap()      , func),
        Node::InstanceVariableReadNode          { .. } => scan_instance_variable_read_node          (&node.as_instance_variable_read_node().unwrap()          , func),
        Node::InstanceVariableTargetNode        { .. } => scan_instance_variable_target_node        (&node.as_instance_variable_target_node().unwrap()        , func),
        Node::InstanceVariableWriteNode         { .. } => scan_instance_variable_write_node         (&node.as_instance_variable_write_node().unwrap()         , func),
        Node::IntegerNode                       { .. } => scan_integer_node                         (&node.as_integer_node().unwrap()                         , func),
        Node::InterpolatedMatchLastLineNode     { .. } => scan_interpolated_match_last_line_node    (&node.as_interpolated_match_last_line_node().unwrap()    , func),
        Node::InterpolatedRegularExpressionNode { .. } => scan_interpolated_regular_expression_node (&node.as_interpolated_regular_expression_node().unwrap() , func),
        Node::InterpolatedStringNode            { .. } => scan_interpolated_string_node             (&node.as_interpolated_string_node().unwrap()             , func),
        Node::InterpolatedSymbolNode            { .. } => scan_interpolated_symbol_node             (&node.as_interpolated_symbol_node().unwrap()             , func),
        Node::InterpolatedXStringNode           { .. } => scan_interpolated_x_string_node           (&node.as_interpolated_x_string_node().unwrap()           , func),
        Node::ItLocalVariableReadNode           { .. } => scan_it_local_variable_read_node          (&node.as_it_local_variable_read_node().unwrap()          , func),
        Node::ItParametersNode                  { .. } => scan_it_parameters_node                   (&node.as_it_parameters_node().unwrap()                   , func),
        Node::KeywordHashNode                   { .. } => scan_keyword_hash_node                    (&node.as_keyword_hash_node().unwrap()                    , func),
        Node::KeywordRestParameterNode          { .. } => scan_keyword_rest_parameter_node          (&node.as_keyword_rest_parameter_node().unwrap()          , func),
        Node::LambdaNode                        { .. } => scan_lambda_node                          (&node.as_lambda_node().unwrap()                          , func),
        Node::LocalVariableAndWriteNode         { .. } => scan_local_variable_and_write_node        (&node.as_local_variable_and_write_node().unwrap()        , func),
        Node::LocalVariableOperatorWriteNode    { .. } => scan_local_variable_operator_write_node   (&node.as_local_variable_operator_write_node().unwrap()   , func),
        Node::LocalVariableOrWriteNode          { .. } => scan_local_variable_or_write_node         (&node.as_local_variable_or_write_node().unwrap()         , func),
        Node::LocalVariableReadNode             { .. } => scan_local_variable_read_node             (&node.as_local_variable_read_node().unwrap()             , func),
        Node::LocalVariableTargetNode           { .. } => scan_local_variable_target_node           (&node.as_local_variable_target_node().unwrap()           , func),
        Node::LocalVariableWriteNode            { .. } => scan_local_variable_write_node            (&node.as_local_variable_write_node().unwrap()            , func),
        Node::MatchLastLineNode                 { .. } => scan_match_last_line_node                 (&node.as_match_last_line_node().unwrap()                 , func),
        Node::MatchPredicateNode                { .. } => scan_match_predicate_node                 (&node.as_match_predicate_node().unwrap()                 , func),
        Node::MatchRequiredNode                 { .. } => scan_match_required_node                  (&node.as_match_required_node().unwrap()                  , func),
        Node::MatchWriteNode                    { .. } => scan_match_write_node                     (&node.as_match_write_node().unwrap()                     , func),
        Node::MissingNode                       { .. } => scan_missing_node                         (&node.as_missing_node().unwrap()                         , func),
        Node::ModuleNode                        { .. } => scan_module_node                          (&node.as_module_node().unwrap()                          , func),
        Node::MultiTargetNode                   { .. } => scan_multi_target_node                    (&node.as_multi_target_node().unwrap()                    , func),
        Node::MultiWriteNode                    { .. } => scan_multi_write_node                     (&node.as_multi_write_node().unwrap()                     , func),
        Node::NextNode                          { .. } => scan_next_node                            (&node.as_next_node().unwrap()                            , func),
        Node::NilNode                           { .. } => scan_nil_node                             (&node.as_nil_node().unwrap()                             , func),
        Node::NoKeywordsParameterNode           { .. } => scan_no_keywords_parameter_node           (&node.as_no_keywords_parameter_node().unwrap()           , func),
        Node::NumberedParametersNode            { .. } => scan_numbered_parameters_node             (&node.as_numbered_parameters_node().unwrap()             , func),
        Node::NumberedReferenceReadNode         { .. } => scan_numbered_reference_read_node         (&node.as_numbered_reference_read_node().unwrap()         , func),
        Node::OptionalKeywordParameterNode      { .. } => scan_optional_keyword_parameter_node      (&node.as_optional_keyword_parameter_node().unwrap()      , func),
        Node::OptionalParameterNode             { .. } => scan_optional_parameter_node              (&node.as_optional_parameter_node().unwrap()              , func),
        Node::OrNode                            { .. } => scan_or_node                              (&node.as_or_node().unwrap()                              , func),
        Node::ParametersNode                    { .. } => scan_parameters_node                      (&node.as_parameters_node().unwrap()                      , func),
        Node::ParenthesesNode                   { .. } => scan_parentheses_node                     (&node.as_parentheses_node().unwrap()                     , func),
        Node::PinnedExpressionNode              { .. } => scan_pinned_expression_node               (&node.as_pinned_expression_node().unwrap()               , func),
        Node::PinnedVariableNode                { .. } => scan_pinned_variable_node                 (&node.as_pinned_variable_node().unwrap()                 , func),
        Node::PostExecutionNode                 { .. } => scan_post_execution_node                  (&node.as_post_execution_node().unwrap()                  , func),
        Node::PreExecutionNode                  { .. } => scan_pre_execution_node                   (&node.as_pre_execution_node().unwrap()                   , func),
        Node::ProgramNode                       { .. } => scan_program_node                         (&node.as_program_node().unwrap()                         , func),
        Node::RangeNode                         { .. } => scan_range_node                           (&node.as_range_node().unwrap()                           , func),
        Node::RationalNode                      { .. } => scan_rational_node                        (&node.as_rational_node().unwrap()                        , func),
        Node::RedoNode                          { .. } => scan_redo_node                            (&node.as_redo_node().unwrap()                            , func),
        Node::RegularExpressionNode             { .. } => scan_regular_expression_node              (&node.as_regular_expression_node().unwrap()              , func),
        Node::RequiredKeywordParameterNode      { .. } => scan_required_keyword_parameter_node      (&node.as_required_keyword_parameter_node().unwrap()      , func),
        Node::RequiredParameterNode             { .. } => scan_required_parameter_node              (&node.as_required_parameter_node().unwrap()              , func),
        Node::RescueModifierNode                { .. } => scan_rescue_modifier_node                 (&node.as_rescue_modifier_node().unwrap()                 , func),
        Node::RescueNode                        { .. } => scan_rescue_node                          (&node.as_rescue_node().unwrap()                          , func),
        Node::RestParameterNode                 { .. } => scan_rest_parameter_node                  (&node.as_rest_parameter_node().unwrap()                  , func),
        Node::RetryNode                         { .. } => scan_retry_node                           (&node.as_retry_node().unwrap()                           , func),
        Node::ReturnNode                        { .. } => scan_return_node                          (&node.as_return_node().unwrap()                          , func),
        Node::SelfNode                          { .. } => scan_self_node                            (&node.as_self_node().unwrap()                            , func),
        Node::ShareableConstantNode             { .. } => scan_shareable_constant_node              (&node.as_shareable_constant_node().unwrap()              , func),
        Node::SingletonClassNode                { .. } => scan_singleton_class_node                 (&node.as_singleton_class_node().unwrap()                 , func),
        Node::SourceEncodingNode                { .. } => scan_source_encoding_node                 (&node.as_source_encoding_node().unwrap()                 , func),
        Node::SourceFileNode                    { .. } => scan_source_file_node                     (&node.as_source_file_node().unwrap()                     , func),
        Node::SourceLineNode                    { .. } => scan_source_line_node                     (&node.as_source_line_node().unwrap()                     , func),
        Node::SplatNode                         { .. } => scan_splat_node                           (&node.as_splat_node().unwrap()                           , func),
        Node::StatementsNode                    { .. } => scan_statements_node                      (&node.as_statements_node().unwrap()                      , func),
        Node::StringNode                        { .. } => scan_string_node                          (&node.as_string_node().unwrap()                          , func),
        Node::SuperNode                         { .. } => scan_super_node                           (&node.as_super_node().unwrap()                           , func),
        Node::SymbolNode                        { .. } => scan_symbol_node                          (&node.as_symbol_node().unwrap()                          , func),
        Node::TrueNode                          { .. } => scan_true_node                            (&node.as_true_node().unwrap()                            , func),
        Node::UndefNode                         { .. } => scan_undef_node                           (&node.as_undef_node().unwrap()                           , func),
        Node::UnlessNode                        { .. } => scan_unless_node                          (&node.as_unless_node().unwrap()                          , func),
        Node::UntilNode                         { .. } => scan_until_node                           (&node.as_until_node().unwrap()                           , func),
        Node::WhenNode                          { .. } => scan_when_node                            (&node.as_when_node().unwrap()                            , func),
        Node::WhileNode                         { .. } => scan_while_node                           (&node.as_while_node().unwrap()                           , func),
        Node::XStringNode                       { .. } => scan_x_string_node                        (&node.as_x_string_node().unwrap()                        , func),
        Node::YieldNode                         { .. } => scan_yield_node                           (&node.as_yield_node().unwrap()                           , func),
    }
}

#[rustfmt::skip]
fn scan_alias_global_variable_node(node: &AliasGlobalVariableNode, func: &mut impl FnMut(&Node)) {
    scan(&node.new_name(), func);
    scan(&node.old_name(), func);
}
#[rustfmt::skip]
fn scan_alias_method_node(node: &AliasMethodNode, func: &mut impl FnMut(&Node)) {
    scan(&node.new_name(), func);
    scan(&node.old_name(), func);
}
#[rustfmt::skip]
fn scan_alternation_pattern_node(node: &AlternationPatternNode, func: &mut impl FnMut(&Node)) {
    scan(&node.left(), func);
    scan(&node.right(), func);
}
#[rustfmt::skip]
fn scan_and_node(node: &AndNode, func: &mut impl FnMut(&Node)) {
    scan(&node.left(), func);
    scan(&node.right(), func);
}
#[rustfmt::skip]
fn scan_arguments_node(node: &ArgumentsNode, func: &mut impl FnMut(&Node)) {
    for node in &node.arguments() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_array_node(node: &ArrayNode, func: &mut impl FnMut(&Node)) {
    for node in &node.elements() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_array_pattern_node(node: &ArrayPatternNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.constant() {
        scan(&node, func);
    }
    for node in &node.requireds() {
        scan(&node, func);
    }
    if let Some(node) = &node.rest() {
        scan(&node, func);
    }
    for node in &node.posts() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_assoc_node(node: &AssocNode, func: &mut impl FnMut(&Node)) {
    scan(&node.key(), func);
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_assoc_splat_node(node: &AssocSplatNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.value() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_back_reference_read_node(_node: &BackReferenceReadNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_begin_node(node: &BeginNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.statements() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.rescue_clause() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.else_clause() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.ensure_clause() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_block_argument_node(node: &BlockArgumentNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.expression() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_block_local_variable_node(_node: &BlockLocalVariableNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_block_node(node: &BlockNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.parameters() {
        scan(&node, func);
    }
    if let Some(node) = &node.body() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_block_parameter_node(_node: &BlockParameterNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_block_parameters_node(node: &BlockParametersNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.parameters() {
        scan(&node.as_node(), func);
    }
    for node in &node.locals() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_break_node(node: &BreakNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_call_and_write_node(node: &CallAndWriteNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.receiver() {
        scan(&node, func);
    }
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_call_node(node: &CallNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.receiver() {
        scan(&node, func);
    }
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = node.block() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_call_operator_write_node(node: &CallOperatorWriteNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.receiver() {
        scan(&node, func);
    }
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_call_or_write_node(node: &CallOrWriteNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.receiver() {
        scan(&node, func);
    }
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_call_target_node(node: &CallTargetNode, func: &mut impl FnMut(&Node)) {
    scan(&node.receiver(), func);
}
#[rustfmt::skip]
fn scan_capture_pattern_node(node: &CapturePatternNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
    scan(&node.target().as_node(), func);
}
#[rustfmt::skip]
fn scan_case_match_node(node: &CaseMatchNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.predicate() {
        scan(&node, func);
    }
    for node in &node.conditions() {
        scan(&node, func);
    }
    if let Some(node) = &node.else_clause() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_case_node(node: &CaseNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.predicate() {
        scan(&node, func);
    }
    for node in &node.conditions() {
        scan(&node, func);
    }
    if let Some(node) = &node.else_clause() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_class_node(node: &ClassNode, func: &mut impl FnMut(&Node)) {
    scan(&node.constant_path(), func);
    if let Some(node) = &node.superclass() {
        scan(&node, func);
    }
    if let Some(node) = &node.body() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_class_variable_and_write_node(node: &ClassVariableAndWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_class_variable_operator_write_node(node: &ClassVariableOperatorWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_class_variable_or_write_node(node: &ClassVariableOrWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_class_variable_read_node(_node: &ClassVariableReadNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_class_variable_target_node(_node: &ClassVariableTargetNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_class_variable_write_node(node: &ClassVariableWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_constant_and_write_node(node: &ConstantAndWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_constant_operator_write_node(node: &ConstantOperatorWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_constant_or_write_node(node: &ConstantOrWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_constant_path_and_write_node(node: &ConstantPathAndWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.target().as_node(), func);
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_constant_path_node(node: &ConstantPathNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.parent() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_constant_path_operator_write_node(node: &ConstantPathOperatorWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.target().as_node(), func);
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_constant_path_or_write_node(node: &ConstantPathOrWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.target().as_node(), func);
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_constant_path_target_node(node: &ConstantPathTargetNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.parent() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_constant_path_write_node(node: &ConstantPathWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.target().as_node(), func);
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_constant_read_node(_node: &ConstantReadNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_constant_target_node(_node: &ConstantTargetNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_constant_write_node(node: &ConstantWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_def_node(node: &DefNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.receiver() {
        scan(&node, func);
    }
    if let Some(node) = &node.parameters() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.body() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_defined_node(node: &DefinedNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_else_node(node: &ElseNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_embedded_statements_node(node: &EmbeddedStatementsNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_embedded_variable_node(node: &EmbeddedVariableNode, func: &mut impl FnMut(&Node)) {
    scan(&node.variable(), func);
}
#[rustfmt::skip]
fn scan_ensure_node(node: &EnsureNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_false_node(_node: &FalseNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_find_pattern_node(node: &FindPatternNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.constant() {
        scan(&node, func);
    }
    scan(&node.left().as_node(), func);
    for node in &node.requireds() {
        scan(&node, func);
    }
    scan(&node.right(), func);
}
#[rustfmt::skip]
fn scan_flip_flop_node(node: &FlipFlopNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.left() {
        scan(&node, func);
    }
    if let Some(node) = &node.right() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_float_node(_node: &FloatNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_for_node(node: &ForNode, func: &mut impl FnMut(&Node)) {
    scan(&node.index(), func);
    scan(&node.collection(), func);
    if let Some(node) = node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_forwarding_arguments_node(_node: &ForwardingArgumentsNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_forwarding_parameter_node(_node: &ForwardingParameterNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_forwarding_super_node(node: &ForwardingSuperNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = node.block() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_global_variable_and_write_node(node: &GlobalVariableAndWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_global_variable_operator_write_node(node: &GlobalVariableOperatorWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_global_variable_or_write_node(node: &GlobalVariableOrWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_global_variable_read_node(_node: &GlobalVariableReadNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_global_variable_target_node(_node: &GlobalVariableTargetNode, _func: &mut impl FnMut(&Node)) {
}
#[rustfmt::skip]
fn scan_global_variable_write_node(node: &GlobalVariableWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_hash_node(node: &HashNode, func: &mut impl FnMut(&Node)) {
    for node in &node.elements() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_hash_pattern_node(node: &HashPatternNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = node.constant() {
        scan(&node, func);
    }
    for node in &node.elements() {
        scan(&node, func);
    }
    if let Some(node) = node.rest() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_if_node(node: &IfNode, func: &mut impl FnMut(&Node)) {
    scan(&node.predicate(), func);
    if let Some(node) = &node.statements() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.subsequent() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_imaginary_node(node: &ImaginaryNode, func: &mut impl FnMut(&Node)) {
    scan(&node.numeric(), func);
}
#[rustfmt::skip]
fn scan_implicit_node(node: &ImplicitNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_implicit_rest_node(_node: &ImplicitRestNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_in_node(node: &InNode, func: &mut impl FnMut(&Node)) {
    scan(&node.pattern(), func);
    if let Some(node) = node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_index_and_write_node(node: &IndexAndWriteNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.receiver() {
        scan(&node, func);
    }
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.block() {
        scan(&node.as_node(), func);
    }
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_index_operator_write_node(node: &IndexOperatorWriteNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.receiver() {
        scan(&node, func);
    }
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.block() {
        scan(&node.as_node(), func);
    }
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_index_or_write_node(node: &IndexOrWriteNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.receiver() {
        scan(&node, func);
    }
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.block() {
        scan(&node.as_node(), func);
    }
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_index_target_node(node: &IndexTargetNode, func: &mut impl FnMut(&Node)) {
    scan(&node.receiver(), func);
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.block() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_instance_variable_and_write_node(node: &InstanceVariableAndWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_instance_variable_operator_write_node(node: &InstanceVariableOperatorWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_instance_variable_or_write_node(node: &InstanceVariableOrWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_instance_variable_read_node(_node: &InstanceVariableReadNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_instance_variable_target_node(_node: &InstanceVariableTargetNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_instance_variable_write_node(node: &InstanceVariableWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_integer_node(_node: &IntegerNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_interpolated_match_last_line_node(node: &InterpolatedMatchLastLineNode, func: &mut impl FnMut(&Node)) {
    for node in &node.parts() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_interpolated_regular_expression_node(node: &InterpolatedRegularExpressionNode, func: &mut impl FnMut(&Node)) {
    for node in &node.parts() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_interpolated_string_node(node: &InterpolatedStringNode, func: &mut impl FnMut(&Node)) {
    for node in &node.parts() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_interpolated_symbol_node(node: &InterpolatedSymbolNode, func: &mut impl FnMut(&Node)) {
    for node in &node.parts() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_interpolated_x_string_node(node: &InterpolatedXStringNode, func: &mut impl FnMut(&Node)) {
    for node in &node.parts() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_it_local_variable_read_node(_node: &ItLocalVariableReadNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_it_parameters_node(_node: &ItParametersNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_keyword_hash_node(node: &KeywordHashNode, func: &mut impl FnMut(&Node)) {
    for node in &node.elements() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_keyword_rest_parameter_node(_node: &KeywordRestParameterNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_lambda_node(node: &LambdaNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.parameters() {
        scan(&node, func);
    }
    if let Some(node) = &node.body() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_local_variable_and_write_node(node: &LocalVariableAndWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_local_variable_operator_write_node(node: &LocalVariableOperatorWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_local_variable_or_write_node(node: &LocalVariableOrWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_local_variable_read_node(_node: &LocalVariableReadNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_local_variable_target_node(_node: &LocalVariableTargetNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_local_variable_write_node(node: &LocalVariableWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_match_last_line_node(_node: &MatchLastLineNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_match_predicate_node(node: &MatchPredicateNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
    scan(&node.pattern(), func);
}
#[rustfmt::skip]
fn scan_match_required_node(node: &MatchRequiredNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
    scan(&node.pattern(), func);
}
#[rustfmt::skip]
fn scan_match_write_node(node: &MatchWriteNode, func: &mut impl FnMut(&Node)) {
    scan(&node.call().as_node(), func);
    for node in &node.targets() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_missing_node(_node: &MissingNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_module_node(node: &ModuleNode, func: &mut impl FnMut(&Node)) {
    scan(&node.constant_path(), func);
    if let Some(node) = &node.body() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_multi_target_node(node: &MultiTargetNode, func: &mut impl FnMut(&Node)) {
    for node in &node.lefts() {
        scan(&node, func);
    }
    if let Some(node) = &node.rest() {
        scan(&node, func);
    }
    for node in &node.rights() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_multi_write_node(node: &MultiWriteNode, func: &mut impl FnMut(&Node)) {
    for node in &node.lefts() {
        scan(&node, func);
    }
    if let Some(node) = &node.rest() {
        scan(&node, func);
    }
    for node in &node.rights() {
        scan(&node, func);
    }
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_next_node(node: &NextNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_nil_node(_node: &NilNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_no_keywords_parameter_node(_node: &NoKeywordsParameterNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_numbered_parameters_node(_node: &NumberedParametersNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_numbered_reference_read_node(_node: &NumberedReferenceReadNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_optional_keyword_parameter_node(node: &OptionalKeywordParameterNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_optional_parameter_node(node: &OptionalParameterNode, func: &mut impl FnMut(&Node)) {
    scan(&node.value(), func);
}
#[rustfmt::skip]
fn scan_or_node(node: &OrNode, func: &mut impl FnMut(&Node)) {
    scan(&node.left(), func);
    scan(&node.right(), func);
}
#[rustfmt::skip]
fn scan_parameters_node(node: &ParametersNode, func: &mut impl FnMut(&Node)) {
    for node in &node.requireds() {
        scan(&node, func);
    }
    for node in &node.optionals() {
        scan(&node, func);
    }
    if let Some(node) = &node.rest() {
        scan(&node, func);
    }
    for node in &node.posts() {
        scan(&node, func);
    }
    for node in &node.keywords() {
        scan(&node, func);
    }
    if let Some(node) = &node.keyword_rest() {
        scan(&node, func);
    }
    if let Some(node) = &node.block() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_parentheses_node(node: &ParenthesesNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.body() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_pinned_expression_node(node: &PinnedExpressionNode, func: &mut impl FnMut(&Node)) {
    scan(&node.expression(), func);
}
#[rustfmt::skip]
fn scan_pinned_variable_node(node: &PinnedVariableNode, func: &mut impl FnMut(&Node)) {
    scan(&node.variable(), func);
}
#[rustfmt::skip]
fn scan_post_execution_node(node: &PostExecutionNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_pre_execution_node(node: &PreExecutionNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_program_node(node: &ProgramNode, func: &mut impl FnMut(&Node)) {
    scan(&node.statements().as_node(), func);
}
#[rustfmt::skip]
fn scan_range_node(node: &RangeNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.left() {
        scan(&node, func);
    }
    if let Some(node) = &node.right() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_rational_node(_node: &RationalNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_redo_node(_node: &RedoNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_regular_expression_node(_node: &RegularExpressionNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_required_keyword_parameter_node(_node: &RequiredKeywordParameterNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_required_parameter_node(_node: &RequiredParameterNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_rescue_modifier_node(node: &RescueModifierNode, func: &mut impl FnMut(&Node)) {
    scan(&node.expression(), func);
    scan(&node.rescue_expression(), func);
}
#[rustfmt::skip]
fn scan_rescue_node(node: &RescueNode, func: &mut impl FnMut(&Node)) {
    for node in &node.exceptions() {
        scan(&node, func);
    }
    if let Some(node) = &node.reference() {
        scan(&node, func);
    }
    if let Some(node) = &node.statements() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.subsequent() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_rest_parameter_node(_node: &RestParameterNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_retry_node(_node: &RetryNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_return_node(node: &ReturnNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_self_node(_node: &SelfNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_shareable_constant_node(node: &ShareableConstantNode, func: &mut impl FnMut(&Node)) {
    scan(&node.write(), func);
}
#[rustfmt::skip]
fn scan_singleton_class_node(node: &SingletonClassNode, func: &mut impl FnMut(&Node)) {
    scan(&node.expression(), func);
    if let Some(node) = &node.body() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_source_encoding_node(_node: &SourceEncodingNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_source_file_node(_node: &SourceFileNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_source_line_node(_node: &SourceLineNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_splat_node(node: &SplatNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.expression() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_statements_node(node: &StatementsNode, func: &mut impl FnMut(&Node)) {
    for node in &node.body() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_string_node(_node: &StringNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_super_node(node: &SuperNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = node.block() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_symbol_node(_node: &SymbolNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_true_node(_node: &TrueNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_undef_node(node: &UndefNode, func: &mut impl FnMut(&Node)) {
    for node in &node.names() {
        scan(&node, func);
    }
}
#[rustfmt::skip]
fn scan_unless_node(node: &UnlessNode, func: &mut impl FnMut(&Node)) {
    scan(&node.predicate(), func);
    if let Some(node) = &node.statements() {
        scan(&node.as_node(), func);
    }
    if let Some(node) = &node.else_clause() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_until_node(node: &UntilNode, func: &mut impl FnMut(&Node)) {
    scan(&node.predicate(), func);
    if let Some(node) = node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_when_node(node: &WhenNode, func: &mut impl FnMut(&Node)) {
    for node in &node.conditions() {
        scan(&node, func);
    }
    if let Some(node) = &node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_while_node(node: &WhileNode, func: &mut impl FnMut(&Node)) {
    scan(&node.predicate(), func);
    if let Some(node) = node.statements() {
        scan(&node.as_node(), func);
    }
}
#[rustfmt::skip]
fn scan_x_string_node(_node: &XStringNode, _func: &mut impl FnMut(&Node)) {}
#[rustfmt::skip]
fn scan_yield_node(node: &YieldNode, func: &mut impl FnMut(&Node)) {
    if let Some(node) = &node.arguments() {
        scan(&node.as_node(), func);
    }
}
