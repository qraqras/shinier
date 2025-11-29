foo.bar &&= value
foo.bar &&=
  value

# long expression
very_long_receiver_name.very_long_method_name &&= very_long_value_expression

foo.bar &&=
  value # trailing comment 1

foo.bar &&= # trailing comment 1
  value # trailing comment 2

foo.bar &&=
  # leading comment 1
  value
