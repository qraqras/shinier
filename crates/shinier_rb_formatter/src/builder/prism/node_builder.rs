use crate::Document;
use crate::builder::NodeVariant;
use crate::builder::comments_builder::{build_leading_comments, build_trailing_comments};
use crate::builder::line_breaks_builder::build_leading_line_breaks;
use ruby_prism::*;
use std::iter::Peekable;

pub struct BuildContext<'a> {
    pub source: &'a [u8],
    pub built_end: usize,
    pub comments: &'a mut Peekable<Comments<'a>>,
    pub inner_comment: Vec<Comment<'a>>,
    pub leading_line_breaks: bool,
}

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
    fn __build__(&self, context: &mut BuildContext) -> Document {
        #[rustfmt::skip]
        fn build_node(node: &Node, context: &mut BuildContext) -> Document {
            match node {
                Node::AliasGlobalVariableNode           { .. } => <AliasGlobalVariableNode           as NodeVariant>::build(&node.as_alias_global_variable_node().unwrap()           , context),
                Node::AliasMethodNode                   { .. } => <AliasMethodNode                   as NodeVariant>::build(&node.as_alias_method_node().unwrap()                    , context),
                Node::AlternationPatternNode            { .. } => <AlternationPatternNode            as NodeVariant>::build(&node.as_alternation_pattern_node().unwrap()             , context),
                Node::AndNode                           { .. } => <AndNode                           as NodeVariant>::build(&node.as_and_node().unwrap()                             , context),
                Node::ArgumentsNode                     { .. } => <ArgumentsNode                     as NodeVariant>::build(&node.as_arguments_node().unwrap()                       , context),
                Node::ArrayNode                         { .. } => <ArrayNode                         as NodeVariant>::build(&node.as_array_node().unwrap()                           , context),
                Node::ArrayPatternNode                  { .. } => <ArrayPatternNode                  as NodeVariant>::build(&node.as_array_pattern_node().unwrap()                   , context),
                Node::AssocNode                         { .. } => <AssocNode                         as NodeVariant>::build(&node.as_assoc_node().unwrap()                           , context),
                Node::AssocSplatNode                    { .. } => <AssocSplatNode                    as NodeVariant>::build(&node.as_assoc_splat_node().unwrap()                     , context),
                Node::BackReferenceReadNode             { .. } => <BackReferenceReadNode             as NodeVariant>::build(&node.as_back_reference_read_node().unwrap()             , context),
                Node::BeginNode                         { .. } => <BeginNode                         as NodeVariant>::build(&node.as_begin_node().unwrap()                           , context),
                Node::BlockArgumentNode                 { .. } => <BlockArgumentNode                 as NodeVariant>::build(&node.as_block_argument_node().unwrap()                  , context),
                Node::BlockLocalVariableNode            { .. } => <BlockLocalVariableNode            as NodeVariant>::build(&node.as_block_local_variable_node().unwrap()            , context),
                Node::BlockNode                         { .. } => <BlockNode                         as NodeVariant>::build(&node.as_block_node().unwrap()                           , context),
                Node::BlockParameterNode                { .. } => <BlockParameterNode                as NodeVariant>::build(&node.as_block_parameter_node().unwrap()                 , context),
                Node::BlockParametersNode               { .. } => <BlockParametersNode               as NodeVariant>::build(&node.as_block_parameters_node().unwrap()                , context),
                Node::BreakNode                         { .. } => <BreakNode                         as NodeVariant>::build(&node.as_break_node().unwrap()                           , context),
                Node::CallAndWriteNode                  { .. } => <CallAndWriteNode                  as NodeVariant>::build(&node.as_call_and_write_node().unwrap()                  , context),
                Node::CallNode                          { .. } => <CallNode                          as NodeVariant>::build(&node.as_call_node().unwrap()                            , context),
                Node::CallOperatorWriteNode             { .. } => <CallOperatorWriteNode             as NodeVariant>::build(&node.as_call_operator_write_node().unwrap()             , context),
                Node::CallOrWriteNode                   { .. } => <CallOrWriteNode                   as NodeVariant>::build(&node.as_call_or_write_node().unwrap()                   , context),
                Node::CallTargetNode                    { .. } => <CallTargetNode                    as NodeVariant>::build(&node.as_call_target_node().unwrap()                     , context),
                Node::CapturePatternNode                { .. } => <CapturePatternNode                as NodeVariant>::build(&node.as_capture_pattern_node().unwrap()                 , context),
                Node::CaseMatchNode                     { .. } => <CaseMatchNode                     as NodeVariant>::build(&node.as_case_match_node().unwrap()                      , context),
                Node::CaseNode                          { .. } => <CaseNode                          as NodeVariant>::build(&node.as_case_node().unwrap()                            , context),
                Node::ClassNode                         { .. } => <ClassNode                         as NodeVariant>::build(&node.as_class_node().unwrap()                           , context),
                Node::ClassVariableAndWriteNode         { .. } => <ClassVariableAndWriteNode         as NodeVariant>::build(&node.as_class_variable_and_write_node().unwrap()        , context),
                Node::ClassVariableOperatorWriteNode    { .. } => <ClassVariableOperatorWriteNode    as NodeVariant>::build(&node.as_class_variable_operator_write_node().unwrap()   , context),
                Node::ClassVariableOrWriteNode          { .. } => <ClassVariableOrWriteNode          as NodeVariant>::build(&node.as_class_variable_or_write_node().unwrap()         , context),
                Node::ClassVariableReadNode             { .. } => <ClassVariableReadNode             as NodeVariant>::build(&node.as_class_variable_read_node().unwrap()             , context),
                Node::ClassVariableTargetNode           { .. } => <ClassVariableTargetNode           as NodeVariant>::build(&node.as_class_variable_target_node().unwrap()           , context),
                Node::ClassVariableWriteNode            { .. } => <ClassVariableWriteNode            as NodeVariant>::build(&node.as_class_variable_write_node().unwrap()            , context),
                Node::ConstantAndWriteNode              { .. } => <ConstantAndWriteNode              as NodeVariant>::build(&node.as_constant_and_write_node().unwrap()              , context),
                Node::ConstantOperatorWriteNode         { .. } => <ConstantOperatorWriteNode         as NodeVariant>::build(&node.as_constant_operator_write_node().unwrap()         , context),
                Node::ConstantOrWriteNode               { .. } => <ConstantOrWriteNode               as NodeVariant>::build(&node.as_constant_or_write_node().unwrap()               , context),
                Node::ConstantPathAndWriteNode          { .. } => <ConstantPathAndWriteNode          as NodeVariant>::build(&node.as_constant_path_and_write_node().unwrap()         , context),
                Node::ConstantPathNode                  { .. } => <ConstantPathNode                  as NodeVariant>::build(&node.as_constant_path_node().unwrap()                   , context),
                Node::ConstantPathOperatorWriteNode     { .. } => <ConstantPathOperatorWriteNode     as NodeVariant>::build(&node.as_constant_path_operator_write_node().unwrap()    , context),
                Node::ConstantPathOrWriteNode           { .. } => <ConstantPathOrWriteNode           as NodeVariant>::build(&node.as_constant_path_or_write_node().unwrap()          , context),
                Node::ConstantPathTargetNode            { .. } => <ConstantPathTargetNode            as NodeVariant>::build(&node.as_constant_path_target_node().unwrap()            , context),
                Node::ConstantPathWriteNode             { .. } => <ConstantPathWriteNode             as NodeVariant>::build(&node.as_constant_path_write_node().unwrap()             , context),
                Node::ConstantReadNode                  { .. } => <ConstantReadNode                  as NodeVariant>::build(&node.as_constant_read_node().unwrap()                   , context),
                Node::ConstantTargetNode                { .. } => <ConstantTargetNode                as NodeVariant>::build(&node.as_constant_target_node().unwrap()                 , context),
                Node::ConstantWriteNode                 { .. } => <ConstantWriteNode                 as NodeVariant>::build(&node.as_constant_write_node().unwrap()                  , context),
                Node::DefNode                           { .. } => <DefNode                           as NodeVariant>::build(&node.as_def_node().unwrap()                             , context),
                Node::DefinedNode                       { .. } => <DefinedNode                       as NodeVariant>::build(&node.as_defined_node().unwrap()                         , context),
                Node::ElseNode                          { .. } => <ElseNode                          as NodeVariant>::build(&node.as_else_node().unwrap()                            , context),
                Node::EmbeddedStatementsNode            { .. } => <EmbeddedStatementsNode            as NodeVariant>::build(&node.as_embedded_statements_node().unwrap()             , context),
                Node::EmbeddedVariableNode              { .. } => <EmbeddedVariableNode              as NodeVariant>::build(&node.as_embedded_variable_node().unwrap()               , context),
                Node::EnsureNode                        { .. } => <EnsureNode                        as NodeVariant>::build(&node.as_ensure_node().unwrap()                          , context),
                Node::FalseNode                         { .. } => <FalseNode                         as NodeVariant>::build(&node.as_false_node().unwrap()                           , context),
                Node::FindPatternNode                   { .. } => <FindPatternNode                   as NodeVariant>::build(&node.as_find_pattern_node().unwrap()                    , context),
                Node::FlipFlopNode                      { .. } => <FlipFlopNode                      as NodeVariant>::build(&node.as_flip_flop_node().unwrap()                       , context),
                Node::FloatNode                         { .. } => <FloatNode                         as NodeVariant>::build(&node.as_float_node().unwrap()                           , context),
                Node::ForNode                           { .. } => <ForNode                           as NodeVariant>::build(&node.as_for_node().unwrap()                             , context),
                Node::ForwardingArgumentsNode           { .. } => <ForwardingArgumentsNode           as NodeVariant>::build(&node.as_forwarding_arguments_node().unwrap()            , context),
                Node::ForwardingParameterNode           { .. } => <ForwardingParameterNode           as NodeVariant>::build(&node.as_forwarding_parameter_node().unwrap()            , context),
                Node::ForwardingSuperNode               { .. } => <ForwardingSuperNode               as NodeVariant>::build(&node.as_forwarding_super_node().unwrap()                , context),
                Node::GlobalVariableAndWriteNode        { .. } => <GlobalVariableAndWriteNode        as NodeVariant>::build(&node.as_global_variable_and_write_node().unwrap()       , context),
                Node::GlobalVariableOperatorWriteNode   { .. } => <GlobalVariableOperatorWriteNode   as NodeVariant>::build(&node.as_global_variable_operator_write_node().unwrap()  , context),
                Node::GlobalVariableOrWriteNode         { .. } => <GlobalVariableOrWriteNode         as NodeVariant>::build(&node.as_global_variable_or_write_node().unwrap()        , context),
                Node::GlobalVariableReadNode            { .. } => <GlobalVariableReadNode            as NodeVariant>::build(&node.as_global_variable_read_node().unwrap()            , context),
                Node::GlobalVariableTargetNode          { .. } => <GlobalVariableTargetNode          as NodeVariant>::build(&node.as_global_variable_target_node().unwrap()          , context),
                Node::GlobalVariableWriteNode           { .. } => <GlobalVariableWriteNode           as NodeVariant>::build(&node.as_global_variable_write_node().unwrap()           , context),
                Node::HashNode                          { .. } => <HashNode                          as NodeVariant>::build(&node.as_hash_node().unwrap()                            , context),
                Node::HashPatternNode                   { .. } => <HashPatternNode                   as NodeVariant>::build(&node.as_hash_pattern_node().unwrap()                    , context),
                Node::IfNode                            { .. } => <IfNode                            as NodeVariant>::build(&node.as_if_node().unwrap()                              , context),
                Node::ImaginaryNode                     { .. } => <ImaginaryNode                     as NodeVariant>::build(&node.as_imaginary_node().unwrap()                       , context),
                Node::ImplicitNode                      { .. } => <ImplicitNode                      as NodeVariant>::build(&node.as_implicit_node().unwrap()                        , context),
                Node::ImplicitRestNode                  { .. } => <ImplicitRestNode                  as NodeVariant>::build(&node.as_implicit_rest_node().unwrap()                   , context),
                Node::InNode                            { .. } => <InNode                            as NodeVariant>::build(&node.as_in_node().unwrap()                              , context),
                Node::IndexAndWriteNode                 { .. } => <IndexAndWriteNode                 as NodeVariant>::build(&node.as_index_and_write_node().unwrap()                 , context),
                Node::IndexOperatorWriteNode            { .. } => <IndexOperatorWriteNode            as NodeVariant>::build(&node.as_index_operator_write_node().unwrap()            , context),
                Node::IndexOrWriteNode                  { .. } => <IndexOrWriteNode                  as NodeVariant>::build(&node.as_index_or_write_node().unwrap()                  , context),
                Node::IndexTargetNode                   { .. } => <IndexTargetNode                   as NodeVariant>::build(&node.as_index_target_node().unwrap()                    , context),
                Node::InstanceVariableAndWriteNode      { .. } => <InstanceVariableAndWriteNode      as NodeVariant>::build(&node.as_instance_variable_and_write_node().unwrap()     , context),
                Node::InstanceVariableOperatorWriteNode { .. } => <InstanceVariableOperatorWriteNode as NodeVariant>::build(&node.as_instance_variable_operator_write_node().unwrap(), context),
                Node::InstanceVariableOrWriteNode       { .. } => <InstanceVariableOrWriteNode       as NodeVariant>::build(&node.as_instance_variable_or_write_node().unwrap()      , context),
                Node::InstanceVariableReadNode          { .. } => <InstanceVariableReadNode          as NodeVariant>::build(&node.as_instance_variable_read_node().unwrap()          , context),
                Node::InstanceVariableTargetNode        { .. } => <InstanceVariableTargetNode        as NodeVariant>::build(&node.as_instance_variable_target_node().unwrap()        , context),
                Node::InstanceVariableWriteNode         { .. } => <InstanceVariableWriteNode         as NodeVariant>::build(&node.as_instance_variable_write_node().unwrap()         , context),
                Node::IntegerNode                       { .. } => <IntegerNode                       as NodeVariant>::build(&node.as_integer_node().unwrap()                         , context),
                Node::InterpolatedMatchLastLineNode     { .. } => <InterpolatedMatchLastLineNode     as NodeVariant>::build(&node.as_interpolated_match_last_line_node().unwrap()    , context),
                Node::InterpolatedRegularExpressionNode { .. } => <InterpolatedRegularExpressionNode as NodeVariant>::build(&node.as_interpolated_regular_expression_node().unwrap() , context),
                Node::InterpolatedStringNode            { .. } => <InterpolatedStringNode            as NodeVariant>::build(&node.as_interpolated_string_node().unwrap()             , context),
                Node::InterpolatedSymbolNode            { .. } => <InterpolatedSymbolNode            as NodeVariant>::build(&node.as_interpolated_symbol_node().unwrap()             , context),
                Node::InterpolatedXStringNode           { .. } => <InterpolatedXStringNode           as NodeVariant>::build(&node.as_interpolated_x_string_node().unwrap()           , context),
                Node::ItLocalVariableReadNode           { .. } => <ItLocalVariableReadNode           as NodeVariant>::build(&node.as_it_local_variable_read_node().unwrap()          , context),
                Node::ItParametersNode                  { .. } => <ItParametersNode                  as NodeVariant>::build(&node.as_it_parameters_node().unwrap()                   , context),
                Node::KeywordHashNode                   { .. } => <KeywordHashNode                   as NodeVariant>::build(&node.as_keyword_hash_node().unwrap()                    , context),
                Node::KeywordRestParameterNode          { .. } => <KeywordRestParameterNode          as NodeVariant>::build(&node.as_keyword_rest_parameter_node().unwrap()          , context),
                Node::LambdaNode                        { .. } => <LambdaNode                        as NodeVariant>::build(&node.as_lambda_node().unwrap()                          , context),
                Node::LocalVariableAndWriteNode         { .. } => <LocalVariableAndWriteNode         as NodeVariant>::build(&node.as_local_variable_and_write_node().unwrap()        , context),
                Node::LocalVariableOperatorWriteNode    { .. } => <LocalVariableOperatorWriteNode    as NodeVariant>::build(&node.as_local_variable_operator_write_node().unwrap()   , context),
                Node::LocalVariableOrWriteNode          { .. } => <LocalVariableOrWriteNode          as NodeVariant>::build(&node.as_local_variable_or_write_node().unwrap()         , context),
                Node::LocalVariableReadNode             { .. } => <LocalVariableReadNode             as NodeVariant>::build(&node.as_local_variable_read_node().unwrap()             , context),
                Node::LocalVariableTargetNode           { .. } => <LocalVariableTargetNode           as NodeVariant>::build(&node.as_local_variable_target_node().unwrap()           , context),
                Node::LocalVariableWriteNode            { .. } => <LocalVariableWriteNode            as NodeVariant>::build(&node.as_local_variable_write_node().unwrap()            , context),
                Node::MatchLastLineNode                 { .. } => <MatchLastLineNode                 as NodeVariant>::build(&node.as_match_last_line_node().unwrap()                 , context),
                Node::MatchPredicateNode                { .. } => <MatchPredicateNode                as NodeVariant>::build(&node.as_match_predicate_node().unwrap()                 , context),
                Node::MatchRequiredNode                 { .. } => <MatchRequiredNode                 as NodeVariant>::build(&node.as_match_required_node().unwrap()                  , context),
                Node::MatchWriteNode                    { .. } => <MatchWriteNode                    as NodeVariant>::build(&node.as_match_write_node().unwrap()                     , context),
                Node::MissingNode                       { .. } => <MissingNode                       as NodeVariant>::build(&node.as_missing_node().unwrap()                         , context),
                Node::ModuleNode                        { .. } => <ModuleNode                        as NodeVariant>::build(&node.as_module_node().unwrap()                          , context),
                Node::MultiTargetNode                   { .. } => <MultiTargetNode                   as NodeVariant>::build(&node.as_multi_target_node().unwrap()                    , context),
                Node::MultiWriteNode                    { .. } => <MultiWriteNode                    as NodeVariant>::build(&node.as_multi_write_node().unwrap()                     , context),
                Node::NextNode                          { .. } => <NextNode                          as NodeVariant>::build(&node.as_next_node().unwrap()                            , context),
                Node::NilNode                           { .. } => <NilNode                           as NodeVariant>::build(&node.as_nil_node().unwrap()                             , context),
                Node::NoKeywordsParameterNode           { .. } => <NoKeywordsParameterNode           as NodeVariant>::build(&node.as_no_keywords_parameter_node().unwrap()           , context),
                Node::NumberedParametersNode            { .. } => <NumberedParametersNode            as NodeVariant>::build(&node.as_numbered_parameters_node().unwrap()             , context),
                Node::NumberedReferenceReadNode         { .. } => <NumberedReferenceReadNode         as NodeVariant>::build(&node.as_numbered_reference_read_node().unwrap()         , context),
                Node::OptionalKeywordParameterNode      { .. } => <OptionalKeywordParameterNode      as NodeVariant>::build(&node.as_optional_keyword_parameter_node().unwrap()      , context),
                Node::OptionalParameterNode             { .. } => <OptionalParameterNode             as NodeVariant>::build(&node.as_optional_parameter_node().unwrap()              , context),
                Node::OrNode                            { .. } => <OrNode                            as NodeVariant>::build(&node.as_or_node().unwrap()                              , context),
                Node::ParametersNode                    { .. } => <ParametersNode                    as NodeVariant>::build(&node.as_parameters_node().unwrap()                      , context),
                Node::ParenthesesNode                   { .. } => <ParenthesesNode                   as NodeVariant>::build(&node.as_parentheses_node().unwrap()                     , context),
                Node::PinnedExpressionNode              { .. } => <PinnedExpressionNode              as NodeVariant>::build(&node.as_pinned_expression_node().unwrap()               , context),
                Node::PinnedVariableNode                { .. } => <PinnedVariableNode                as NodeVariant>::build(&node.as_pinned_variable_node().unwrap()                 , context),
                Node::PostExecutionNode                 { .. } => <PostExecutionNode                 as NodeVariant>::build(&node.as_post_execution_node().unwrap()                  , context),
                Node::PreExecutionNode                  { .. } => <PreExecutionNode                  as NodeVariant>::build(&node.as_pre_execution_node().unwrap()                   , context),
                Node::ProgramNode                       { .. } => <ProgramNode                       as NodeVariant>::build(&node.as_program_node().unwrap()                         , context),
                Node::RangeNode                         { .. } => <RangeNode                         as NodeVariant>::build(&node.as_range_node().unwrap()                           , context),
                Node::RationalNode                      { .. } => <RationalNode                      as NodeVariant>::build(&node.as_rational_node().unwrap()                        , context),
                Node::RedoNode                          { .. } => <RedoNode                          as NodeVariant>::build(&node.as_redo_node().unwrap()                            , context),
                Node::RegularExpressionNode             { .. } => <RegularExpressionNode             as NodeVariant>::build(&node.as_regular_expression_node().unwrap()              , context),
                Node::RequiredKeywordParameterNode      { .. } => <RequiredKeywordParameterNode      as NodeVariant>::build(&node.as_required_keyword_parameter_node().unwrap()      , context),
                Node::RequiredParameterNode             { .. } => <RequiredParameterNode             as NodeVariant>::build(&node.as_required_parameter_node().unwrap()              , context),
                Node::RescueModifierNode                { .. } => <RescueModifierNode                as NodeVariant>::build(&node.as_rescue_modifier_node().unwrap()                 , context),
                Node::RescueNode                        { .. } => <RescueNode                        as NodeVariant>::build(&node.as_rescue_node().unwrap()                          , context),
                Node::RestParameterNode                 { .. } => <RestParameterNode                 as NodeVariant>::build(&node.as_rest_parameter_node().unwrap()                  , context),
                Node::RetryNode                         { .. } => <RetryNode                         as NodeVariant>::build(&node.as_retry_node().unwrap()                           , context),
                Node::ReturnNode                        { .. } => <ReturnNode                        as NodeVariant>::build(&node.as_return_node().unwrap()                          , context),
                Node::SelfNode                          { .. } => <SelfNode                          as NodeVariant>::build(&node.as_self_node().unwrap()                            , context),
                Node::ShareableConstantNode             { .. } => <ShareableConstantNode             as NodeVariant>::build(&node.as_shareable_constant_node().unwrap()              , context),
                Node::SingletonClassNode                { .. } => <SingletonClassNode                as NodeVariant>::build(&node.as_singleton_class_node().unwrap()                 , context),
                Node::SourceEncodingNode                { .. } => <SourceEncodingNode                as NodeVariant>::build(&node.as_source_encoding_node().unwrap()                 , context),
                Node::SourceFileNode                    { .. } => <SourceFileNode                    as NodeVariant>::build(&node.as_source_file_node().unwrap()                     , context),
                Node::SourceLineNode                    { .. } => <SourceLineNode                    as NodeVariant>::build(&node.as_source_line_node().unwrap()                     , context),
                Node::SplatNode                         { .. } => <SplatNode                         as NodeVariant>::build(&node.as_splat_node().unwrap()                           , context),
                Node::StatementsNode                    { .. } => <StatementsNode                    as NodeVariant>::build(&node.as_statements_node().unwrap()                      , context),
                Node::StringNode                        { .. } => <StringNode                        as NodeVariant>::build(&node.as_string_node().unwrap()                          , context),
                Node::SuperNode                         { .. } => <SuperNode                         as NodeVariant>::build(&node.as_super_node().unwrap()                           , context),
                Node::SymbolNode                        { .. } => <SymbolNode                        as NodeVariant>::build(&node.as_symbol_node().unwrap()                          , context),
                Node::TrueNode                          { .. } => <TrueNode                          as NodeVariant>::build(&node.as_true_node().unwrap()                            , context),
                Node::UndefNode                         { .. } => <UndefNode                         as NodeVariant>::build(&node.as_undef_node().unwrap()                           , context),
                Node::UnlessNode                        { .. } => <UnlessNode                        as NodeVariant>::build(&node.as_unless_node().unwrap()                          , context),
                Node::UntilNode                         { .. } => <UntilNode                         as NodeVariant>::build(&node.as_until_node().unwrap()                           , context),
                Node::WhenNode                          { .. } => <WhenNode                          as NodeVariant>::build(&node.as_when_node().unwrap()                            , context),
                Node::WhileNode                         { .. } => <WhileNode                         as NodeVariant>::build(&node.as_while_node().unwrap()                           , context),
                Node::XStringNode                       { .. } => <XStringNode                       as NodeVariant>::build(&node.as_x_string_node().unwrap()                        , context),
                Node::YieldNode                         { .. } => <YieldNode                         as NodeVariant>::build(&node.as_yield_node().unwrap()                           , context),
            }
        }
        let mut vec = Vec::new();
        if context.leading_line_breaks {
            if let Some(line_breaks) =
                build_leading_line_breaks(context, self.location().start_offset(), 1)
            {
                vec.push(line_breaks);
            }
        }
        if let Some(leading_comments) = build_leading_comments(self, context) {
            vec.push(leading_comments);
        }
        let prev_is_statement = context.leading_line_breaks;
        context.leading_line_breaks = match self {
            Node::StatementsNode { .. } => true,
            Node::ProgramNode { .. } => true,
            _ => false,
        };
        vec.push(build_node(self, context));
        if let Some(trailing_comments) = build_trailing_comments(self, context) {
            vec.push(trailing_comments);
        }
        context.leading_line_breaks = prev_is_statement;
        context.built_end = context.built_end.max(self.location().end_offset());
        Document::Array(vec)
    }
}

impl<T: Build> Build for Option<T> {
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

impl<'sh> ListBuild for Option<NodeList<'sh>> {
    fn __build__(&self, context: &mut BuildContext, separator: &Document) -> Document {
        match self {
            Some(node) => node.build(context, separator),
            None => Document::None,
        }
    }
}
