foo.bar += baz

# unnecessary line break
foo.bar +=
  baz

# long expression
very_long_receiver_name.very_long_method_name += very_long_value_expression

foo.bar += # trailing comment 1
  baz
foo.bar +=
  # leading comment 1
  baz
foo.bar +=
  baz # trailing comment 1
