use crate::Document;
use crate::builder::builder::*;
use crate::builder::prism::_new_layout_node::*;
use crate::builder::prism::_new_layout_node_param::*;
use crate::builder::prism::BuildContext;
use crate::keyword::*;
use ruby_prism::*;
use std::io::Read;

#[rustfmt::skip]
pub fn build_node(node: &Node<'_>, context: &mut BuildContext) -> Document{
    match node {
        Node::AliasGlobalVariableNode           { .. } => build_alias_global_variable_node           (&node.as_alias_global_variable_node().unwrap()           , context),
        Node::AliasMethodNode                   { .. } => build_alias_method_node                    (&node.as_alias_method_node().unwrap()                    , context),
        Node::AlternationPatternNode            { .. } => build_alternation_pattern_node             (&node.as_alternation_pattern_node().unwrap()             , context),
        Node::AndNode                           { .. } => build_and_node                             (&node.as_and_node().unwrap()                             , context),
        Node::ArgumentsNode                     { .. } => build_arguments_node                       (&node.as_arguments_node().unwrap()                       , context),
        Node::ArrayNode                         { .. } => build_array_node                           (&node.as_array_node().unwrap()                           , context),
        Node::ArrayPatternNode                  { .. } => build_array_pattern_node                   (&node.as_array_pattern_node().unwrap()                   , context),
        Node::AssocNode                         { .. } => build_assoc_node                           (&node.as_assoc_node().unwrap()                           , context),
        Node::AssocSplatNode                    { .. } => build_assoc_splat_node                     (&node.as_assoc_splat_node().unwrap()                     , context),
        Node::BackReferenceReadNode             { .. } => build_back_reference_read_node             (&node.as_back_reference_read_node().unwrap()             , context),
        Node::BeginNode                         { .. } => build_begin_node                           (&node.as_begin_node().unwrap()                           , context),
        Node::BlockArgumentNode                 { .. } => build_block_argument_node                  (&node.as_block_argument_node().unwrap()                  , context),
        Node::BlockLocalVariableNode            { .. } => build_block_local_variable_node            (&node.as_block_local_variable_node().unwrap()            , context),
        Node::BlockNode                         { .. } => build_block_node                           (&node.as_block_node().unwrap()                           , context),
        Node::BlockParameterNode                { .. } => build_block_parameter_node                 (&node.as_block_parameter_node().unwrap()                 , context),
        Node::BlockParametersNode               { .. } => build_block_parameters_node                (&node.as_block_parameters_node().unwrap()                , context),
        Node::BreakNode                         { .. } => build_break_node                           (&node.as_break_node().unwrap()                           , context),
        Node::CallAndWriteNode                  { .. } => build_call_and_write_node                  (&node.as_call_and_write_node().unwrap()                  , context),
        Node::CallNode                          { .. } => build_call_node                            (&node.as_call_node().unwrap()                            , context),
        Node::CallOperatorWriteNode             { .. } => build_call_operator_write_node             (&node.as_call_operator_write_node().unwrap()             , context),
        Node::CallOrWriteNode                   { .. } => build_call_or_write_node                   (&node.as_call_or_write_node().unwrap()                   , context),
        Node::CallTargetNode                    { .. } => build_call_target_node                     (&node.as_call_target_node().unwrap()                     , context),
        Node::CapturePatternNode                { .. } => build_capture_pattern_node                 (&node.as_capture_pattern_node().unwrap()                 , context),
        Node::CaseMatchNode                     { .. } => build_case_match_node                      (&node.as_case_match_node().unwrap()                      , context),
        Node::CaseNode                          { .. } => build_case_node                            (&node.as_case_node().unwrap()                            , context),
        Node::ClassNode                         { .. } => build_class_node                           (&node.as_class_node().unwrap()                           , context),
        Node::ClassVariableAndWriteNode         { .. } => build_class_variable_and_write_node        (&node.as_class_variable_and_write_node().unwrap()        , context),
        Node::ClassVariableOperatorWriteNode    { .. } => build_class_variable_operator_write_node   (&node.as_class_variable_operator_write_node().unwrap()   , context),
        Node::ClassVariableOrWriteNode          { .. } => build_class_variable_or_write_node         (&node.as_class_variable_or_write_node().unwrap()         , context),
        Node::ClassVariableReadNode             { .. } => build_class_variable_read_node             (&node.as_class_variable_read_node().unwrap()             , context),
        Node::ClassVariableTargetNode           { .. } => build_class_variable_target_node           (&node.as_class_variable_target_node().unwrap()           , context),
        Node::ClassVariableWriteNode            { .. } => build_class_variable_write_node            (&node.as_class_variable_write_node().unwrap()            , context),
        Node::ConstantAndWriteNode              { .. } => build_constant_and_write_node              (&node.as_constant_and_write_node().unwrap()              , context),
        Node::ConstantOperatorWriteNode         { .. } => build_constant_operator_write_node         (&node.as_constant_operator_write_node().unwrap()         , context),
        Node::ConstantOrWriteNode               { .. } => build_constant_or_write_node               (&node.as_constant_or_write_node().unwrap()               , context),
        Node::ConstantPathAndWriteNode          { .. } => build_constant_path_and_write_node         (&node.as_constant_path_and_write_node().unwrap()         , context),
        Node::ConstantPathNode                  { .. } => build_constant_path_node                   (&node.as_constant_path_node().unwrap()                   , context),
        Node::ConstantPathOperatorWriteNode     { .. } => build_constant_path_operator_write_node    (&node.as_constant_path_operator_write_node().unwrap()    , context),
        Node::ConstantPathOrWriteNode           { .. } => build_constant_path_or_write_node          (&node.as_constant_path_or_write_node().unwrap()          , context),
        Node::ConstantPathTargetNode            { .. } => build_constant_path_target_node            (&node.as_constant_path_target_node().unwrap()            , context),
        Node::ConstantPathWriteNode             { .. } => build_constant_path_write_node             (&node.as_constant_path_write_node().unwrap()             , context),
        Node::ConstantReadNode                  { .. } => build_constant_read_node                   (&node.as_constant_read_node().unwrap()                   , context),
        Node::ConstantTargetNode                { .. } => build_constant_target_node                 (&node.as_constant_target_node().unwrap()                 , context),
        Node::ConstantWriteNode                 { .. } => build_constant_write_node                  (&node.as_constant_write_node().unwrap()                  , context),
        Node::DefNode                           { .. } => build_def_node                             (&node.as_def_node().unwrap()                             , context),
        Node::DefinedNode                       { .. } => build_defined_node                         (&node.as_defined_node().unwrap()                         , context),
        Node::ElseNode                          { .. } => build_else_node                            (&node.as_else_node().unwrap()                            , context),
        Node::EmbeddedStatementsNode            { .. } => build_embedded_statements_node             (&node.as_embedded_statements_node().unwrap()             , context),
        Node::EmbeddedVariableNode              { .. } => build_embedded_variable_node               (&node.as_embedded_variable_node().unwrap()               , context),
        Node::EnsureNode                        { .. } => build_ensure_node                          (&node.as_ensure_node().unwrap()                          , context),
        Node::FalseNode                         { .. } => build_false_node                           (&node.as_false_node().unwrap()                           , context),
        Node::FindPatternNode                   { .. } => build_find_pattern_node                    (&node.as_find_pattern_node().unwrap()                    , context),
        Node::FlipFlopNode                      { .. } => build_flip_flop_node                       (&node.as_flip_flop_node().unwrap()                       , context),
        Node::FloatNode                         { .. } => build_float_node                           (&node.as_float_node().unwrap()                           , context),
        Node::ForNode                           { .. } => build_for_node                             (&node.as_for_node().unwrap()                             , context),
        Node::ForwardingArgumentsNode           { .. } => build_forwarding_arguments_node            (&node.as_forwarding_arguments_node().unwrap()            , context),
        Node::ForwardingParameterNode           { .. } => build_forwarding_parameter_node            (&node.as_forwarding_parameter_node().unwrap()            , context),
        Node::ForwardingSuperNode               { .. } => build_forwarding_super_node                (&node.as_forwarding_super_node().unwrap()                , context),
        Node::GlobalVariableAndWriteNode        { .. } => build_global_variable_and_write_node       (&node.as_global_variable_and_write_node().unwrap()       , context),
        Node::GlobalVariableOperatorWriteNode   { .. } => build_global_variable_operator_write_node  (&node.as_global_variable_operator_write_node().unwrap()  , context),
        Node::GlobalVariableOrWriteNode         { .. } => build_global_variable_or_write_node        (&node.as_global_variable_or_write_node().unwrap()        , context),
        Node::GlobalVariableReadNode            { .. } => build_global_variable_read_node            (&node.as_global_variable_read_node().unwrap()            , context),
        Node::GlobalVariableTargetNode          { .. } => build_global_variable_target_node          (&node.as_global_variable_target_node().unwrap()          , context),
        Node::GlobalVariableWriteNode           { .. } => build_global_variable_write_node           (&node.as_global_variable_write_node().unwrap()           , context),
        Node::HashNode                          { .. } => build_hash_node                            (&node.as_hash_node().unwrap()                            , context),
        Node::HashPatternNode                   { .. } => build_hash_pattern_node                    (&node.as_hash_pattern_node().unwrap()                    , context),
        Node::IfNode                            { .. } => build_if_node                              (&node.as_if_node().unwrap()                              , context),
        Node::ImaginaryNode                     { .. } => build_imaginary_node                       (&node.as_imaginary_node().unwrap()                       , context),
        Node::ImplicitNode                      { .. } => build_implicit_node                        (&node.as_implicit_node().unwrap()                        , context),
        Node::ImplicitRestNode                  { .. } => build_implicit_rest_node                   (&node.as_implicit_rest_node().unwrap()                   , context),
        Node::InNode                            { .. } => build_in_node                              (&node.as_in_node().unwrap()                              , context),
        Node::IndexAndWriteNode                 { .. } => build_index_and_write_node                 (&node.as_index_and_write_node().unwrap()                 , context),
        Node::IndexOperatorWriteNode            { .. } => build_index_operator_write_node            (&node.as_index_operator_write_node().unwrap()            , context),
        Node::IndexOrWriteNode                  { .. } => build_index_or_write_node                  (&node.as_index_or_write_node().unwrap()                  , context),
        Node::IndexTargetNode                   { .. } => build_index_target_node                    (&node.as_index_target_node().unwrap()                    , context),
        Node::InstanceVariableAndWriteNode      { .. } => build_instance_variable_and_write_node     (&node.as_instance_variable_and_write_node().unwrap()     , context),
        Node::InstanceVariableOperatorWriteNode { .. } => build_instance_variable_operator_write_node(&node.as_instance_variable_operator_write_node().unwrap(), context),
        Node::InstanceVariableOrWriteNode       { .. } => build_instance_variable_or_write_node      (&node.as_instance_variable_or_write_node().unwrap()      , context),
        Node::InstanceVariableReadNode          { .. } => build_instance_variable_read_node          (&node.as_instance_variable_read_node().unwrap()          , context),
        Node::InstanceVariableTargetNode        { .. } => build_instance_variable_target_node        (&node.as_instance_variable_target_node().unwrap()        , context),
        Node::InstanceVariableWriteNode         { .. } => build_instance_variable_write_node         (&node.as_instance_variable_write_node().unwrap()         , context),
        Node::IntegerNode                       { .. } => build_integer_node                         (&node.as_integer_node().unwrap()                         , context),
        Node::InterpolatedMatchLastLineNode     { .. } => build_interpolated_match_last_line_node    (&node.as_interpolated_match_last_line_node().unwrap()    , context),
        Node::InterpolatedRegularExpressionNode { .. } => build_interpolated_regular_expression_node (&node.as_interpolated_regular_expression_node().unwrap() , context),
        Node::InterpolatedStringNode            { .. } => build_interpolated_string_node             (&node.as_interpolated_string_node().unwrap()             , context),
        Node::InterpolatedSymbolNode            { .. } => build_interpolated_symbol_node             (&node.as_interpolated_symbol_node().unwrap()             , context),
        Node::InterpolatedXStringNode           { .. } => build_interpolated_x_string_node           (&node.as_interpolated_x_string_node().unwrap()           , context),
        Node::ItLocalVariableReadNode           { .. } => build_it_local_variable_read_node          (&node.as_it_local_variable_read_node().unwrap()          , context),
        Node::ItParametersNode                  { .. } => build_it_parameters_node                   (&node.as_it_parameters_node().unwrap()                   , context),
        Node::KeywordHashNode                   { .. } => build_keyword_hash_node                    (&node.as_keyword_hash_node().unwrap()                    , context),
        Node::KeywordRestParameterNode          { .. } => build_keyword_rest_parameter_node          (&node.as_keyword_rest_parameter_node().unwrap()          , context),
        Node::LambdaNode                        { .. } => build_lambda_node                          (&node.as_lambda_node().unwrap()                          , context),
        Node::LocalVariableAndWriteNode         { .. } => build_local_variable_and_write_node        (&node.as_local_variable_and_write_node().unwrap()        , context),
        Node::LocalVariableOperatorWriteNode    { .. } => build_local_variable_operator_write_node   (&node.as_local_variable_operator_write_node().unwrap()   , context),
        Node::LocalVariableOrWriteNode          { .. } => build_local_variable_or_write_node         (&node.as_local_variable_or_write_node().unwrap()         , context),
        Node::LocalVariableReadNode             { .. } => build_local_variable_read_node             (&node.as_local_variable_read_node().unwrap()             , context),
        Node::LocalVariableTargetNode           { .. } => build_local_variable_target_node           (&node.as_local_variable_target_node().unwrap()           , context),
        Node::LocalVariableWriteNode            { .. } => build_local_variable_write_node            (&node.as_local_variable_write_node().unwrap()            , context),
        Node::MatchLastLineNode                 { .. } => build_match_last_line_node                 (&node.as_match_last_line_node().unwrap()                 , context),
        Node::MatchPredicateNode                { .. } => build_match_predicate_node                 (&node.as_match_predicate_node().unwrap()                 , context),
        Node::MatchRequiredNode                 { .. } => build_match_required_node                  (&node.as_match_required_node().unwrap()                  , context),
        Node::MatchWriteNode                    { .. } => build_match_write_node                     (&node.as_match_write_node().unwrap()                     , context),
        Node::MissingNode                       { .. } => build_missing_node                         (&node.as_missing_node().unwrap()                         , context),
        Node::ModuleNode                        { .. } => build_module_node                          (&node.as_module_node().unwrap()                          , context),
        Node::MultiTargetNode                   { .. } => build_multi_target_node                    (&node.as_multi_target_node().unwrap()                    , context),
        Node::MultiWriteNode                    { .. } => build_multi_write_node                     (&node.as_multi_write_node().unwrap()                     , context),
        Node::NextNode                          { .. } => build_next_node                            (&node.as_next_node().unwrap()                            , context),
        Node::NilNode                           { .. } => build_nil_node                             (&node.as_nil_node().unwrap()                             , context),
        Node::NoKeywordsParameterNode           { .. } => build_no_keywords_parameter_node           (&node.as_no_keywords_parameter_node().unwrap()           , context),
        Node::NumberedParametersNode            { .. } => build_numbered_parameters_node             (&node.as_numbered_parameters_node().unwrap()             , context),
        Node::NumberedReferenceReadNode         { .. } => build_numbered_reference_read_node         (&node.as_numbered_reference_read_node().unwrap()         , context),
        Node::OptionalKeywordParameterNode      { .. } => build_optional_keyword_parameter_node      (&node.as_optional_keyword_parameter_node().unwrap()      , context),
        Node::OptionalParameterNode             { .. } => build_optional_parameter_node              (&node.as_optional_parameter_node().unwrap()              , context),
        Node::OrNode                            { .. } => build_or_node                              (&node.as_or_node().unwrap()                              , context),
        Node::ParametersNode                    { .. } => build_parameters_node                      (&node.as_parameters_node().unwrap()                      , context),
        Node::ParenthesesNode                   { .. } => build_parentheses_node                     (&node.as_parentheses_node().unwrap()                     , context),
        Node::PinnedExpressionNode              { .. } => build_pinned_expression_node               (&node.as_pinned_expression_node().unwrap()               , context),
        Node::PinnedVariableNode                { .. } => build_pinned_variable_node                 (&node.as_pinned_variable_node().unwrap()                 , context),
        Node::PostExecutionNode                 { .. } => build_post_execution_node                  (&node.as_post_execution_node().unwrap()                  , context),
        Node::PreExecutionNode                  { .. } => build_pre_execution_node                   (&node.as_pre_execution_node().unwrap()                   , context),
        Node::ProgramNode                       { .. } => build_program_node                         (&node.as_program_node().unwrap()                         , context),
        Node::RangeNode                         { .. } => build_range_node                           (&node.as_range_node().unwrap()                           , context),
        Node::RationalNode                      { .. } => build_rational_node                        (&node.as_rational_node().unwrap()                        , context),
        Node::RedoNode                          { .. } => build_redo_node                            (&node.as_redo_node().unwrap()                            , context),
        Node::RegularExpressionNode             { .. } => build_regular_expression_node              (&node.as_regular_expression_node().unwrap()              , context),
        Node::RequiredKeywordParameterNode      { .. } => build_required_keyword_parameter_node      (&node.as_required_keyword_parameter_node().unwrap()      , context),
        Node::RequiredParameterNode             { .. } => build_required_parameter_node              (&node.as_required_parameter_node().unwrap()              , context),
        Node::RescueModifierNode                { .. } => build_rescue_modifier_node                 (&node.as_rescue_modifier_node().unwrap()                 , context),
        Node::RescueNode                        { .. } => build_rescue_node                          (&node.as_rescue_node().unwrap()                          , context),
        Node::RestParameterNode                 { .. } => build_rest_parameter_node                  (&node.as_rest_parameter_node().unwrap()                  , context),
        Node::RetryNode                         { .. } => build_retry_node                           (&node.as_retry_node().unwrap()                           , context),
        Node::ReturnNode                        { .. } => build_return_node                          (&node.as_return_node().unwrap()                          , context),
        Node::SelfNode                          { .. } => build_self_node                            (&node.as_self_node().unwrap()                            , context),
        Node::ShareableConstantNode             { .. } => build_shareable_constant_node              (&node.as_shareable_constant_node().unwrap()              , context),
        Node::SingletonClassNode                { .. } => build_singleton_class_node                 (&node.as_singleton_class_node().unwrap()                 , context),
        Node::SourceEncodingNode                { .. } => build_source_encoding_node                 (&node.as_source_encoding_node().unwrap()                 , context),
        Node::SourceFileNode                    { .. } => build_source_file_node                     (&node.as_source_file_node().unwrap()                     , context),
        Node::SourceLineNode                    { .. } => build_source_line_node                     (&node.as_source_line_node().unwrap()                     , context),
        Node::SplatNode                         { .. } => build_splat_node                           (&node.as_splat_node().unwrap()                           , context),
        Node::StatementsNode                    { .. } => build_statements_node                      (&node.as_statements_node().unwrap()                      , context),
        Node::StringNode                        { .. } => build_string_node                          (&node.as_string_node().unwrap()                          , context),
        Node::SuperNode                         { .. } => build_super_node                           (&node.as_super_node().unwrap()                           , context),
        Node::SymbolNode                        { .. } => build_symbol_node                          (&node.as_symbol_node().unwrap()                          , context),
        Node::TrueNode                          { .. } => build_true_node                            (&node.as_true_node().unwrap()                            , context),
        Node::UndefNode                         { .. } => build_undef_node                           (&node.as_undef_node().unwrap()                           , context),
        Node::UnlessNode                        { .. } => build_unless_node                          (&node.as_unless_node().unwrap()                          , context),
        Node::UntilNode                         { .. } => build_until_node                           (&node.as_until_node().unwrap()                           , context),
        Node::WhenNode                          { .. } => build_when_node                            (&node.as_when_node().unwrap()                            , context),
        Node::WhileNode                         { .. } => build_while_node                           (&node.as_while_node().unwrap()                           , context),
        Node::XStringNode                       { .. } => build_x_string_node                        (&node.as_x_string_node().unwrap()                        , context),
        Node::YieldNode                         { .. } => build_yield_node                           (&node.as_yield_node().unwrap()                           , context),
    }
}

