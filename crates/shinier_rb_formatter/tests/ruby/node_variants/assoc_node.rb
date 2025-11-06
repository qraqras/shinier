{ a => b }
{:foo => :bar}
{ "foo" => "bar", }
{ foo: bar }

# Multiple associations
{ a => b, c => d }
{ :foo => 1, :bar => 2 }
{ "foo" => "1", "bar" => "2" }
{ foo: 1, bar: 2 }

# Unnecessary line breaks
{ a =>
  b }
{
  :foo =>
  bar
}
{
  "foo" => "1", "bar" =>
  "2"
}

# Long associations
{ foo1 => bar1, foo2 => bar2, foo3 => bar3, foo4 => bar4, foo5 => bar5, foo6 => bar6 }
{ very_long_key_name_that_exceeds_line_limit_1 => very_long_value_name_that_exceeds_line_limit_1, very_long_key_name_that_exceeds_line_limit_2 => very_long_value_name_that_exceeds_line_limit_2 }

# Nested hashes
{ a => { b => c } }
{ a => { :b => { "c" => "d" } } }

# Complex values
{ a => [1, 2, 3] }
{ :a => foo.bar.baz }
{ "a" => method_call(arg1, arg2) }

# Comments
{ a => b } # trailing comment
{
  a => b, # trailing comment 1
  c => d # trailing comment 2
}
{
  # owning comment
  a => b
}
{
  a =>
  # owning comment
  b
}

# Multiline values
{
  a => {
    :nested => :value
  }
}
{
  :key => [
    1,
    2,
    3
  ]
}
