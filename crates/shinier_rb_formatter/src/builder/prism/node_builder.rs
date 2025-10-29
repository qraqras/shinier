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
        #[rustfmt::skip]
        fn build_node(node: &Node, context: &mut BuildContext) -> Document {
            match node {
                Node::AliasGlobalVariableNode           { .. } => node.as_alias_global_variable_node().as_ref().build(context),
                Node::AliasMethodNode                   { .. } => node.as_alias_method_node().as_ref().build(context),
                Node::AlternationPatternNode            { .. } => node.as_alternation_pattern_node().as_ref().build(context),
                Node::AndNode                           { .. } => node.as_and_node().as_ref().build(context),
                Node::ArgumentsNode                     { .. } => node.as_arguments_node().as_ref().build(context),
                Node::ArrayNode                         { .. } => node.as_array_node().as_ref().build(context),
                Node::ArrayPatternNode                  { .. } => node.as_array_pattern_node().as_ref().build(context),
                Node::AssocNode                         { .. } => node.as_assoc_node().as_ref().build(context),
                Node::AssocSplatNode                    { .. } => node.as_assoc_splat_node().as_ref().build(context),
                Node::BackReferenceReadNode             { .. } => node.as_back_reference_read_node().as_ref().build(context),
                Node::BeginNode                         { .. } => node.as_begin_node().as_ref().build(context),
                Node::BlockArgumentNode                 { .. } => node.as_block_argument_node().as_ref().build(context),
                Node::BlockLocalVariableNode            { .. } => node.as_block_local_variable_node().as_ref().build(context),
                Node::BlockNode                         { .. } => node.as_block_node().as_ref().build(context),
                Node::BlockParameterNode                { .. } => node.as_block_parameter_node().as_ref().build(context),
                Node::BlockParametersNode               { .. } => node.as_block_parameters_node().as_ref().build(context),
                Node::BreakNode                         { .. } => node.as_break_node().as_ref().build(context),
                Node::CallAndWriteNode                  { .. } => node.as_call_and_write_node().as_ref().build(context),
                Node::CallNode                          { .. } => node.as_call_node().as_ref().build(context),
                Node::CallOperatorWriteNode             { .. } => node.as_call_operator_write_node().as_ref().build(context),
                Node::CallOrWriteNode                   { .. } => node.as_call_or_write_node().as_ref().build(context),
                Node::CallTargetNode                    { .. } => node.as_call_target_node().as_ref().build(context),
                Node::CapturePatternNode                { .. } => node.as_capture_pattern_node().as_ref().build(context),
                Node::CaseMatchNode                     { .. } => node.as_case_match_node().as_ref().build(context),
                Node::CaseNode                          { .. } => node.as_case_node().as_ref().build(context),
                Node::ClassNode                         { .. } => node.as_class_node().as_ref().build(context),
                Node::ClassVariableAndWriteNode         { .. } => node.as_class_variable_and_write_node().as_ref().build(context),
                Node::ClassVariableOperatorWriteNode    { .. } => node.as_class_variable_operator_write_node().as_ref().build(context),
                Node::ClassVariableOrWriteNode          { .. } => node.as_class_variable_or_write_node().as_ref().build(context),
                Node::ClassVariableReadNode             { .. } => node.as_class_variable_read_node().as_ref().build(context),
                Node::ClassVariableTargetNode           { .. } => node.as_class_variable_target_node().as_ref().build(context),
                Node::ClassVariableWriteNode            { .. } => node.as_class_variable_write_node().as_ref().build(context),
                Node::ConstantAndWriteNode              { .. } => node.as_constant_and_write_node().as_ref().build(context),
                Node::ConstantOperatorWriteNode         { .. } => node.as_constant_operator_write_node().as_ref().build(context),
                Node::ConstantOrWriteNode               { .. } => node.as_constant_or_write_node().as_ref().build(context),
                Node::ConstantPathAndWriteNode          { .. } => node.as_constant_path_and_write_node().as_ref().build(context),
                Node::ConstantPathNode                  { .. } => node.as_constant_path_node().as_ref().build(context),
                Node::ConstantPathOperatorWriteNode     { .. } => node.as_constant_path_operator_write_node().as_ref().build(context),
                Node::ConstantPathOrWriteNode           { .. } => node.as_constant_path_or_write_node().as_ref().build(context),
                Node::ConstantPathTargetNode            { .. } => node.as_constant_path_target_node().as_ref().build(context),
                Node::ConstantPathWriteNode             { .. } => node.as_constant_path_write_node().as_ref().build(context),
                Node::ConstantReadNode                  { .. } => node.as_constant_read_node().as_ref().build(context),
                Node::ConstantTargetNode                { .. } => node.as_constant_target_node().as_ref().build(context),
                Node::ConstantWriteNode                 { .. } => node.as_constant_write_node().as_ref().build(context),
                Node::DefNode                           { .. } => node.as_def_node().as_ref().build(context),
                Node::DefinedNode                       { .. } => node.as_defined_node().as_ref().build(context),
                Node::ElseNode                          { .. } => node.as_else_node().as_ref().build(context),
                Node::EmbeddedStatementsNode            { .. } => node.as_embedded_statements_node().as_ref().build(context),
                Node::EmbeddedVariableNode              { .. } => node.as_embedded_variable_node().as_ref().build(context),
                Node::EnsureNode                        { .. } => node.as_ensure_node().as_ref().build(context),
                Node::FalseNode                         { .. } => node.as_false_node().as_ref().build(context),
                Node::FindPatternNode                   { .. } => node.as_find_pattern_node().as_ref().build(context),
                Node::FlipFlopNode                      { .. } => node.as_flip_flop_node().as_ref().build(context),
                Node::FloatNode                         { .. } => node.as_float_node().as_ref().build(context),
                Node::ForNode                           { .. } => node.as_for_node().as_ref().build(context),
                Node::ForwardingArgumentsNode           { .. } => node.as_forwarding_arguments_node().as_ref().build(context),
                Node::ForwardingParameterNode           { .. } => node.as_forwarding_parameter_node().as_ref().build(context),
                Node::ForwardingSuperNode               { .. } => node.as_forwarding_super_node().as_ref().build(context),
                Node::GlobalVariableAndWriteNode        { .. } => node.as_global_variable_and_write_node().as_ref().build(context),
                Node::GlobalVariableOperatorWriteNode   { .. } => node.as_global_variable_operator_write_node().as_ref().build(context),
                Node::GlobalVariableOrWriteNode         { .. } => node.as_global_variable_or_write_node().as_ref().build(context),
                Node::GlobalVariableReadNode            { .. } => node.as_global_variable_read_node().as_ref().build(context),
                Node::GlobalVariableTargetNode          { .. } => node.as_global_variable_target_node().as_ref().build(context),
                Node::GlobalVariableWriteNode           { .. } => node.as_global_variable_write_node().as_ref().build(context),
                Node::HashNode                          { .. } => node.as_hash_node().as_ref().build(context),
                Node::HashPatternNode                   { .. } => node.as_hash_pattern_node().as_ref().build(context),
                Node::IfNode                            { .. } => node.as_if_node().as_ref().build(context),
                Node::ImaginaryNode                     { .. } => node.as_imaginary_node().as_ref().build(context),
                Node::ImplicitNode                      { .. } => node.as_implicit_node().as_ref().build(context),
                Node::ImplicitRestNode                  { .. } => node.as_implicit_rest_node().as_ref().build(context),
                Node::InNode                            { .. } => node.as_in_node().as_ref().build(context),
                Node::IndexAndWriteNode                 { .. } => node.as_index_and_write_node().as_ref().build(context),
                Node::IndexOperatorWriteNode            { .. } => node.as_index_operator_write_node().as_ref().build(context),
                Node::IndexOrWriteNode                  { .. } => node.as_index_or_write_node().as_ref().build(context),
                Node::IndexTargetNode                   { .. } => node.as_index_target_node().as_ref().build(context),
                Node::InstanceVariableAndWriteNode      { .. } => node.as_instance_variable_and_write_node().as_ref().build(context),
                Node::InstanceVariableOperatorWriteNode { .. } => node.as_instance_variable_operator_write_node().as_ref().build(context),
                Node::InstanceVariableOrWriteNode       { .. } => node.as_instance_variable_or_write_node().as_ref().build(context),
                Node::InstanceVariableReadNode          { .. } => node.as_instance_variable_read_node().as_ref().build(context),
                Node::InstanceVariableTargetNode        { .. } => node.as_instance_variable_target_node().as_ref().build(context),
                Node::InstanceVariableWriteNode         { .. } => node.as_instance_variable_write_node().as_ref().build(context),
                Node::IntegerNode                       { .. } => node.as_integer_node().as_ref().build(context),
                Node::InterpolatedMatchLastLineNode     { .. } => node.as_interpolated_match_last_line_node().as_ref().build(context),
                Node::InterpolatedRegularExpressionNode { .. } => node.as_interpolated_regular_expression_node().as_ref().build(context),
                Node::InterpolatedStringNode            { .. } => node.as_interpolated_string_node().as_ref().build(context),
                Node::InterpolatedSymbolNode            { .. } => node.as_interpolated_symbol_node().as_ref().build(context),
                Node::InterpolatedXStringNode           { .. } => node.as_interpolated_x_string_node().as_ref().build(context),
                Node::ItLocalVariableReadNode           { .. } => node.as_it_local_variable_read_node().as_ref().build(context),
                Node::ItParametersNode                  { .. } => node.as_it_parameters_node().as_ref().build(context),
                Node::KeywordHashNode                   { .. } => node.as_keyword_hash_node().as_ref().build(context),
                Node::KeywordRestParameterNode          { .. } => node.as_keyword_rest_parameter_node().as_ref().build(context),
                Node::LambdaNode                        { .. } => node.as_lambda_node().as_ref().build(context),
                Node::LocalVariableAndWriteNode         { .. } => node.as_local_variable_and_write_node().as_ref().build(context),
                Node::LocalVariableOperatorWriteNode    { .. } => node.as_local_variable_operator_write_node().as_ref().build(context),
                Node::LocalVariableOrWriteNode          { .. } => node.as_local_variable_or_write_node().as_ref().build(context),
                Node::LocalVariableReadNode             { .. } => node.as_local_variable_read_node().as_ref().build(context),
                Node::LocalVariableTargetNode           { .. } => node.as_local_variable_target_node().as_ref().build(context),
                Node::LocalVariableWriteNode            { .. } => node.as_local_variable_write_node().as_ref().build(context),
                Node::MatchLastLineNode                 { .. } => node.as_match_last_line_node().as_ref().build(context),
                Node::MatchPredicateNode                { .. } => node.as_match_predicate_node().as_ref().build(context),
                Node::MatchRequiredNode                 { .. } => node.as_match_required_node().as_ref().build(context),
                Node::MatchWriteNode                    { .. } => node.as_match_write_node().as_ref().build(context),
                Node::MissingNode                       { .. } => node.as_missing_node().as_ref().build(context),
                Node::ModuleNode                        { .. } => node.as_module_node().as_ref().build(context),
                Node::MultiTargetNode                   { .. } => node.as_multi_target_node().as_ref().build(context),
                Node::MultiWriteNode                    { .. } => node.as_multi_write_node().as_ref().build(context),
                Node::NextNode                          { .. } => node.as_next_node().as_ref().build(context),
                Node::NilNode                           { .. } => node.as_nil_node().as_ref().build(context),
                Node::NoKeywordsParameterNode           { .. } => node.as_no_keywords_parameter_node().as_ref().build(context),
                Node::NumberedParametersNode            { .. } => node.as_numbered_parameters_node().as_ref().build(context),
                Node::NumberedReferenceReadNode         { .. } => node.as_numbered_reference_read_node().as_ref().build(context),
                Node::OptionalKeywordParameterNode      { .. } => node.as_optional_keyword_parameter_node().as_ref().build(context),
                Node::OptionalParameterNode             { .. } => node.as_optional_parameter_node().as_ref().build(context),
                Node::OrNode                            { .. } => node.as_or_node().as_ref().build(context),
                Node::ParametersNode                    { .. } => node.as_parameters_node().as_ref().build(context),
                Node::ParenthesesNode                   { .. } => node.as_parentheses_node().as_ref().build(context),
                Node::PinnedExpressionNode              { .. } => node.as_pinned_expression_node().as_ref().build(context),
                Node::PinnedVariableNode                { .. } => node.as_pinned_variable_node().as_ref().build(context),
                Node::PostExecutionNode                 { .. } => node.as_post_execution_node().as_ref().build(context),
                Node::PreExecutionNode                  { .. } => node.as_pre_execution_node().as_ref().build(context),
                Node::ProgramNode                       { .. } => node.as_program_node().as_ref().build(context),
                Node::RangeNode                         { .. } => node.as_range_node().as_ref().build(context),
                Node::RationalNode                      { .. } => node.as_rational_node().as_ref().build(context),
                Node::RedoNode                          { .. } => node.as_redo_node().as_ref().build(context),
                Node::RegularExpressionNode             { .. } => node.as_regular_expression_node().as_ref().build(context),
                Node::RequiredKeywordParameterNode      { .. } => node.as_required_keyword_parameter_node().as_ref().build(context),
                Node::RequiredParameterNode             { .. } => node.as_required_parameter_node().as_ref().build(context),
                Node::RescueModifierNode                { .. } => node.as_rescue_modifier_node().as_ref().build(context),
                Node::RescueNode                        { .. } => node.as_rescue_node().as_ref().build(context),
                Node::RestParameterNode                 { .. } => node.as_rest_parameter_node().as_ref().build(context),
                Node::RetryNode                         { .. } => node.as_retry_node().as_ref().build(context),
                Node::ReturnNode                        { .. } => node.as_return_node().as_ref().build(context),
                Node::SelfNode                          { .. } => node.as_self_node().as_ref().build(context),
                Node::ShareableConstantNode             { .. } => node.as_shareable_constant_node().as_ref().build(context),
                Node::SingletonClassNode                { .. } => node.as_singleton_class_node().as_ref().build(context),
                Node::SourceEncodingNode                { .. } => node.as_source_encoding_node().as_ref().build(context),
                Node::SourceFileNode                    { .. } => node.as_source_file_node().as_ref().build(context),
                Node::SourceLineNode                    { .. } => node.as_source_line_node().as_ref().build(context),
                Node::SplatNode                         { .. } => node.as_splat_node().as_ref().build(context),
                Node::StatementsNode                    { .. } => node.as_statements_node().as_ref().build(context),
                Node::StringNode                        { .. } => node.as_string_node().as_ref().build(context),
                Node::SuperNode                         { .. } => node.as_super_node().as_ref().build(context),
                Node::SymbolNode                        { .. } => node.as_symbol_node().as_ref().build(context),
                Node::TrueNode                          { .. } => node.as_true_node().as_ref().build(context),
                Node::UndefNode                         { .. } => node.as_undef_node().as_ref().build(context),
                Node::UnlessNode                        { .. } => node.as_unless_node().as_ref().build(context),
                Node::UntilNode                         { .. } => node.as_until_node().as_ref().build(context),
                Node::WhenNode                          { .. } => node.as_when_node().as_ref().build(context),
                Node::WhileNode                         { .. } => node.as_while_node().as_ref().build(context),
                Node::XStringNode                       { .. } => node.as_x_string_node().as_ref().build(context),
                Node::YieldNode                         { .. } => node.as_yield_node().as_ref().build(context),
            }
        }
        build_node(self, context)
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