pub fn escape(unescaped: &[u8]) -> String {
    // TODO
    String::new()
}
pub fn build_constant_id(constant_id: &ConstantId<'_>, context: &mut BuildContext) -> Document {
    let mut buf = String::new();
    constant_id.as_slice().read_to_string(&mut buf);
    string(buf)
}
pub fn build_f64(value: &f64, context: &mut BuildContext) -> Document {
    // TODO
    Document::None
}
pub fn build_integer(integer: &Integer<'_>, context: &mut BuildContext) -> Document {
    // TODO
    Document::None
}

#[rustfmt::skip]
fn build_alias_global_variable_node(node: &AliasGlobalVariableNode<'_>, context: &mut BuildContext) -> Document {
    let new_name = build_node(&node.new_name(), context);
    let old_name = build_node(&node.old_name(), context);
    layout_alias_global_variable_node(&LayoutParamAliasGlobalVariableNode { new_name, old_name })
}
#[rustfmt::skip]
fn build_alias_method_node(node: &AliasMethodNode<'_>, context: &mut BuildContext) -> Document {
    let new_name = build_node(&node.new_name(), context);
    let old_name = build_node(&node.old_name(), context);
    layout_alias_method_node(&LayoutParamAliasMethodNode { new_name, old_name })
}
#[rustfmt::skip]
fn build_alternation_pattern_node(node: &AlternationPatternNode<'_>, context: &mut BuildContext) -> Document {
    let left = build_node(&node.left(), context);
    let right = build_node(&node.right(), context);
    layout_alternation_pattern_node(&LayoutParamAlternationPatternNode { left, right })
}
#[rustfmt::skip]
fn build_and_node(node: &AndNode<'_>, context: &mut BuildContext) -> Document {
    let left = build_node(&node.left(), context);
    let right = build_node(&node.right(), context);
    layout_and_node(&LayoutParamAndNode{ left, right })
}
#[rustfmt::skip]
fn build_arguments_node(node: &ArgumentsNode<'_>, context: &mut BuildContext) -> Document {
    let mut arguments = Vec::new();
    for node in &node.arguments() {
        arguments.push(build_node(&node, context));
    }
    layout_arguments_node(&LayoutParamArgumentsNode { arguments })
}
#[rustfmt::skip]
fn build_array_node(node: &ArrayNode<'_>, context: &mut BuildContext) -> Document {
    let mut elements = Vec::new();
    for node in &node.elements() {
        elements.push(build_node(&node, context));
    }
    layout_array_node(&LayoutParamArrayNode { elements } )
}
#[rustfmt::skip]
fn build_array_pattern_node(node: &ArrayPatternNode<'_>, context: &mut BuildContext) -> Document {
    let constant = match &node.constant() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut requireds = Vec::new();
    for node in &node.requireds() {
        requireds.push(build_node(&node, context));
    }
    let rest = match &node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut posts = Vec::new();
    for node in &node.posts() {
        posts.push(build_node(&node, context));
    }
    layout_array_pattern_node(&LayoutParamArrayPatternNode { constant, requireds, rest, posts })
}
#[rustfmt::skip]
fn build_assoc_node(node: &AssocNode<'_>, context: &mut BuildContext) -> Document {
    let key = build_node(&node.key(), context);
    let value = build_node(&node.value(), context);
    layout_assoc_node(&LayoutParamAssocNode { key, value })
}
#[rustfmt::skip]
fn build_assoc_splat_node(node: &AssocSplatNode<'_>, context: &mut BuildContext) -> Document {
    let value = match &node.value() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_assoc_splat_node(&LayoutParamAssocSplatNode { value } )
}
#[rustfmt::skip]
fn build_back_reference_read_node(node: &BackReferenceReadNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_back_reference_read_node(&LayoutParamBackReferenceReadNode { name })
}
#[rustfmt::skip]
fn build_begin_node(node: &BeginNode<'_>, context: &mut BuildContext) -> Document {
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let rescue_clause = match &node.rescue_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let else_clause = match &node.else_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let ensure_clause = match &node.ensure_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_begin_node(&LayoutParamBeginNode { statements, rescue_clause, else_clause, ensure_clause })
}
#[rustfmt::skip]
fn build_block_argument_node(node: &BlockArgumentNode<'_>, context: &mut BuildContext) -> Document {
    let expression = match &node.expression() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_block_argument_node(&LayoutParamBlockArgumentNode { expression })
}
#[rustfmt::skip]
fn build_block_local_variable_node(node: &BlockLocalVariableNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_block_local_variable_node(&LayoutParamBlockLocalVariableNode { name })
}
#[rustfmt::skip]
fn build_block_node(node: &BlockNode<'_>, context: &mut BuildContext) -> Document {
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_block_node(&LayoutParamBlockNode { parameters, body })
}
#[rustfmt::skip]
fn build_block_parameter_node(node: &BlockParameterNode<'_>, context: &mut BuildContext) -> Document {
    let name = match &node.name() {
        Some(id) => Some(build_constant_id(id, context)),
        None => None,
    };
    layout_block_parameter_node(&LayoutParamBlockParameterNode { name })
}
#[rustfmt::skip]
fn build_block_parameters_node(node: &BlockParametersNode<'_>, context: &mut BuildContext) -> Document {
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let mut locals = Vec::new();
    for node in &node.locals() {
        locals.push(build_node(&node, context));
    }
    layout_block_parameters_node(&LayoutParamBlockParametersNode { parameters, locals })
}
#[rustfmt::skip]
fn build_break_node(node: &BreakNode<'_>, context: &mut BuildContext) -> Document {
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_break_node(&LayoutParamBreakNode { arguments })
}
#[rustfmt::skip]
fn build_call_and_write_node(node: &CallAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let value = build_node(&node.value(), context);
    layout_call_and_write_node(&LayoutParamCallAndWriteNode { receiver, value })
}
#[rustfmt::skip]
fn build_call_node(node: &CallNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match node.block() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_call_node(&LayoutParamCallNode { receiver, arguments, block })
}
#[rustfmt::skip]
fn build_call_operator_write_node(node: &CallOperatorWriteNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let value = build_node(&node.value(), context);
    layout_call_operator_write_node(&LayoutParamCallOperatorWriteNode { receiver, value })
}
#[rustfmt::skip]
fn build_call_or_write_node(node: &CallOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let value = build_node(&node.value(), context);
    layout_call_or_write_node(&LayoutParamCallOrWriteNode { receiver, value })
}
#[rustfmt::skip]
fn build_call_target_node(node: &CallTargetNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = build_node(&node.receiver(), context);
    layout_call_target_node(&LayoutParamCallTargetNode { receiver })
}
#[rustfmt::skip]
fn build_capture_pattern_node(node: &CapturePatternNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    let target = build_node(&node.target().as_node(), context);
    layout_capture_pattern_node(&LayoutParamCapturePatternNode { value, target })
}
#[rustfmt::skip]
fn build_case_match_node(node: &CaseMatchNode<'_>, context: &mut BuildContext) -> Document {
    let predicate = match &node.predicate() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut conditions = Vec::new();
    for node in &node.conditions() {
        conditions.push(build_node(&node, context));
    }
    let else_clause = match &node.else_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_case_match_node(&LayoutParamCaseMatchNode { predicate, conditions, else_clause })
}
#[rustfmt::skip]
fn build_case_node(node: &CaseNode<'_>, context: &mut BuildContext) -> Document {
    let predicate = match &node.predicate() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut conditions = Vec::new();
    for node in &node.conditions() {
        conditions.push(build_node(&node, context));
    }
    let else_clause = match &node.else_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_case_node(&LayoutParamCaseNode { predicate, conditions, else_clause })
}
#[rustfmt::skip]
fn build_class_node(node: &ClassNode<'_>, context: &mut BuildContext) -> Document {
    build_node(&node.constant_path(), context);
    let superclass = match &node.superclass() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_class_node(&LayoutParamClassNode { superclass, body })
}
#[rustfmt::skip]
fn build_class_variable_and_write_node(node: &ClassVariableAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_class_variable_and_write_node(&LayoutParamClassVariableAndWriteNode { value })
}
#[rustfmt::skip]
fn build_class_variable_operator_write_node(node: &ClassVariableOperatorWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_class_variable_operator_write_node(&LayoutParamClassVariableOperatorWriteNode { value })
}
#[rustfmt::skip]
fn build_class_variable_or_write_node(node: &ClassVariableOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_class_variable_or_write_node(&LayoutParamClassVariableOrWriteNode { value })
}
#[rustfmt::skip]
fn build_class_variable_read_node(node: &ClassVariableReadNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_class_variable_read_node(&LayoutParamClassVariableReadNode { name })
}
#[rustfmt::skip]
fn build_class_variable_target_node(node: &ClassVariableTargetNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_class_variable_target_node(&LayoutParamClassVariableTargetNode { name })
}
#[rustfmt::skip]
fn build_class_variable_write_node(node: &ClassVariableWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_class_variable_write_node(&LayoutParamClassVariableWriteNode { value })
}
#[rustfmt::skip]
fn build_constant_and_write_node(node: &ConstantAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_constant_and_write_node(&LayoutParamConstantAndWriteNode { value })
}
#[rustfmt::skip]
fn build_constant_operator_write_node(node: &ConstantOperatorWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_constant_operator_write_node(&LayoutParamConstantOperatorWriteNode { value })
}
#[rustfmt::skip]
fn build_constant_or_write_node(node: &ConstantOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_constant_or_write_node(&LayoutParamConstantOrWriteNode { value })
}
#[rustfmt::skip]
fn build_constant_path_and_write_node(node: &ConstantPathAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let target = build_node(&node.target().as_node(), context);
    let value = build_node(&node.value(), context);
    layout_constant_path_and_write_node(&LayoutParamConstantPathAndWriteNode { target, value })
}
#[rustfmt::skip]
fn build_constant_path_node(node: &ConstantPathNode<'_>, context: &mut BuildContext) -> Document {
    let parent = match &node.parent() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_constant_path_node(&LayoutParamConstantPathNode { parent })
}
#[rustfmt::skip]
fn build_constant_path_operator_write_node(node: &ConstantPathOperatorWriteNode<'_>, context: &mut BuildContext) -> Document {
    let target = build_node(&node.target().as_node(), context);
    let value = build_node(&node.value(), context);
    layout_constant_path_operator_write_node(&LayoutParamConstantPathOperatorWriteNode { target, value })
}
#[rustfmt::skip]
fn build_constant_path_or_write_node(node: &ConstantPathOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let target = build_node(&node.target().as_node(), context);
    let value = build_node(&node.value(), context);
    layout_constant_path_or_write_node(&LayoutParamConstantPathOrWriteNode { target, value })
}
#[rustfmt::skip]
fn build_constant_path_target_node(node: &ConstantPathTargetNode<'_>, context: &mut BuildContext) -> Document {
    let parent = match &node.parent() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_constant_path_target_node(&LayoutParamConstantPathTargetNode { parent })
}
#[rustfmt::skip]
fn build_constant_path_write_node(node: &ConstantPathWriteNode<'_>, context: &mut BuildContext) -> Document {
    let target = build_node(&node.target().as_node(), context);
    let value = build_node(&node.value(), context);
    layout_constant_path_write_node(&LayoutParamConstantPathWriteNode { target, value })
}
#[rustfmt::skip]
fn build_constant_read_node(node: &ConstantReadNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_constant_read_node(&LayoutParamConstantReadNode { name })
}
#[rustfmt::skip]
fn build_constant_target_node(node: &ConstantTargetNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_constant_target_node(&LayoutParamConstantTargetNode { name })
}
#[rustfmt::skip]
fn build_constant_write_node(node: &ConstantWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_constant_write_node(&LayoutParamConstantWriteNode { value })
}
#[rustfmt::skip]
fn build_def_node(node: &DefNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_def_node(&LayoutParamDefNode { receiver, parameters, body })
}
#[rustfmt::skip]
fn build_defined_node(node: &DefinedNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_defined_node(&LayoutParamDefinedNode { value })
}
#[rustfmt::skip]
fn build_else_node(node: &ElseNode<'_>, context: &mut BuildContext) -> Document {
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_else_node(&LayoutParamElseNode { statements })
}
#[rustfmt::skip]
fn build_embedded_statements_node(node: &EmbeddedStatementsNode<'_>, context: &mut BuildContext) -> Document {
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_embedded_statements_node(&LayoutParamEmbeddedStatementsNode { statements })
}
#[rustfmt::skip]
fn build_embedded_variable_node(node: &EmbeddedVariableNode<'_>, context: &mut BuildContext) -> Document {
    let variable = build_node(&node.variable(), context);
    layout_embedded_variable_node(&LayoutParamEmbeddedVariableNode { variable })
}
#[rustfmt::skip]
fn build_ensure_node(node: &EnsureNode<'_>, context: &mut BuildContext) -> Document {
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_ensure_node(&LayoutParamEnsureNode { statements })
}
#[rustfmt::skip]
fn build_false_node(node: &FalseNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(FALSE.to_string());
    layout_false_node(&LayoutParamFalseNode { keyword })
}
#[rustfmt::skip]
fn build_find_pattern_node(node: &FindPatternNode<'_>, context: &mut BuildContext) -> Document {
    let constant = match &node.constant() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let left = build_node(&node.left().as_node(), context);
    let mut requireds = Vec::new();
    for node in &node.requireds() {
        requireds.push(build_node(&node, context));
    }
    let right = build_node(&node.right(), context);
    layout_find_pattern_node(&LayoutParamFindPatternNode { constant, left, requireds, right })
}
#[rustfmt::skip]
fn build_flip_flop_node(node: &FlipFlopNode<'_>, context: &mut BuildContext) -> Document {
    let left = match &node.left() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let right = match &node.right() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_flip_flop_node(&LayoutParamFlipFlopNode { left, right })
}
#[rustfmt::skip]
fn build_float_node(node: &FloatNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_f64(&node.value(), context);
    layout_float_node(&LayoutParamFloatNode { value })
}
#[rustfmt::skip]
fn build_for_node(node: &ForNode<'_>, context: &mut BuildContext) -> Document {
    let index = build_node(&node.index(), context);
    let collection = build_node(&node.collection(), context);
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_for_node(&LayoutParamForNode { index, collection, statements })
}
#[rustfmt::skip]
fn build_forwarding_arguments_node(node: &ForwardingArgumentsNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(TRIPLE_DOT.to_string());
    layout_forwarding_arguments_node(&LayoutParamForwardingArgumentsNode { keyword })
}
#[rustfmt::skip]
fn build_forwarding_parameter_node(node: &ForwardingParameterNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(TRIPLE_DOT.to_string());
    layout_forwarding_parameter_node(&LayoutParamForwardingParameterNode { keyword })
}
#[rustfmt::skip]
fn build_forwarding_super_node(node: &ForwardingSuperNode<'_>, context: &mut BuildContext) -> Document {
    let block = match node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_forwarding_super_node(&LayoutParamForwardingSuperNode { block })
}
#[rustfmt::skip]
fn build_global_variable_and_write_node(node: &GlobalVariableAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_global_variable_and_write_node(&LayoutParamGlobalVariableAndWriteNode { value })
}
#[rustfmt::skip]
fn build_global_variable_operator_write_node(node: &GlobalVariableOperatorWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_global_variable_operator_write_node(&LayoutParamGlobalVariableOperatorWriteNode { value })
}
#[rustfmt::skip]
fn build_global_variable_or_write_node(node: &GlobalVariableOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_global_variable_or_write_node(&LayoutParamGlobalVariableOrWriteNode { value })
}
#[rustfmt::skip]
fn build_global_variable_read_node(node: &GlobalVariableReadNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_global_variable_read_node(&LayoutParamGlobalVariableReadNode { name })
}
#[rustfmt::skip]
fn build_global_variable_target_node(node: &GlobalVariableTargetNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_global_variable_target_node(&LayoutParamGlobalVariableTargetNode { name })
}
#[rustfmt::skip]
fn build_global_variable_write_node(node: &GlobalVariableWriteNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    let value = build_node(&node.value(), context);
    layout_global_variable_write_node(&LayoutParamGlobalVariableWriteNode { name, value })
}
#[rustfmt::skip]
fn build_hash_node(node: &HashNode<'_>, context: &mut BuildContext) -> Document {
    let mut elements = Vec::new();
    for node in &node.elements() {
        elements.push(build_node(&node, context));
    }
    layout_hash_node(&LayoutParamHashNode { elements })
}
#[rustfmt::skip]
fn build_hash_pattern_node(node: &HashPatternNode<'_>, context: &mut BuildContext) -> Document {
    let constant = match node.constant() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut elements = Vec::new();
    for node in &node.elements() {
        elements.push(build_node(&node, context));
    }
    let rest = match node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_hash_pattern_node(&LayoutParamHashPatternNode { constant, elements, rest })
}
#[rustfmt::skip]
fn build_if_node(node: &IfNode<'_>, context: &mut BuildContext) -> Document {
    build_node(&node.predicate(), context);
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let subsequent = match &node.subsequent() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_if_node(&LayoutParamIfNode { statements, subsequent })
}
#[rustfmt::skip]
fn build_imaginary_node(node: &ImaginaryNode<'_>, context: &mut BuildContext) -> Document {
    let numeric = build_node(&node.numeric(), context);
    layout_imaginary_node(&LayoutParamImaginaryNode { numeric })
}
#[rustfmt::skip]
fn build_implicit_node(node: &ImplicitNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_implicit_node(&LayoutParamImplicitNode { value })
}
#[rustfmt::skip]
fn build_implicit_rest_node(node: &ImplicitRestNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(COMMA.to_string());
    layout_implicit_rest_node(&LayoutParamImplicitRestNode { keyword })
}
#[rustfmt::skip]
fn build_in_node(node: &InNode<'_>, context: &mut BuildContext) -> Document {
    let pattern = build_node(&node.pattern(), context);
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_in_node(&LayoutParamInNode { pattern, statements })
}
#[rustfmt::skip]
fn build_index_and_write_node(node: &IndexAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match  &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let value = build_node(&node.value(), context);
    layout_index_and_write_node(&LayoutParamIndexAndWriteNode { receiver, arguments, block, value })
}
#[rustfmt::skip]
fn build_index_operator_write_node(node: &IndexOperatorWriteNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let value = build_node(&node.value(), context);
    layout_index_operator_write_node(&LayoutParamIndexOperatorWriteNode { receiver, arguments, block, value })
}
#[rustfmt::skip]
fn build_index_or_write_node(node: &IndexOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let receiver = match &node.receiver() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let value = build_node(&node.value(), context);
    layout_index_or_write_node(&LayoutParamIndexOrWriteNode { receiver, arguments, block, value })
}
#[rustfmt::skip]
fn build_index_target_node(node: &IndexTargetNode<'_>, context: &mut BuildContext) -> Document {
    build_node(&node.receiver(), context);
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_index_target_node(&LayoutParamIndexTargetNode { arguments, block })
}
#[rustfmt::skip]
fn build_instance_variable_and_write_node(node: &InstanceVariableAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_instance_variable_and_write_node(&LayoutParamInstanceVariableAndWriteNode { value })
}
#[rustfmt::skip]
fn build_instance_variable_operator_write_node(node: &InstanceVariableOperatorWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_instance_variable_operator_write_node(&LayoutParamInstanceVariableOperatorWriteNode { value })
}
#[rustfmt::skip]
fn build_instance_variable_or_write_node(node: &InstanceVariableOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_instance_variable_or_write_node(&LayoutParamInstanceVariableOrWriteNode { value })
}
#[rustfmt::skip]
fn build_instance_variable_read_node(node: &InstanceVariableReadNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_instance_variable_read_node(&LayoutParamInstanceVariableReadNode { name })
}
#[rustfmt::skip]
fn build_instance_variable_target_node(node: &InstanceVariableTargetNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_instance_variable_target_node(&LayoutParamInstanceVariableTargetNode { name })
}
#[rustfmt::skip]
fn build_instance_variable_write_node(node: &InstanceVariableWriteNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    let value = build_node(&node.value(), context);
    layout_instance_variable_write_node(&LayoutParamInstanceVariableWriteNode { name, value })
}
#[rustfmt::skip]
fn build_integer_node(node: &IntegerNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_integer(&node.value(), context);
    layout_integer_node(&LayoutParamIntegerNode { value })
}
#[rustfmt::skip]
fn build_interpolated_match_last_line_node(node: &InterpolatedMatchLastLineNode<'_>, context: &mut BuildContext) -> Document {
    let mut parts = Vec::new();
    for node in &node.parts() {
        parts.push(build_node(&node, context));
    }
    layout_interpolated_match_last_line_node(&LayoutParamInterpolatedMatchLastLineNode { parts })
}
#[rustfmt::skip]
fn build_interpolated_regular_expression_node(node: &InterpolatedRegularExpressionNode<'_>, context: &mut BuildContext) -> Document {
    let mut parts = Vec::new();
    for node in &node.parts() {
        parts.push(build_node(&node, context));
    }
    layout_interpolated_regular_expression_node(&LayoutParamInterpolatedRegularExpressionNode { parts })
}
#[rustfmt::skip]
fn build_interpolated_string_node(node: &InterpolatedStringNode<'_>, context: &mut BuildContext) -> Document {
    let mut parts = Vec::new();
    for node in &node.parts() {
        parts.push(build_node(&node, context));
    }
    layout_interpolated_string_node(&LayoutParamInterpolatedStringNode { parts })
}
#[rustfmt::skip]
fn build_interpolated_symbol_node(node: &InterpolatedSymbolNode<'_>, context: &mut BuildContext) -> Document {
    let mut parts = Vec::new();
    for node in &node.parts() {
        parts.push(build_node(&node, context));
    }
    layout_interpolated_symbol_node(&LayoutParamInterpolatedSymbolNode { parts })
}
#[rustfmt::skip]
fn build_interpolated_x_string_node(node: &InterpolatedXStringNode<'_>, context: &mut BuildContext) -> Document {
    let mut parts = Vec::new();
    for node in &node.parts() {
        parts.push(build_node(&node, context));
    }
    layout_interpolated_x_string_node(&LayoutParamInterpolatedXStringNode { parts })
}
#[rustfmt::skip]
fn build_it_local_variable_read_node(node: &ItLocalVariableReadNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(IT.to_string());
    layout_it_local_variable_read_node(&LayoutParamItLocalVariableReadNode { keyword })
}
#[rustfmt::skip]
fn build_it_parameters_node(node: &ItParametersNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    layout_it_parameters_node(&LayoutParamItParametersNode {  })
}
#[rustfmt::skip]
fn build_keyword_hash_node(node: &KeywordHashNode<'_>, context: &mut BuildContext) -> Document {
    let mut elements = Vec::new();
    for node in &node.elements() {
        elements.push(build_node(&node, context));
    }
    layout_keyword_hash_node(&LayoutParamKeywordHashNode { elements })
}
#[rustfmt::skip]
fn build_keyword_rest_parameter_node(node: &KeywordRestParameterNode<'_>, context: &mut BuildContext) -> Document {
    let name = match &node.name() {
        Some(id) => Some(build_constant_id(id, context)),
        None => None,
    };
    layout_keyword_rest_parameter_node(&LayoutParamKeywordRestParameterNode { name })
}
#[rustfmt::skip]
fn build_lambda_node(node: &LambdaNode<'_>, context: &mut BuildContext) -> Document {
    let parameters = match &node.parameters() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_lambda_node(&LayoutParamLambdaNode { parameters, body })
}
#[rustfmt::skip]
fn build_local_variable_and_write_node(node: &LocalVariableAndWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_local_variable_and_write_node(&LayoutParamLocalVariableAndWriteNode { value })
}
#[rustfmt::skip]
fn build_local_variable_operator_write_node(node: &LocalVariableOperatorWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_local_variable_operator_write_node(&LayoutParamLocalVariableOperatorWriteNode { value })
}
#[rustfmt::skip]
fn build_local_variable_or_write_node(node: &LocalVariableOrWriteNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_local_variable_or_write_node(&LayoutParamLocalVariableOrWriteNode { value })
}
#[rustfmt::skip]
fn build_local_variable_read_node(node: &LocalVariableReadNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_local_variable_read_node(&LayoutParamLocalVariableReadNode { name })
}
#[rustfmt::skip]
fn build_local_variable_target_node(node: &LocalVariableTargetNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_local_variable_target_node(&LayoutParamLocalVariableTargetNode { name })
}
#[rustfmt::skip]
fn build_local_variable_write_node(node: &LocalVariableWriteNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    let value = build_node(&node.value(), context);
    layout_local_variable_write_node(&LayoutParamLocalVariableWriteNode { name, value })
}
#[rustfmt::skip]
fn build_match_last_line_node(node: &MatchLastLineNode<'_>, context: &mut BuildContext) -> Document {
    let escaped = Document::String(escape(node.unescaped()));
    layout_match_last_line_node(&LayoutParamMatchLastLineNode { escaped })
}
#[rustfmt::skip]
fn build_match_predicate_node(node: &MatchPredicateNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    let pattern = build_node(&node.pattern(), context);
    layout_match_predicate_node(&LayoutParamMatchPredicateNode { value, pattern })
}
#[rustfmt::skip]
fn build_match_required_node(node: &MatchRequiredNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    let pattern = build_node(&node.pattern(), context);
    layout_match_required_node(&LayoutParamMatchRequiredNode { value, pattern })
}
#[rustfmt::skip]
fn build_match_write_node(node: &MatchWriteNode<'_>, context: &mut BuildContext) -> Document {
    let call = build_node(&node.call().as_node(), context);
    let mut targets = Vec::new();
    for node in &node.targets() {
        targets.push(build_node(&node, context));
    }
    layout_match_write_node(&LayoutParamMatchWriteNode { call, targets })
}
#[rustfmt::skip]
fn build_missing_node(node: &MissingNode<'_>, context: &mut BuildContext) -> Document {
    layout_missing_node(&LayoutParamMissingNode {  })
}
#[rustfmt::skip]
fn build_module_node(node: &ModuleNode<'_>, context: &mut BuildContext) -> Document {
    let constant_path = build_node(&node.constant_path(), context);
    let mut body = Vec::new();
    if let Some(node) = &node.body() {
        body.push(build_node(&node, context));
    }
    layout_module_node(&LayoutParamModuleNode { constant_path, body })
}
#[rustfmt::skip]
fn build_multi_target_node(node: &MultiTargetNode<'_>, context: &mut BuildContext) -> Document {
    let mut lefts = Vec::new();
    for node in &node.lefts() {
        lefts.push(build_node(&node, context));
    }
    let rest = match &node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut rights = Vec::new();
    for node in &node.rights() {
        rights.push(build_node(&node, context));
    };
    layout_multi_target_node(&LayoutParamMultiTargetNode { lefts, rest, rights })
}
#[rustfmt::skip]
fn build_multi_write_node(node: &MultiWriteNode<'_>, context: &mut BuildContext) -> Document {
    let mut lefts = Vec::new();
    for node in &node.lefts() {
        lefts.push(build_node(&node, context));
    }
    let rest = match &node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut rights = Vec::new();
    for node in &node.rights() {
        rights.push(build_node(&node, context));
    }
    let value = build_node(&node.value(), context);
    layout_multi_write_node(&LayoutParamMultiWriteNode { lefts, rest, rights, value })
}
#[rustfmt::skip]
fn build_next_node(node: &NextNode<'_>, context: &mut BuildContext) -> Document {
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_next_node(&LayoutParamNextNode { arguments })
}
#[rustfmt::skip]
fn build_nil_node(node: &NilNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(NIL.to_string());
    layout_nil_node(&LayoutParamNilNode { keyword })
}
#[rustfmt::skip]
fn build_no_keywords_parameter_node(node: &NoKeywordsParameterNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(format!("{}{}", SPLAT, NIL));
    layout_no_keywords_parameter_node(&LayoutParamNoKeywordsParameterNode { keyword })
}
#[rustfmt::skip]
fn build_numbered_parameters_node(node: &NumberedParametersNode<'_>, context: &mut BuildContext) -> Document {
    let maximum = Document::String(node.maximum().to_string());
    layout_numbered_parameters_node(&LayoutParamNumberedParametersNode { maximum })
}
#[rustfmt::skip]
fn build_numbered_reference_read_node(node: &NumberedReferenceReadNode<'_>, context: &mut BuildContext) -> Document {
    let number = Document::String(format!("{}{}", DOLLAR, node.number()));
    layout_numbered_reference_read_node(&LayoutParamNumberedReferenceReadNode { number })
}
#[rustfmt::skip]
fn build_optional_keyword_parameter_node(node: &OptionalKeywordParameterNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_optional_keyword_param_node(&LayoutParamOptionalKeywordParameterNode { value })
}
#[rustfmt::skip]
fn build_optional_parameter_node(node: &OptionalParameterNode<'_>, context: &mut BuildContext) -> Document {
    let value = build_node(&node.value(), context);
    layout_optional_parameter_node(&LayoutParamOptionalParameterNode { value })
}
#[rustfmt::skip]
fn build_or_node(node: &OrNode<'_>, context: &mut BuildContext) -> Document {
    let left = build_node(&node.left(), context);
    let right = build_node(&node.right(), context);
    layout_or_node(&LayoutParamOrNode { left, right })
}
#[rustfmt::skip]
fn build_parameters_node(node: &ParametersNode<'_>, context: &mut BuildContext) -> Document {
    let mut requireds = Vec::new();
    for node in &node.requireds() {
        requireds.push(build_node(&node, context));
    }
    let mut optionals = Vec::new();
    for node in &node.optionals() {
        optionals.push(build_node(&node, context));
    }
    let rest = match &node.rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let mut posts = Vec::new();
    for node in &node.posts() {
        posts.push(build_node(&node, context));
    }
    let mut keywords = Vec::new();
    for node in &node.keywords() {
        keywords.push(build_node(&node, context));
    }
    let keyword = match &node.keyword_rest() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_parameters_node(&LayoutParamParametersNode { requireds, optionals, rest, posts, keywords, keyword, block })
}
#[rustfmt::skip]
fn build_parentheses_node(node: &ParenthesesNode<'_>, context: &mut BuildContext) -> Document {
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_parentheses_node(&LayoutParamParenthesesNode { body })
}
#[rustfmt::skip]
fn build_pinned_expression_node(node: &PinnedExpressionNode<'_>, context: &mut BuildContext) -> Document {
    let expression = build_node(&node.expression(), context);
    layout_pinned_expression_node(&LayoutParamPinnedExpressionNode { expression })
}
#[rustfmt::skip]
fn build_pinned_variable_node(node: &PinnedVariableNode<'_>, context: &mut BuildContext) -> Document {
    let variable = build_node(&node.variable(), context);
    layout_pinned_variable_node(&LayoutParamPinnedVariableNode { variable })
}
#[rustfmt::skip]
fn build_post_execution_node(node: &PostExecutionNode<'_>, context: &mut BuildContext) -> Document {
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_post_execution_node(&LayoutParamPostExecutionNode { statements })
}
#[rustfmt::skip]
fn build_pre_execution_node(node: &PreExecutionNode<'_>, context: &mut BuildContext) -> Document {
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_pre_execution_node(&LayoutParamPreExecutionNode { statements })
}
#[rustfmt::skip]
fn build_program_node(node: &ProgramNode<'_>, context: &mut BuildContext) -> Document {
    let statements = build_node(&node.statements().as_node(), context);
    layout_program_node(&LayoutParamProgramNode { statements })
}
#[rustfmt::skip]
fn build_range_node(node: &RangeNode<'_>, context: &mut BuildContext) -> Document {
    let left = match &node.left() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let right = match &node.right() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_range_node(&LayoutParamRangeNode { left, right })
}
#[rustfmt::skip]
fn build_rational_node(node: &RationalNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    layout_rational_node(&LayoutParamRationalNode {  })
}
#[rustfmt::skip]
fn build_redo_node(node: &RedoNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(REDO.to_string());
    layout_redo_node(&LayoutParamRedoNode { keyword })
}
#[rustfmt::skip]
fn build_regular_expression_node(node: &RegularExpressionNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    let escaped = Document::String(escape(node.unescaped()));
    layout_regular_expression_node(&LayoutParamRegularExpressionNode { escaped })
}
#[rustfmt::skip]
fn build_required_keyword_parameter_node(node: &RequiredKeywordParameterNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_required_keyword_parameter_node(&LayoutParamRequiredKeywordParameterNode { name })
}
#[rustfmt::skip]
fn build_required_parameter_node(node: &RequiredParameterNode<'_>, context: &mut BuildContext) -> Document {
    let name = build_constant_id(&node.name(), context);
    layout_required_parameter_node(&LayoutParamRequiredParameterNode { name })
}
#[rustfmt::skip]
fn build_rescue_modifier_node(node: &RescueModifierNode<'_>, context: &mut BuildContext) -> Document {
    let expression = build_node(&node.expression(), context);
    let rescue_expression = build_node(&node.rescue_expression(), context);
    layout_rescue_modifier_node(&LayoutParamRescueModifierNode { expression, rescue_expression })
}
#[rustfmt::skip]
fn build_rescue_node(node: &RescueNode<'_>, context: &mut BuildContext) -> Document {
    let mut expressions = Vec::new();
    for node in &node.exceptions() {
        expressions.push(build_node(&node, context));
    }
    let reference = match &node.reference() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let subsequent = match &node.subsequent() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_rescue_node(&LayoutParamRescueNode { expressions, reference, statements, subsequent })
}
#[rustfmt::skip]
fn build_rest_parameter_node(node: &RestParameterNode<'_>, context: &mut BuildContext) -> Document {
    let name = match &node.name() {
        Some(id) => Some(build_constant_id(id, context)),
        None => None,
    };
    layout_rest_parameter_node(&LayoutParamRestParameterNode { name })
}
#[rustfmt::skip]
fn build_retry_node(node: &RetryNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(RETRY.to_string());
    layout_retry_node(&LayoutParamRetryNode { keyword })
}
#[rustfmt::skip]
fn build_return_node(node: &ReturnNode<'_>, context: &mut BuildContext) -> Document {
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_return_node(&LayoutParamReturnNode { arguments })
}
#[rustfmt::skip]
fn build_self_node(node: &SelfNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(SELF.to_string());
    layout_self_node(&LayoutParamSelfNode { keyword })
}
#[rustfmt::skip]
fn build_shareable_constant_node(node: &ShareableConstantNode<'_>, context: &mut BuildContext) -> Document {
    let write = build_node(&node.write(), context);
    layout_shareable_constant_node(&LayoutParamShareableConstantNode { write })
}
#[rustfmt::skip]
fn build_singleton_class_node(node: &SingletonClassNode<'_>, context: &mut BuildContext) -> Document {
    let expression = build_node(&node.expression(), context);
    let body = match &node.body() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_singleton_class_node(&LayoutParamSingletonClassNode { expression, body })
}
#[rustfmt::skip]
fn build_source_encoding_node(node: &SourceEncodingNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(ENCODING.to_string());
    layout_source_encoding_node(&LayoutParamSourceEncodingNode { keyword })
}
#[rustfmt::skip]
fn build_source_file_node(node: &SourceFileNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(FILE.to_string());
    layout_source_file_node(&LayoutParamSourceFileNode { keyword })
}
#[rustfmt::skip]
fn build_source_line_node(node: &SourceLineNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(LINE.to_string());
    layout_source_line_node(&LayoutParamSourceLineNode { keyword })
}
#[rustfmt::skip]
fn build_splat_node(node: &SplatNode<'_>, context: &mut BuildContext) -> Document {
    let expression = match &node.expression() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_splat_node(&LayoutParamSplatNode { expression })
}
#[rustfmt::skip]
fn build_statements_node(node: &StatementsNode<'_>, context: &mut BuildContext) -> Document {
    let mut body = Vec::new();
    for node in &node.body() {
        body.push(build_node(&node, context));
    }
    layout_statements_node(&LayoutParamStatementsNode { body })
}
#[rustfmt::skip]
fn build_string_node(node: &StringNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    let escaped = Document::String(escape(node.unescaped()));
    layout_string_node(&LayoutParamStringNode { escaped })
}
#[rustfmt::skip]
fn build_super_node(node: &SuperNode<'_>, context: &mut BuildContext) -> Document {
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let block = match &node.block() {
        Some(node) => Some(build_node(&node, context)),
        None => None,
    };
    layout_super_node(&LayoutParamSuperNode { arguments, block })
}
#[rustfmt::skip]
fn build_symbol_node(node: &SymbolNode<'_>, context: &mut BuildContext) -> Document {
    let escaped = Document::String(escape(node.unescaped()));
    layout_symbol_node(&LayoutParamSymbolNode { escaped })
}
#[rustfmt::skip]
fn build_true_node(node: &TrueNode<'_>, context: &mut BuildContext) -> Document {
    let keyword = Document::String(TRUE.to_string());
    layout_true_node(&LayoutParamTrueNode { keyword })
}
#[rustfmt::skip]
fn build_undef_node(node: &UndefNode<'_>, context: &mut BuildContext) -> Document {
    let mut names = Vec::new();
    for node in &node.names() {
        names.push(build_node(&node, context));
    }
    layout_undef_node(&LayoutParamUndefNode { names })
}
#[rustfmt::skip]
fn build_unless_node(node: &UnlessNode<'_>, context: &mut BuildContext) -> Document {
    let predicate = build_node(&node.predicate(), context);
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    let else_clause = match &node.else_clause() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_unless_node(&LayoutParamUnlessNode { predicate, statements, else_clause })
}
#[rustfmt::skip]
fn build_until_node(node: &UntilNode<'_>, context: &mut BuildContext) -> Document {
    let predicate = build_node(&node.predicate(), context);
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_until_node(&LayoutParamUntilNode { predicate, statements })
}
#[rustfmt::skip]
fn build_when_node(node: &WhenNode<'_>, context: &mut BuildContext) -> Document {
    let mut conditions = Vec::new();
    for node in &node.conditions() {
        conditions.push(build_node(&node, context));
    }
    let statements = match &node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_when_node(&LayoutParamWhenNode { conditions, statements })
}
#[rustfmt::skip]
fn build_while_node(node: &WhileNode<'_>, context: &mut BuildContext) -> Document {
    let predicate = build_node(&node.predicate(), context);
    let statements = match node.statements() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_while_node(&LayoutParamWhileNode { predicate, statements })
}
#[rustfmt::skip]
fn build_x_string_node(node: &XStringNode<'_>, context: &mut BuildContext) -> Document {
    // TODO
    let escaped = Document::String(escape(node.unescaped()));
    layout_x_string_node(&LayoutParamXStringNode { escaped })
}
#[rustfmt::skip]
fn build_yield_node(node: &YieldNode<'_>, context: &mut BuildContext) -> Document {
    let arguments = match &node.arguments() {
        Some(node) => Some(build_node(&node.as_node(), context)),
        None => None,
    };
    layout_yield_node(&LayoutParamYieldNode { arguments })
}
