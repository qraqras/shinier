foo => [bar => baz]

# unneccesary line breaks
foo => [bar =>
  baz
]

# long expression
very_long_variable_name => [very_long_key_name => very_long_value_expression]

foo => [
  # leading comment 1
  bar => baz
]
foo => [bar => # trailing comment 1
  baz
]
foo => [bar =>
  # leading comment 1
  baz
]
foo => [bar =>
  baz # trailing comment 1
]
foo => [bar =>
  baz
  # trailing comment 1
]
