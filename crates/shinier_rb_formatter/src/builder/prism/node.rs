use crate::Document;
use crate::builder::BuildContext;
use crate::builder::NodeVariant;
use ruby_prism::*;

/// Trait for building documents.
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
        let mut documents = Vec::new();
        if let Some(before) = before {
            documents.push(before);
        }
        documents.push(self.build(context));
        if let Some(after) = after {
            documents.push(after);
        }
        Document::Array(documents)
    }
}

/// Trait for building lists of documents.
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
        let mut documents = Vec::new();
        if let Some(before) = before {
            documents.push(before);
        }
        documents.push(self.build(context, separator));
        if let Some(after) = after {
            documents.push(after);
        }
        Document::Array(documents)
    }
}

impl<'sh> Build for Node<'sh> {
    #[rustfmt::skip]
    fn __build__(&self, context: &mut BuildContext) -> Document {
        match self {
            Node::AliasGlobalVariableNode           { .. } => <AliasGlobalVariableNode           as NodeVariant>::build(&self.as_alias_global_variable_node().unwrap()           , context),
            Node::AliasMethodNode                   { .. } => <AliasMethodNode                   as NodeVariant>::build(&self.as_alias_method_node().unwrap()                    , context),
            Node::AlternationPatternNode            { .. } => <AlternationPatternNode            as NodeVariant>::build(&self.as_alternation_pattern_node().unwrap()             , context),
            Node::AndNode                           { .. } => <AndNode                           as NodeVariant>::build(&self.as_and_node().unwrap()                             , context),
            Node::ArgumentsNode                     { .. } => <ArgumentsNode                     as NodeVariant>::build(&self.as_arguments_node().unwrap()                       , context),
            Node::ArrayNode                         { .. } => <ArrayNode                         as NodeVariant>::build(&self.as_array_node().unwrap()                           , context),
            Node::ArrayPatternNode                  { .. } => <ArrayPatternNode                  as NodeVariant>::build(&self.as_array_pattern_node().unwrap()                   , context),
            Node::AssocNode                         { .. } => <AssocNode                         as NodeVariant>::build(&self.as_assoc_node().unwrap()                           , context),
            Node::AssocSplatNode                    { .. } => <AssocSplatNode                    as NodeVariant>::build(&self.as_assoc_splat_node().unwrap()                     , context),
            Node::BackReferenceReadNode             { .. } => <BackReferenceReadNode             as NodeVariant>::build(&self.as_back_reference_read_node().unwrap()             , context),
            Node::BeginNode                         { .. } => <BeginNode                         as NodeVariant>::build(&self.as_begin_node().unwrap()                           , context),
            Node::BlockArgumentNode                 { .. } => <BlockArgumentNode                 as NodeVariant>::build(&self.as_block_argument_node().unwrap()                  , context),
            Node::BlockLocalVariableNode            { .. } => <BlockLocalVariableNode            as NodeVariant>::build(&self.as_block_local_variable_node().unwrap()            , context),
            Node::BlockNode                         { .. } => <BlockNode                         as NodeVariant>::build(&self.as_block_node().unwrap()                           , context),
            Node::BlockParameterNode                { .. } => <BlockParameterNode                as NodeVariant>::build(&self.as_block_parameter_node().unwrap()                 , context),
            Node::BlockParametersNode               { .. } => <BlockParametersNode               as NodeVariant>::build(&self.as_block_parameters_node().unwrap()                , context),
            Node::BreakNode                         { .. } => <BreakNode                         as NodeVariant>::build(&self.as_break_node().unwrap()                           , context),
            Node::CallAndWriteNode                  { .. } => <CallAndWriteNode                  as NodeVariant>::build(&self.as_call_and_write_node().unwrap()                  , context),
            Node::CallNode                          { .. } => <CallNode                          as NodeVariant>::build(&self.as_call_node().unwrap()                            , context),
            Node::CallOperatorWriteNode             { .. } => <CallOperatorWriteNode             as NodeVariant>::build(&self.as_call_operator_write_node().unwrap()             , context),
            Node::CallOrWriteNode                   { .. } => <CallOrWriteNode                   as NodeVariant>::build(&self.as_call_or_write_node().unwrap()                   , context),
            Node::CallTargetNode                    { .. } => <CallTargetNode                    as NodeVariant>::build(&self.as_call_target_node().unwrap()                     , context),
            Node::CapturePatternNode                { .. } => <CapturePatternNode                as NodeVariant>::build(&self.as_capture_pattern_node().unwrap()                 , context),
            Node::CaseMatchNode                     { .. } => <CaseMatchNode                     as NodeVariant>::build(&self.as_case_match_node().unwrap()                      , context),
            Node::CaseNode                          { .. } => <CaseNode                          as NodeVariant>::build(&self.as_case_node().unwrap()                            , context),
            Node::ClassNode                         { .. } => <ClassNode                         as NodeVariant>::build(&self.as_class_node().unwrap()                           , context),
            Node::ClassVariableAndWriteNode         { .. } => <ClassVariableAndWriteNode         as NodeVariant>::build(&self.as_class_variable_and_write_node().unwrap()        , context),
            Node::ClassVariableOperatorWriteNode    { .. } => <ClassVariableOperatorWriteNode    as NodeVariant>::build(&self.as_class_variable_operator_write_node().unwrap()   , context),
            Node::ClassVariableOrWriteNode          { .. } => <ClassVariableOrWriteNode          as NodeVariant>::build(&self.as_class_variable_or_write_node().unwrap()         , context),
            Node::ClassVariableReadNode             { .. } => <ClassVariableReadNode             as NodeVariant>::build(&self.as_class_variable_read_node().unwrap()             , context),
            Node::ClassVariableTargetNode           { .. } => <ClassVariableTargetNode           as NodeVariant>::build(&self.as_class_variable_target_node().unwrap()           , context),
            Node::ClassVariableWriteNode            { .. } => <ClassVariableWriteNode            as NodeVariant>::build(&self.as_class_variable_write_node().unwrap()            , context),
            Node::ConstantAndWriteNode              { .. } => <ConstantAndWriteNode              as NodeVariant>::build(&self.as_constant_and_write_node().unwrap()              , context),
            Node::ConstantOperatorWriteNode         { .. } => <ConstantOperatorWriteNode         as NodeVariant>::build(&self.as_constant_operator_write_node().unwrap()         , context),
            Node::ConstantOrWriteNode               { .. } => <ConstantOrWriteNode               as NodeVariant>::build(&self.as_constant_or_write_node().unwrap()               , context),
            Node::ConstantPathAndWriteNode          { .. } => <ConstantPathAndWriteNode          as NodeVariant>::build(&self.as_constant_path_and_write_node().unwrap()         , context),
            Node::ConstantPathNode                  { .. } => <ConstantPathNode                  as NodeVariant>::build(&self.as_constant_path_node().unwrap()                   , context),
            Node::ConstantPathOperatorWriteNode     { .. } => <ConstantPathOperatorWriteNode     as NodeVariant>::build(&self.as_constant_path_operator_write_node().unwrap()    , context),
            Node::ConstantPathOrWriteNode           { .. } => <ConstantPathOrWriteNode           as NodeVariant>::build(&self.as_constant_path_or_write_node().unwrap()          , context),
            Node::ConstantPathTargetNode            { .. } => <ConstantPathTargetNode            as NodeVariant>::build(&self.as_constant_path_target_node().unwrap()            , context),
            Node::ConstantPathWriteNode             { .. } => <ConstantPathWriteNode             as NodeVariant>::build(&self.as_constant_path_write_node().unwrap()             , context),
            Node::ConstantReadNode                  { .. } => <ConstantReadNode                  as NodeVariant>::build(&self.as_constant_read_node().unwrap()                   , context),
            Node::ConstantTargetNode                { .. } => <ConstantTargetNode                as NodeVariant>::build(&self.as_constant_target_node().unwrap()                 , context),
            Node::ConstantWriteNode                 { .. } => <ConstantWriteNode                 as NodeVariant>::build(&self.as_constant_write_node().unwrap()                  , context),
            Node::DefNode                           { .. } => <DefNode                           as NodeVariant>::build(&self.as_def_node().unwrap()                             , context),
            Node::DefinedNode                       { .. } => <DefinedNode                       as NodeVariant>::build(&self.as_defined_node().unwrap()                         , context),
            Node::ElseNode                          { .. } => <ElseNode                          as NodeVariant>::build(&self.as_else_node().unwrap()                            , context),
            Node::EmbeddedStatementsNode            { .. } => <EmbeddedStatementsNode            as NodeVariant>::build(&self.as_embedded_statements_node().unwrap()             , context),
            Node::EmbeddedVariableNode              { .. } => <EmbeddedVariableNode              as NodeVariant>::build(&self.as_embedded_variable_node().unwrap()               , context),
            Node::EnsureNode                        { .. } => <EnsureNode                        as NodeVariant>::build(&self.as_ensure_node().unwrap()                          , context),
            Node::FalseNode                         { .. } => <FalseNode                         as NodeVariant>::build(&self.as_false_node().unwrap()                           , context),
            Node::FindPatternNode                   { .. } => <FindPatternNode                   as NodeVariant>::build(&self.as_find_pattern_node().unwrap()                    , context),
            Node::FlipFlopNode                      { .. } => <FlipFlopNode                      as NodeVariant>::build(&self.as_flip_flop_node().unwrap()                       , context),
            Node::FloatNode                         { .. } => <FloatNode                         as NodeVariant>::build(&self.as_float_node().unwrap()                           , context),
            Node::ForNode                           { .. } => <ForNode                           as NodeVariant>::build(&self.as_for_node().unwrap()                             , context),
            Node::ForwardingArgumentsNode           { .. } => <ForwardingArgumentsNode           as NodeVariant>::build(&self.as_forwarding_arguments_node().unwrap()            , context),
            Node::ForwardingParameterNode           { .. } => <ForwardingParameterNode           as NodeVariant>::build(&self.as_forwarding_parameter_node().unwrap()            , context),
            Node::ForwardingSuperNode               { .. } => <ForwardingSuperNode               as NodeVariant>::build(&self.as_forwarding_super_node().unwrap()                , context),
            Node::GlobalVariableAndWriteNode        { .. } => <GlobalVariableAndWriteNode        as NodeVariant>::build(&self.as_global_variable_and_write_node().unwrap()       , context),
            Node::GlobalVariableOperatorWriteNode   { .. } => <GlobalVariableOperatorWriteNode   as NodeVariant>::build(&self.as_global_variable_operator_write_node().unwrap()  , context),
            Node::GlobalVariableOrWriteNode         { .. } => <GlobalVariableOrWriteNode         as NodeVariant>::build(&self.as_global_variable_or_write_node().unwrap()        , context),
            Node::GlobalVariableReadNode            { .. } => <GlobalVariableReadNode            as NodeVariant>::build(&self.as_global_variable_read_node().unwrap()            , context),
            Node::GlobalVariableTargetNode          { .. } => <GlobalVariableTargetNode          as NodeVariant>::build(&self.as_global_variable_target_node().unwrap()          , context),
            Node::GlobalVariableWriteNode           { .. } => <GlobalVariableWriteNode           as NodeVariant>::build(&self.as_global_variable_write_node().unwrap()           , context),
            Node::HashNode                          { .. } => <HashNode                          as NodeVariant>::build(&self.as_hash_node().unwrap()                            , context),
            Node::HashPatternNode                   { .. } => <HashPatternNode                   as NodeVariant>::build(&self.as_hash_pattern_node().unwrap()                    , context),
            Node::IfNode                            { .. } => <IfNode                            as NodeVariant>::build(&self.as_if_node().unwrap()                              , context),
            Node::ImaginaryNode                     { .. } => <ImaginaryNode                     as NodeVariant>::build(&self.as_imaginary_node().unwrap()                       , context),
            Node::ImplicitNode                      { .. } => <ImplicitNode                      as NodeVariant>::build(&self.as_implicit_node().unwrap()                        , context),
            Node::ImplicitRestNode                  { .. } => <ImplicitRestNode                  as NodeVariant>::build(&self.as_implicit_rest_node().unwrap()                   , context),
            Node::InNode                            { .. } => <InNode                            as NodeVariant>::build(&self.as_in_node().unwrap()                              , context),
            Node::IndexAndWriteNode                 { .. } => <IndexAndWriteNode                 as NodeVariant>::build(&self.as_index_and_write_node().unwrap()                 , context),
            Node::IndexOperatorWriteNode            { .. } => <IndexOperatorWriteNode            as NodeVariant>::build(&self.as_index_operator_write_node().unwrap()            , context),
            Node::IndexOrWriteNode                  { .. } => <IndexOrWriteNode                  as NodeVariant>::build(&self.as_index_or_write_node().unwrap()                  , context),
            Node::IndexTargetNode                   { .. } => <IndexTargetNode                   as NodeVariant>::build(&self.as_index_target_node().unwrap()                    , context),
            Node::InstanceVariableAndWriteNode      { .. } => <InstanceVariableAndWriteNode      as NodeVariant>::build(&self.as_instance_variable_and_write_node().unwrap()     , context),
            Node::InstanceVariableOperatorWriteNode { .. } => <InstanceVariableOperatorWriteNode as NodeVariant>::build(&self.as_instance_variable_operator_write_node().unwrap(), context),
            Node::InstanceVariableOrWriteNode       { .. } => <InstanceVariableOrWriteNode       as NodeVariant>::build(&self.as_instance_variable_or_write_node().unwrap()      , context),
            Node::InstanceVariableReadNode          { .. } => <InstanceVariableReadNode          as NodeVariant>::build(&self.as_instance_variable_read_node().unwrap()          , context),
            Node::InstanceVariableTargetNode        { .. } => <InstanceVariableTargetNode        as NodeVariant>::build(&self.as_instance_variable_target_node().unwrap()        , context),
            Node::InstanceVariableWriteNode         { .. } => <InstanceVariableWriteNode         as NodeVariant>::build(&self.as_instance_variable_write_node().unwrap()         , context),
            Node::IntegerNode                       { .. } => <IntegerNode                       as NodeVariant>::build(&self.as_integer_node().unwrap()                         , context),
            Node::InterpolatedMatchLastLineNode     { .. } => <InterpolatedMatchLastLineNode     as NodeVariant>::build(&self.as_interpolated_match_last_line_node().unwrap()    , context),
            Node::InterpolatedRegularExpressionNode { .. } => <InterpolatedRegularExpressionNode as NodeVariant>::build(&self.as_interpolated_regular_expression_node().unwrap() , context),
            Node::InterpolatedStringNode            { .. } => <InterpolatedStringNode            as NodeVariant>::build(&self.as_interpolated_string_node().unwrap()             , context),
            Node::InterpolatedSymbolNode            { .. } => <InterpolatedSymbolNode            as NodeVariant>::build(&self.as_interpolated_symbol_node().unwrap()             , context),
            Node::InterpolatedXStringNode           { .. } => <InterpolatedXStringNode           as NodeVariant>::build(&self.as_interpolated_x_string_node().unwrap()           , context),
            Node::ItLocalVariableReadNode           { .. } => <ItLocalVariableReadNode           as NodeVariant>::build(&self.as_it_local_variable_read_node().unwrap()          , context),
            Node::ItParametersNode                  { .. } => <ItParametersNode                  as NodeVariant>::build(&self.as_it_parameters_node().unwrap()                   , context),
            Node::KeywordHashNode                   { .. } => <KeywordHashNode                   as NodeVariant>::build(&self.as_keyword_hash_node().unwrap()                    , context),
            Node::KeywordRestParameterNode          { .. } => <KeywordRestParameterNode          as NodeVariant>::build(&self.as_keyword_rest_parameter_node().unwrap()          , context),
            Node::LambdaNode                        { .. } => <LambdaNode                        as NodeVariant>::build(&self.as_lambda_node().unwrap()                          , context),
            Node::LocalVariableAndWriteNode         { .. } => <LocalVariableAndWriteNode         as NodeVariant>::build(&self.as_local_variable_and_write_node().unwrap()        , context),
            Node::LocalVariableOperatorWriteNode    { .. } => <LocalVariableOperatorWriteNode    as NodeVariant>::build(&self.as_local_variable_operator_write_node().unwrap()   , context),
            Node::LocalVariableOrWriteNode          { .. } => <LocalVariableOrWriteNode          as NodeVariant>::build(&self.as_local_variable_or_write_node().unwrap()         , context),
            Node::LocalVariableReadNode             { .. } => <LocalVariableReadNode             as NodeVariant>::build(&self.as_local_variable_read_node().unwrap()             , context),
            Node::LocalVariableTargetNode           { .. } => <LocalVariableTargetNode           as NodeVariant>::build(&self.as_local_variable_target_node().unwrap()           , context),
            Node::LocalVariableWriteNode            { .. } => <LocalVariableWriteNode            as NodeVariant>::build(&self.as_local_variable_write_node().unwrap()            , context),
            Node::MatchLastLineNode                 { .. } => <MatchLastLineNode                 as NodeVariant>::build(&self.as_match_last_line_node().unwrap()                 , context),
            Node::MatchPredicateNode                { .. } => <MatchPredicateNode                as NodeVariant>::build(&self.as_match_predicate_node().unwrap()                 , context),
            Node::MatchRequiredNode                 { .. } => <MatchRequiredNode                 as NodeVariant>::build(&self.as_match_required_node().unwrap()                  , context),
            Node::MatchWriteNode                    { .. } => <MatchWriteNode                    as NodeVariant>::build(&self.as_match_write_node().unwrap()                     , context),
            Node::MissingNode                       { .. } => <MissingNode                       as NodeVariant>::build(&self.as_missing_node().unwrap()                         , context),
            Node::ModuleNode                        { .. } => <ModuleNode                        as NodeVariant>::build(&self.as_module_node().unwrap()                          , context),
            Node::MultiTargetNode                   { .. } => <MultiTargetNode                   as NodeVariant>::build(&self.as_multi_target_node().unwrap()                    , context),
            Node::MultiWriteNode                    { .. } => <MultiWriteNode                    as NodeVariant>::build(&self.as_multi_write_node().unwrap()                     , context),
            Node::NextNode                          { .. } => <NextNode                          as NodeVariant>::build(&self.as_next_node().unwrap()                            , context),
            Node::NilNode                           { .. } => <NilNode                           as NodeVariant>::build(&self.as_nil_node().unwrap()                             , context),
            Node::NoKeywordsParameterNode           { .. } => <NoKeywordsParameterNode           as NodeVariant>::build(&self.as_no_keywords_parameter_node().unwrap()           , context),
            Node::NumberedParametersNode            { .. } => <NumberedParametersNode            as NodeVariant>::build(&self.as_numbered_parameters_node().unwrap()             , context),
            Node::NumberedReferenceReadNode         { .. } => <NumberedReferenceReadNode         as NodeVariant>::build(&self.as_numbered_reference_read_node().unwrap()         , context),
            Node::OptionalKeywordParameterNode      { .. } => <OptionalKeywordParameterNode      as NodeVariant>::build(&self.as_optional_keyword_parameter_node().unwrap()      , context),
            Node::OptionalParameterNode             { .. } => <OptionalParameterNode             as NodeVariant>::build(&self.as_optional_parameter_node().unwrap()              , context),
            Node::OrNode                            { .. } => <OrNode                            as NodeVariant>::build(&self.as_or_node().unwrap()                              , context),
            Node::ParametersNode                    { .. } => <ParametersNode                    as NodeVariant>::build(&self.as_parameters_node().unwrap()                      , context),
            Node::ParenthesesNode                   { .. } => <ParenthesesNode                   as NodeVariant>::build(&self.as_parentheses_node().unwrap()                     , context),
            Node::PinnedExpressionNode              { .. } => <PinnedExpressionNode              as NodeVariant>::build(&self.as_pinned_expression_node().unwrap()               , context),
            Node::PinnedVariableNode                { .. } => <PinnedVariableNode                as NodeVariant>::build(&self.as_pinned_variable_node().unwrap()                 , context),
            Node::PostExecutionNode                 { .. } => <PostExecutionNode                 as NodeVariant>::build(&self.as_post_execution_node().unwrap()                  , context),
            Node::PreExecutionNode                  { .. } => <PreExecutionNode                  as NodeVariant>::build(&self.as_pre_execution_node().unwrap()                   , context),
            Node::ProgramNode                       { .. } => <ProgramNode                       as NodeVariant>::build(&self.as_program_node().unwrap()                         , context),
            Node::RangeNode                         { .. } => <RangeNode                         as NodeVariant>::build(&self.as_range_node().unwrap()                           , context),
            Node::RationalNode                      { .. } => <RationalNode                      as NodeVariant>::build(&self.as_rational_node().unwrap()                        , context),
            Node::RedoNode                          { .. } => <RedoNode                          as NodeVariant>::build(&self.as_redo_node().unwrap()                            , context),
            Node::RegularExpressionNode             { .. } => <RegularExpressionNode             as NodeVariant>::build(&self.as_regular_expression_node().unwrap()              , context),
            Node::RequiredKeywordParameterNode      { .. } => <RequiredKeywordParameterNode      as NodeVariant>::build(&self.as_required_keyword_parameter_node().unwrap()      , context),
            Node::RequiredParameterNode             { .. } => <RequiredParameterNode             as NodeVariant>::build(&self.as_required_parameter_node().unwrap()              , context),
            Node::RescueModifierNode                { .. } => <RescueModifierNode                as NodeVariant>::build(&self.as_rescue_modifier_node().unwrap()                 , context),
            Node::RescueNode                        { .. } => <RescueNode                        as NodeVariant>::build(&self.as_rescue_node().unwrap()                          , context),
            Node::RestParameterNode                 { .. } => <RestParameterNode                 as NodeVariant>::build(&self.as_rest_parameter_node().unwrap()                  , context),
            Node::RetryNode                         { .. } => <RetryNode                         as NodeVariant>::build(&self.as_retry_node().unwrap()                           , context),
            Node::ReturnNode                        { .. } => <ReturnNode                        as NodeVariant>::build(&self.as_return_node().unwrap()                          , context),
            Node::SelfNode                          { .. } => <SelfNode                          as NodeVariant>::build(&self.as_self_node().unwrap()                            , context),
            Node::ShareableConstantNode             { .. } => <ShareableConstantNode             as NodeVariant>::build(&self.as_shareable_constant_node().unwrap()              , context),
            Node::SingletonClassNode                { .. } => <SingletonClassNode                as NodeVariant>::build(&self.as_singleton_class_node().unwrap()                 , context),
            Node::SourceEncodingNode                { .. } => <SourceEncodingNode                as NodeVariant>::build(&self.as_source_encoding_node().unwrap()                 , context),
            Node::SourceFileNode                    { .. } => <SourceFileNode                    as NodeVariant>::build(&self.as_source_file_node().unwrap()                     , context),
            Node::SourceLineNode                    { .. } => <SourceLineNode                    as NodeVariant>::build(&self.as_source_line_node().unwrap()                     , context),
            Node::SplatNode                         { .. } => <SplatNode                         as NodeVariant>::build(&self.as_splat_node().unwrap()                           , context),
            Node::StatementsNode                    { .. } => <StatementsNode                    as NodeVariant>::build(&self.as_statements_node().unwrap()                      , context),
            Node::StringNode                        { .. } => <StringNode                        as NodeVariant>::build(&self.as_string_node().unwrap()                          , context),
            Node::SuperNode                         { .. } => <SuperNode                         as NodeVariant>::build(&self.as_super_node().unwrap()                           , context),
            Node::SymbolNode                        { .. } => <SymbolNode                        as NodeVariant>::build(&self.as_symbol_node().unwrap()                          , context),
            Node::TrueNode                          { .. } => <TrueNode                          as NodeVariant>::build(&self.as_true_node().unwrap()                            , context),
            Node::UndefNode                         { .. } => <UndefNode                         as NodeVariant>::build(&self.as_undef_node().unwrap()                           , context),
            Node::UnlessNode                        { .. } => <UnlessNode                        as NodeVariant>::build(&self.as_unless_node().unwrap()                          , context),
            Node::UntilNode                         { .. } => <UntilNode                         as NodeVariant>::build(&self.as_until_node().unwrap()                           , context),
            Node::WhenNode                          { .. } => <WhenNode                          as NodeVariant>::build(&self.as_when_node().unwrap()                            , context),
            Node::WhileNode                         { .. } => <WhileNode                         as NodeVariant>::build(&self.as_while_node().unwrap()                           , context),
            Node::XStringNode                       { .. } => <XStringNode                       as NodeVariant>::build(&self.as_x_string_node().unwrap()                        , context),
            Node::YieldNode                         { .. } => <YieldNode                         as NodeVariant>::build(&self.as_yield_node().unwrap()                           , context),
        }
    }
}

