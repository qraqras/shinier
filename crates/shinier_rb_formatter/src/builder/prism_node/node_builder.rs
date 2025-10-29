use crate::{BuildContext, Document};
use ruby_prism::*;

pub trait Build {
    fn __build__(&self, context: &mut BuildContext) -> Document;
    fn build(&self, context: &mut BuildContext) -> Document {
        self.__build__(context)
    }
    fn build_with(
        &self,
        context: &mut BuildContext,
        before: Option<Document>,
        after: Option<Document>,
    ) -> Document {
        let before = before.unwrap_or(Document::None);
        let after = after.unwrap_or(Document::None);
        Document::Array(Vec::from([before, self.build(context), after]))
    }
}

pub trait ListBuild {
    fn __build__(&self, context: &mut BuildContext, separator: &Document) -> Document;
    fn build(&self, context: &mut BuildContext, separator: &Document) -> Document {
        self.__build__(context, separator)
    }
    fn build_with(
        &self,
        context: &mut BuildContext,
        separator: &Document,
        before: Option<Document>,
        after: Option<Document>,
    ) -> Document {
        let before = before.unwrap_or(Document::None);
        let after = after.unwrap_or(Document::None);
        Document::Array(Vec::from(&[before, self.build(context, separator), after]))
    }
}

impl<'sh> Build for Node<'sh> {
    #[rustfmt::skip]
    fn __build__(&self, context: &mut BuildContext) -> Document {
        match self {
            Node::AliasGlobalVariableNode           { .. } => self.as_alias_global_variable_node().as_ref().build(context),
            Node::AliasMethodNode                   { .. } => self.as_alias_method_node().as_ref().build(context),
            Node::AlternationPatternNode            { .. } => self.as_alternation_pattern_node().as_ref().build(context),
            Node::AndNode                           { .. } => self.as_and_node().as_ref().build(context),
            Node::ArgumentsNode                     { .. } => self.as_arguments_node().as_ref().build(context),
            Node::ArrayNode                         { .. } => self.as_array_node().as_ref().build(context),
            Node::ArrayPatternNode                  { .. } => self.as_array_pattern_node().as_ref().build(context),
            Node::AssocNode                         { .. } => self.as_assoc_node().as_ref().build(context),
            Node::AssocSplatNode                    { .. } => self.as_assoc_splat_node().as_ref().build(context),
            Node::BackReferenceReadNode             { .. } => self.as_back_reference_read_node().as_ref().build(context),
            Node::BeginNode                         { .. } => self.as_begin_node().as_ref().build(context),
            Node::BlockArgumentNode                 { .. } => self.as_block_argument_node().as_ref().build(context),
            Node::BlockLocalVariableNode            { .. } => self.as_block_local_variable_node().as_ref().build(context),
            Node::BlockNode                         { .. } => self.as_block_node().as_ref().build(context),
            Node::BlockParameterNode                { .. } => self.as_block_parameter_node().as_ref().build(context),
            Node::BlockParametersNode               { .. } => self.as_block_parameters_node().as_ref().build(context),
            Node::BreakNode                         { .. } => self.as_break_node().as_ref().build(context),
            Node::CallAndWriteNode                  { .. } => self.as_call_and_write_node().as_ref().build(context),
            Node::CallNode                          { .. } => self.as_call_node().as_ref().build(context),
            Node::CallOperatorWriteNode             { .. } => self.as_call_operator_write_node().as_ref().build(context),
            Node::CallOrWriteNode                   { .. } => self.as_call_or_write_node().as_ref().build(context),
            Node::CallTargetNode                    { .. } => self.as_call_target_node().as_ref().build(context),
            Node::CapturePatternNode                { .. } => self.as_capture_pattern_node().as_ref().build(context),
            Node::CaseMatchNode                     { .. } => self.as_case_match_node().as_ref().build(context),
            Node::CaseNode                          { .. } => self.as_case_node().as_ref().build(context),
            Node::ClassNode                         { .. } => self.as_class_node().as_ref().build(context),
            Node::ClassVariableAndWriteNode         { .. } => self.as_class_variable_and_write_node().as_ref().build(context),
            Node::ClassVariableOperatorWriteNode    { .. } => self.as_class_variable_operator_write_node().as_ref().build(context),
            Node::ClassVariableOrWriteNode          { .. } => self.as_class_variable_or_write_node().as_ref().build(context),
            Node::ClassVariableReadNode             { .. } => self.as_class_variable_read_node().as_ref().build(context),
            Node::ClassVariableTargetNode           { .. } => self.as_class_variable_target_node().as_ref().build(context),
            Node::ClassVariableWriteNode            { .. } => self.as_class_variable_write_node().as_ref().build(context),
            Node::ConstantAndWriteNode              { .. } => self.as_constant_and_write_node().as_ref().build(context),
            Node::ConstantOperatorWriteNode         { .. } => self.as_constant_operator_write_node().as_ref().build(context),
            Node::ConstantOrWriteNode               { .. } => self.as_constant_or_write_node().as_ref().build(context),
            Node::ConstantPathAndWriteNode          { .. } => self.as_constant_path_and_write_node().as_ref().build(context),
            Node::ConstantPathNode                  { .. } => self.as_constant_path_node().as_ref().build(context),
            Node::ConstantPathOperatorWriteNode     { .. } => self.as_constant_path_operator_write_node().as_ref().build(context),
            Node::ConstantPathOrWriteNode           { .. } => self.as_constant_path_or_write_node().as_ref().build(context),
            Node::ConstantPathTargetNode            { .. } => self.as_constant_path_target_node().as_ref().build(context),
            Node::ConstantPathWriteNode             { .. } => self.as_constant_path_write_node().as_ref().build(context),
            Node::ConstantReadNode                  { .. } => self.as_constant_read_node().as_ref().build(context),
            Node::ConstantTargetNode                { .. } => self.as_constant_target_node().as_ref().build(context),
            Node::ConstantWriteNode                 { .. } => self.as_constant_write_node().as_ref().build(context),
            Node::DefNode                           { .. } => self.as_def_node().as_ref().build(context),
            Node::DefinedNode                       { .. } => self.as_defined_node().as_ref().build(context),
            Node::ElseNode                          { .. } => self.as_else_node().as_ref().build(context),
            Node::EmbeddedStatementsNode            { .. } => self.as_embedded_statements_node().as_ref().build(context),
            Node::EmbeddedVariableNode              { .. } => self.as_embedded_variable_node().as_ref().build(context),
            Node::EnsureNode                        { .. } => self.as_ensure_node().as_ref().build(context),
            Node::FalseNode                         { .. } => self.as_false_node().as_ref().build(context),
            Node::FindPatternNode                   { .. } => self.as_find_pattern_node().as_ref().build(context),
            Node::FlipFlopNode                      { .. } => self.as_flip_flop_node().as_ref().build(context),
            Node::FloatNode                         { .. } => self.as_float_node().as_ref().build(context),
            Node::ForNode                           { .. } => self.as_for_node().as_ref().build(context),
            Node::ForwardingArgumentsNode           { .. } => self.as_forwarding_arguments_node().as_ref().build(context),
            Node::ForwardingParameterNode           { .. } => self.as_forwarding_parameter_node().as_ref().build(context),
            Node::ForwardingSuperNode               { .. } => self.as_forwarding_super_node().as_ref().build(context),
            Node::GlobalVariableAndWriteNode        { .. } => self.as_global_variable_and_write_node().as_ref().build(context),
            Node::GlobalVariableOperatorWriteNode   { .. } => self.as_global_variable_operator_write_node().as_ref().build(context),
            Node::GlobalVariableOrWriteNode         { .. } => self.as_global_variable_or_write_node().as_ref().build(context),
            Node::GlobalVariableReadNode            { .. } => self.as_global_variable_read_node().as_ref().build(context),
            Node::GlobalVariableTargetNode          { .. } => self.as_global_variable_target_node().as_ref().build(context),
            Node::GlobalVariableWriteNode           { .. } => self.as_global_variable_write_node().as_ref().build(context),
            Node::HashNode                          { .. } => self.as_hash_node().as_ref().build(context),
            Node::HashPatternNode                   { .. } => self.as_hash_pattern_node().as_ref().build(context),
            Node::IfNode                            { .. } => self.as_if_node().as_ref().build(context),
            Node::ImaginaryNode                     { .. } => self.as_imaginary_node().as_ref().build(context),
            Node::ImplicitNode                      { .. } => self.as_implicit_node().as_ref().build(context),
            Node::ImplicitRestNode                  { .. } => self.as_implicit_rest_node().as_ref().build(context),
            Node::InNode                            { .. } => self.as_in_node().as_ref().build(context),
            Node::IndexAndWriteNode                 { .. } => self.as_index_and_write_node().as_ref().build(context),
            Node::IndexOperatorWriteNode            { .. } => self.as_index_operator_write_node().as_ref().build(context),
            Node::IndexOrWriteNode                  { .. } => self.as_index_or_write_node().as_ref().build(context),
            Node::IndexTargetNode                   { .. } => self.as_index_target_node().as_ref().build(context),
            Node::InstanceVariableAndWriteNode      { .. } => self.as_instance_variable_and_write_node().as_ref().build(context),
            Node::InstanceVariableOperatorWriteNode { .. } => self.as_instance_variable_operator_write_node().as_ref().build(context),
            Node::InstanceVariableOrWriteNode       { .. } => self.as_instance_variable_or_write_node().as_ref().build(context),
            Node::InstanceVariableReadNode          { .. } => self.as_instance_variable_read_node().as_ref().build(context),
            Node::InstanceVariableTargetNode        { .. } => self.as_instance_variable_target_node().as_ref().build(context),
            Node::InstanceVariableWriteNode         { .. } => self.as_instance_variable_write_node().as_ref().build(context),
            Node::IntegerNode                       { .. } => self.as_integer_node().as_ref().build(context),
            Node::InterpolatedMatchLastLineNode     { .. } => self.as_interpolated_match_last_line_node().as_ref().build(context),
            Node::InterpolatedRegularExpressionNode { .. } => self.as_interpolated_regular_expression_node().as_ref().build(context),
            Node::InterpolatedStringNode            { .. } => self.as_interpolated_string_node().as_ref().build(context),
            Node::InterpolatedSymbolNode            { .. } => self.as_interpolated_symbol_node().as_ref().build(context),
            Node::InterpolatedXStringNode           { .. } => self.as_interpolated_x_string_node().as_ref().build(context),
            Node::ItLocalVariableReadNode           { .. } => self.as_it_local_variable_read_node().as_ref().build(context),
            Node::ItParametersNode                  { .. } => self.as_it_parameters_node().as_ref().build(context),
            Node::KeywordHashNode                   { .. } => self.as_keyword_hash_node().as_ref().build(context),
            Node::KeywordRestParameterNode          { .. } => self.as_keyword_rest_parameter_node().as_ref().build(context),
            Node::LambdaNode                        { .. } => self.as_lambda_node().as_ref().build(context),
            Node::LocalVariableAndWriteNode         { .. } => self.as_local_variable_and_write_node().as_ref().build(context),
            Node::LocalVariableOperatorWriteNode    { .. } => self.as_local_variable_operator_write_node().as_ref().build(context),
            Node::LocalVariableOrWriteNode          { .. } => self.as_local_variable_or_write_node().as_ref().build(context),
            Node::LocalVariableReadNode             { .. } => self.as_local_variable_read_node().as_ref().build(context),
            Node::LocalVariableTargetNode           { .. } => self.as_local_variable_target_node().as_ref().build(context),
            Node::LocalVariableWriteNode            { .. } => self.as_local_variable_write_node().as_ref().build(context),
            Node::MatchLastLineNode                 { .. } => self.as_match_last_line_node().as_ref().build(context),
            Node::MatchPredicateNode                { .. } => self.as_match_predicate_node().as_ref().build(context),
            Node::MatchRequiredNode                 { .. } => self.as_match_required_node().as_ref().build(context),
            Node::MatchWriteNode                    { .. } => self.as_match_write_node().as_ref().build(context),
            Node::MissingNode                       { .. } => self.as_missing_node().as_ref().build(context),
            Node::ModuleNode                        { .. } => self.as_module_node().as_ref().build(context),
            Node::MultiTargetNode                   { .. } => self.as_multi_target_node().as_ref().build(context),
            Node::MultiWriteNode                    { .. } => self.as_multi_write_node().as_ref().build(context),
            Node::NextNode                          { .. } => self.as_next_node().as_ref().build(context),
            Node::NilNode                           { .. } => self.as_nil_node().as_ref().build(context),
            Node::NoKeywordsParameterNode           { .. } => self.as_no_keywords_parameter_node().as_ref().build(context),
            Node::NumberedParametersNode            { .. } => self.as_numbered_parameters_node().as_ref().build(context),
            Node::NumberedReferenceReadNode         { .. } => self.as_numbered_reference_read_node().as_ref().build(context),
            Node::OptionalKeywordParameterNode      { .. } => self.as_optional_keyword_parameter_node().as_ref().build(context),
            Node::OptionalParameterNode             { .. } => self.as_optional_parameter_node().as_ref().build(context),
            Node::OrNode                            { .. } => self.as_or_node().as_ref().build(context),
            Node::ParametersNode                    { .. } => self.as_parameters_node().as_ref().build(context),
            Node::ParenthesesNode                   { .. } => self.as_parentheses_node().as_ref().build(context),
            Node::PinnedExpressionNode              { .. } => self.as_pinned_expression_node().as_ref().build(context),
            Node::PinnedVariableNode                { .. } => self.as_pinned_variable_node().as_ref().build(context),
            Node::PostExecutionNode                 { .. } => self.as_post_execution_node().as_ref().build(context),
            Node::PreExecutionNode                  { .. } => self.as_pre_execution_node().as_ref().build(context),
            Node::ProgramNode                       { .. } => self.as_program_node().as_ref().build(context),
            Node::RangeNode                         { .. } => self.as_range_node().as_ref().build(context),
            Node::RationalNode                      { .. } => self.as_rational_node().as_ref().build(context),
            Node::RedoNode                          { .. } => self.as_redo_node().as_ref().build(context),
            Node::RegularExpressionNode             { .. } => self.as_regular_expression_node().as_ref().build(context),
            Node::RequiredKeywordParameterNode      { .. } => self.as_required_keyword_parameter_node().as_ref().build(context),
            Node::RequiredParameterNode             { .. } => self.as_required_parameter_node().as_ref().build(context),
            Node::RescueModifierNode                { .. } => self.as_rescue_modifier_node().as_ref().build(context),
            Node::RescueNode                        { .. } => self.as_rescue_node().as_ref().build(context),
            Node::RestParameterNode                 { .. } => self.as_rest_parameter_node().as_ref().build(context),
            Node::RetryNode                         { .. } => self.as_retry_node().as_ref().build(context),
            Node::ReturnNode                        { .. } => self.as_return_node().as_ref().build(context),
            Node::SelfNode                          { .. } => self.as_self_node().as_ref().build(context),
            Node::ShareableConstantNode             { .. } => self.as_shareable_constant_node().as_ref().build(context),
            Node::SingletonClassNode                { .. } => self.as_singleton_class_node().as_ref().build(context),
            Node::SourceEncodingNode                { .. } => self.as_source_encoding_node().as_ref().build(context),
            Node::SourceFileNode                    { .. } => self.as_source_file_node().as_ref().build(context),
            Node::SourceLineNode                    { .. } => self.as_source_line_node().as_ref().build(context),
            Node::SplatNode                         { .. } => self.as_splat_node().as_ref().build(context),
            Node::StatementsNode                    { .. } => self.as_statements_node().as_ref().build(context),
            Node::StringNode                        { .. } => self.as_string_node().as_ref().build(context),
            Node::SuperNode                         { .. } => self.as_super_node().as_ref().build(context),
            Node::SymbolNode                        { .. } => self.as_symbol_node().as_ref().build(context),
            Node::TrueNode                          { .. } => self.as_true_node().as_ref().build(context),
            Node::UndefNode                         { .. } => self.as_undef_node().as_ref().build(context),
            Node::UnlessNode                        { .. } => self.as_unless_node().as_ref().build(context),
            Node::UntilNode                         { .. } => self.as_until_node().as_ref().build(context),
            Node::WhenNode                          { .. } => self.as_when_node().as_ref().build(context),
            Node::WhileNode                         { .. } => self.as_while_node().as_ref().build(context),
            Node::XStringNode                       { .. } => self.as_x_string_node().as_ref().build(context),
            Node::YieldNode                         { .. } => self.as_yield_node().as_ref().build(context),
        }
    }
}

impl<'sh> Build for Option<Node<'sh>> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.build(context),
            None => Document::None,
        }
    }
}

impl ListBuild for NodeList<'_> {
    fn __build__(&self, context: &mut BuildContext, separator: &Document) -> Document {
        if self.iter().next().is_none() {
            return Document::None;
        }
        let mut vec = Vec::new();
        for (i, node) in self.iter().enumerate() {
            if i > 0 {
                vec.push(separator.clone());
            }
            vec.push(node.build(context));
        }
        Document::Array(vec)
    }
}

impl<'a> ListBuild for Option<NodeList<'_>> {
    fn __build__(&self, context: &mut BuildContext, separator: &Document) -> Document {
        match self {
            Some(node) => node.__build__(context, separator),
            None => Document::None,
        }
    }
}
