use insta::assert_snapshot;
use insta::glob;
use ruby_prism::Node;
use shinier_rb_formatter::Printer;
use shinier_rb_formatter::prism::visit_all::VisitAll;

struct Visitor {
    pub depth: usize,
    pub debug_strings: Vec<String>,
}
impl Visitor {
    #[rustfmt::skip]
    fn push_debug_string(&mut self, node: &Node) {
        let mut debug_string = "__".repeat(self.depth);
        match node {
            Node::AliasGlobalVariableNode           { .. } => {debug_string.push_str("AliasGlobalVariableNode");          }
            Node::AliasMethodNode                   { .. } => {debug_string.push_str("AliasMethodNode");                  }
            Node::AlternationPatternNode            { .. } => {debug_string.push_str("AlternationPatternNode");           }
            Node::AndNode                           { .. } => {debug_string.push_str("AndNode");                          }
            Node::ArgumentsNode                     { .. } => {debug_string.push_str("ArgumentsNode");                    }
            Node::ArrayNode                         { .. } => {debug_string.push_str("ArrayNode");                        }
            Node::ArrayPatternNode                  { .. } => {debug_string.push_str("ArrayPatternNode");                 }
            Node::AssocNode                         { .. } => {debug_string.push_str("AssocNode");                        }
            Node::AssocSplatNode                    { .. } => {debug_string.push_str("AssocSplatNode");                   }
            Node::BackReferenceReadNode             { .. } => {debug_string.push_str("BackReferenceReadNode");            }
            Node::BeginNode                         { .. } => {debug_string.push_str("BeginNode");                        }
            Node::BlockArgumentNode                 { .. } => {debug_string.push_str("BlockArgumentNode");                }
            Node::BlockLocalVariableNode            { .. } => {debug_string.push_str("BlockLocalVariableNode");           }
            Node::BlockNode                         { .. } => {debug_string.push_str("BlockNode");                        }
            Node::BlockParameterNode                { .. } => {debug_string.push_str("BlockParameterNode");               }
            Node::BlockParametersNode               { .. } => {debug_string.push_str("BlockParametersNode");              }
            Node::BreakNode                         { .. } => {debug_string.push_str("BreakNode");                        }
            Node::CallAndWriteNode                  { .. } => {debug_string.push_str("CallAndWriteNode");                 }
            Node::CallNode                          { .. } => {debug_string.push_str("CallNode");                         }
            Node::CallOperatorWriteNode             { .. } => {debug_string.push_str("CallOperatorWriteNode");            }
            Node::CallOrWriteNode                   { .. } => {debug_string.push_str("CallOrWriteNode");                  }
            Node::CallTargetNode                    { .. } => {debug_string.push_str("CallTargetNode");                   }
            Node::CapturePatternNode                { .. } => {debug_string.push_str("CapturePatternNode");               }
            Node::CaseMatchNode                     { .. } => {debug_string.push_str("CaseMatchNode");                    }
            Node::CaseNode                          { .. } => {debug_string.push_str("CaseNode");                         }
            Node::ClassNode                         { .. } => {debug_string.push_str("ClassNode");                        }
            Node::ClassVariableAndWriteNode         { .. } => {debug_string.push_str("ClassVariableAndWriteNode");        }
            Node::ClassVariableOperatorWriteNode    { .. } => {debug_string.push_str("ClassVariableOperatorWriteNode");   }
            Node::ClassVariableOrWriteNode          { .. } => {debug_string.push_str("ClassVariableOrWriteNode");         }
            Node::ClassVariableReadNode             { .. } => {debug_string.push_str("ClassVariableReadNode");            }
            Node::ClassVariableTargetNode           { .. } => {debug_string.push_str("ClassVariableTargetNode");          }
            Node::ClassVariableWriteNode            { .. } => {debug_string.push_str("ClassVariableWriteNode");           }
            Node::ConstantAndWriteNode              { .. } => {debug_string.push_str("ConstantAndWriteNode");             }
            Node::ConstantOperatorWriteNode         { .. } => {debug_string.push_str("ConstantOperatorWriteNode");        }
            Node::ConstantOrWriteNode               { .. } => {debug_string.push_str("ConstantOrWriteNode");              }
            Node::ConstantPathAndWriteNode          { .. } => {debug_string.push_str("ConstantPathAndWriteNode");         }
            Node::ConstantPathNode                  { .. } => {debug_string.push_str("ConstantPathNode");                 }
            Node::ConstantPathOperatorWriteNode     { .. } => {debug_string.push_str("ConstantPathOperatorWriteNode");    }
            Node::ConstantPathOrWriteNode           { .. } => {debug_string.push_str("ConstantPathOrWriteNode");          }
            Node::ConstantPathTargetNode            { .. } => {debug_string.push_str("ConstantPathTargetNode");           }
            Node::ConstantPathWriteNode             { .. } => {debug_string.push_str("ConstantPathWriteNode");            }
            Node::ConstantReadNode                  { .. } => {debug_string.push_str("ConstantReadNode");                 }
            Node::ConstantTargetNode                { .. } => {debug_string.push_str("ConstantTargetNode");               }
            Node::ConstantWriteNode                 { .. } => {debug_string.push_str("ConstantWriteNode");                }
            Node::DefNode                           { .. } => {debug_string.push_str("DefNode");                          }
            Node::DefinedNode                       { .. } => {debug_string.push_str("DefinedNode");                      }
            Node::ElseNode                          { .. } => {debug_string.push_str("ElseNode");                         }
            Node::EmbeddedStatementsNode            { .. } => {debug_string.push_str("EmbeddedStatementsNode");           }
            Node::EmbeddedVariableNode              { .. } => {debug_string.push_str("EmbeddedVariableNode");             }
            Node::EnsureNode                        { .. } => {debug_string.push_str("EnsureNode");                       }
            Node::FalseNode                         { .. } => {debug_string.push_str("FalseNode");                        }
            Node::FindPatternNode                   { .. } => {debug_string.push_str("FindPatternNode");                  }
            Node::FlipFlopNode                      { .. } => {debug_string.push_str("FlipFlopNode");                     }
            Node::FloatNode                         { .. } => {debug_string.push_str("FloatNode");                        }
            Node::ForNode                           { .. } => {debug_string.push_str("ForNode");                          }
            Node::ForwardingArgumentsNode           { .. } => {debug_string.push_str("ForwardingArgumentsNode");          }
            Node::ForwardingParameterNode           { .. } => {debug_string.push_str("ForwardingParameterNode");          }
            Node::ForwardingSuperNode               { .. } => {debug_string.push_str("ForwardingSuperNode");              }
            Node::GlobalVariableAndWriteNode        { .. } => {debug_string.push_str("GlobalVariableAndWriteNode");       }
            Node::GlobalVariableOperatorWriteNode   { .. } => {debug_string.push_str("GlobalVariableOperatorWriteNode");  }
            Node::GlobalVariableOrWriteNode         { .. } => {debug_string.push_str("GlobalVariableOrWriteNode");        }
            Node::GlobalVariableReadNode            { .. } => {debug_string.push_str("GlobalVariableReadNode");           }
            Node::GlobalVariableTargetNode          { .. } => {debug_string.push_str("GlobalVariableTargetNode");         }
            Node::GlobalVariableWriteNode           { .. } => {debug_string.push_str("GlobalVariableWriteNode");          }
            Node::HashNode                          { .. } => {debug_string.push_str("HashNode");                         }
            Node::HashPatternNode                   { .. } => {debug_string.push_str("HashPatternNode");                  }
            Node::IfNode                            { .. } => {debug_string.push_str("IfNode");                           }
            Node::ImaginaryNode                     { .. } => {debug_string.push_str("ImaginaryNode");                    }
            Node::ImplicitNode                      { .. } => {debug_string.push_str("ImplicitNode");                     }
            Node::ImplicitRestNode                  { .. } => {debug_string.push_str("ImplicitRestNode");                 }
            Node::InNode                            { .. } => {debug_string.push_str("InNode");                           }
            Node::IndexAndWriteNode                 { .. } => {debug_string.push_str("IndexAndWriteNode");                }
            Node::IndexOperatorWriteNode            { .. } => {debug_string.push_str("IndexOperatorWriteNode");           }
            Node::IndexOrWriteNode                  { .. } => {debug_string.push_str("IndexOrWriteNode");                 }
            Node::IndexTargetNode                   { .. } => {debug_string.push_str("IndexTargetNode");                  }
            Node::InstanceVariableAndWriteNode      { .. } => {debug_string.push_str("InstanceVariableAndWriteNode");     }
            Node::InstanceVariableOperatorWriteNode { .. } => {debug_string.push_str("InstanceVariableOperatorWriteNode");}
            Node::InstanceVariableOrWriteNode       { .. } => {debug_string.push_str("InstanceVariableOrWriteNode");      }
            Node::InstanceVariableReadNode          { .. } => {debug_string.push_str("InstanceVariableReadNode");         }
            Node::InstanceVariableTargetNode        { .. } => {debug_string.push_str("InstanceVariableTargetNode");       }
            Node::InstanceVariableWriteNode         { .. } => {debug_string.push_str("InstanceVariableWriteNode");        }
            Node::IntegerNode                       { .. } => {debug_string.push_str("IntegerNode");                      }
            Node::InterpolatedMatchLastLineNode     { .. } => {debug_string.push_str("InterpolatedMatchLastLineNode");    }
            Node::InterpolatedRegularExpressionNode { .. } => {debug_string.push_str("InterpolatedRegularExpressionNode");}
            Node::InterpolatedStringNode            { .. } => {debug_string.push_str("InterpolatedStringNode");           }
            Node::InterpolatedSymbolNode            { .. } => {debug_string.push_str("InterpolatedSymbolNode");           }
            Node::InterpolatedXStringNode           { .. } => {debug_string.push_str("InterpolatedXStringNode");          }
            Node::ItLocalVariableReadNode           { .. } => {debug_string.push_str("ItLocalVariableReadNode");          }
            Node::ItParametersNode                  { .. } => {debug_string.push_str("ItParametersNode");                 }
            Node::KeywordHashNode                   { .. } => {debug_string.push_str("KeywordHashNode");                  }
            Node::KeywordRestParameterNode          { .. } => {debug_string.push_str("KeywordRestParameterNode");         }
            Node::LambdaNode                        { .. } => {debug_string.push_str("LambdaNode");                       }
            Node::LocalVariableAndWriteNode         { .. } => {debug_string.push_str("LocalVariableAndWriteNode");        }
            Node::LocalVariableOperatorWriteNode    { .. } => {debug_string.push_str("LocalVariableOperatorWriteNode");   }
            Node::LocalVariableOrWriteNode          { .. } => {debug_string.push_str("LocalVariableOrWriteNode");         }
            Node::LocalVariableReadNode             { .. } => {debug_string.push_str("LocalVariableReadNode");            }
            Node::LocalVariableTargetNode           { .. } => {debug_string.push_str("LocalVariableTargetNode");          }
            Node::LocalVariableWriteNode            { .. } => {debug_string.push_str("LocalVariableWriteNode");           }
            Node::MatchLastLineNode                 { .. } => {debug_string.push_str("MatchLastLineNode");                }
            Node::MatchPredicateNode                { .. } => {debug_string.push_str("MatchPredicateNode");               }
            Node::MatchRequiredNode                 { .. } => {debug_string.push_str("MatchRequiredNode");                }
            Node::MatchWriteNode                    { .. } => {debug_string.push_str("MatchWriteNode");                   }
            Node::MissingNode                       { .. } => {debug_string.push_str("MissingNode");                      }
            Node::ModuleNode                        { .. } => {debug_string.push_str("ModuleNode");                       }
            Node::MultiTargetNode                   { .. } => {debug_string.push_str("MultiTargetNode");                  }
            Node::MultiWriteNode                    { .. } => {debug_string.push_str("MultiWriteNode");                   }
            Node::NextNode                          { .. } => {debug_string.push_str("NextNode");                         }
            Node::NilNode                           { .. } => {debug_string.push_str("NilNode");                          }
            Node::NoKeywordsParameterNode           { .. } => {debug_string.push_str("NoKeywordsParameterNode");          }
            Node::NumberedParametersNode            { .. } => {debug_string.push_str("NumberedParametersNode");           }
            Node::NumberedReferenceReadNode         { .. } => {debug_string.push_str("NumberedReferenceReadNode");        }
            Node::OptionalKeywordParameterNode      { .. } => {debug_string.push_str("OptionalKeywordParameterNode");     }
            Node::OptionalParameterNode             { .. } => {debug_string.push_str("OptionalParameterNode");            }
            Node::OrNode                            { .. } => {debug_string.push_str("OrNode");                           }
            Node::ParametersNode                    { .. } => {debug_string.push_str("ParametersNode");                   }
            Node::ParenthesesNode                   { .. } => {debug_string.push_str("ParenthesesNode");                  }
            Node::PinnedExpressionNode              { .. } => {debug_string.push_str("PinnedExpressionNode");             }
            Node::PinnedVariableNode                { .. } => {debug_string.push_str("PinnedVariableNode");               }
            Node::PostExecutionNode                 { .. } => {debug_string.push_str("PostExecutionNode");                }
            Node::PreExecutionNode                  { .. } => {debug_string.push_str("PreExecutionNode");                 }
            Node::ProgramNode                       { .. } => {debug_string.push_str("ProgramNode");                      }
            Node::RangeNode                         { .. } => {debug_string.push_str("RangeNode");                        }
            Node::RationalNode                      { .. } => {debug_string.push_str("RationalNode");                     }
            Node::RedoNode                          { .. } => {debug_string.push_str("RedoNode");                         }
            Node::RegularExpressionNode             { .. } => {debug_string.push_str("RegularExpressionNode");            }
            Node::RequiredKeywordParameterNode      { .. } => {debug_string.push_str("RequiredKeywordParameterNode");     }
            Node::RequiredParameterNode             { .. } => {debug_string.push_str("RequiredParameterNode");            }
            Node::RescueModifierNode                { .. } => {debug_string.push_str("RescueModifierNode");               }
            Node::RescueNode                        { .. } => {debug_string.push_str("RescueNode");                       }
            Node::RestParameterNode                 { .. } => {debug_string.push_str("RestParameterNode");                }
            Node::RetryNode                         { .. } => {debug_string.push_str("RetryNode");                        }
            Node::ReturnNode                        { .. } => {debug_string.push_str("ReturnNode");                       }
            Node::SelfNode                          { .. } => {debug_string.push_str("SelfNode");                         }
            Node::ShareableConstantNode             { .. } => {debug_string.push_str("ShareableConstantNode");            }
            Node::SingletonClassNode                { .. } => {debug_string.push_str("SingletonClassNode");               }
            Node::SourceEncodingNode                { .. } => {debug_string.push_str("SourceEncodingNode");               }
            Node::SourceFileNode                    { .. } => {debug_string.push_str("SourceFileNode");                   }
            Node::SourceLineNode                    { .. } => {debug_string.push_str("SourceLineNode");                   }
            Node::SplatNode                         { .. } => {debug_string.push_str("SplatNode");                        }
            Node::StatementsNode                    { .. } => {debug_string.push_str("StatementsNode");                   }
            Node::StringNode                        { .. } => {debug_string.push_str("StringNode");                       }
            Node::SuperNode                         { .. } => {debug_string.push_str("SuperNode");                        }
            Node::SymbolNode                        { .. } => {debug_string.push_str("SymbolNode");                       }
            Node::TrueNode                          { .. } => {debug_string.push_str("TrueNode");                         }
            Node::UndefNode                         { .. } => {debug_string.push_str("UndefNode");                        }
            Node::UnlessNode                        { .. } => {debug_string.push_str("UnlessNode");                       }
            Node::UntilNode                         { .. } => {debug_string.push_str("UntilNode");                        }
            Node::WhenNode                          { .. } => {debug_string.push_str("WhenNode");                         }
            Node::WhileNode                         { .. } => {debug_string.push_str("WhileNode");                        }
            Node::XStringNode                       { .. } => {debug_string.push_str("XStringNode");                      }
            Node::YieldNode                         { .. } => {debug_string.push_str("YieldNode");                        }
        }
        debug_string.push_str(&format!(
            "[{:?}-{:?}]: {:?}",
            node.location().start_offset(),
            node.location().end_offset(),
            node.location(),
        ));
        self.debug_strings.push(debug_string);
    }
}
impl VisitAll<'_> for Visitor {
    fn node_enter(&mut self, node: &Node<'_>) {
        self.push_debug_string(&node);
        self.depth += 1;
    }
    fn node_leave(&mut self, _node: &Node<'_>) {
        self.depth -= 1;
    }
}

#[test]
fn run() {
    glob!("**/node_variants/**.rb", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        // first formatting
        let printer = Printer::new(contents, ());
        let (parse_result, formatted) = printer.print();
        // // second formatting
        // let printer = Printer::new(formatted.clone(), ());
        // let (reparsed_result, reformatted) = printer.print();
        // // parse result should have no errors
        // assert!(reparsed_result.errors().next().is_none(), "parsing errors");
        // formatting should be idempotent
        /*
        assert!(
            formatted == reformatted,
            "formatting idempotence: first:\n{}----second:\n{}----",
            formatted,
            reformatted
        );
        */
        // build AST debug string
        let mut visitor = Visitor {
            depth: 0,
            debug_strings: Vec::new(),
        };
        visitor.visit(&parse_result.node());
        // snapshot
        let output = format!(
            "****FORMATTED****\n{}\n\n****AST****\n{}",
            formatted,
            visitor.debug_strings.join("\n"),
        );
        assert_snapshot!(output);
    });
}