impl<T: Build> Build for Option<T> {
    fn __build__(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.build(context),
            None => Document::None,
        }
    }
    fn build(&self, context: &mut BuildContext) -> Document {
        match self {
            Some(node) => node.build(context),
            None => Document::None,
        }
    }
    fn build_with(
        &self,
        context: &mut BuildContext,
        before: Option<Document>,
        after: Option<Document>,
    ) -> Document {
        match self {
            Some(node) => node.build_with(context, before, after),
            None => Document::None,
        }
    }
}

impl<'sh> ListBuild for NodeList<'sh> {
    fn __build__(&self, context: &mut BuildContext, separator: &Document) -> Document {
        if self.iter().next().is_none() {
            return Document::None;
        }
        let mut vec = Vec::new();
        let mut was_none = false;
        for (i, node) in self.iter().enumerate() {
            if i > 0 && !was_none {
                vec.push(separator.clone());
            }
            match node.build(context) {
                Document::None => {
                    was_none = true;
                }
                other => {
                    was_none = false;
                    vec.push(other);
                }
            }
        }
        Document::Array(vec)
    }
}

impl<T: ListBuild> ListBuild for Option<T> {
    fn __build__(&self, context: &mut BuildContext, separator: &Document) -> Document {
        match self {
            Some(node) => node.build(context, separator),
            None => Document::None,
        }
    }
}
