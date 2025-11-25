{ **foo }
{ **
  foo }

# With other keys
{ a: 1, **foo }
{ **foo, b: 2 }
{ a: 1, **foo, b: 2 }

# Multiple splats
{ **foo, **bar }
{ a: 1, **foo, b: 2, **bar }

# Nested hashes
{ **{ a: 1, b: 2 } }
{ a: 1, **{ nested: value } }

# With hash rocket syntax
{ :key => :value, **foo }
{ **foo, :key => :value }

# Complex expressions
{ **foo.bar }
{ **method_call(arg) }
{ **some_hash.merge(other_hash) }

# Unnecessary line breaks
{
  **foo
}
{ a: 1,
  **foo }

# Long expressions
{ **very_long_hash_variable_name_very_long_hash_variable_name_very_long_hash_variable_name_very_long_hash_variable_name }

{
  # leading comment 1
  **foo
}
{
  **foo # trailing comment 1
}

{
  **foo
  # trailing comment 1
}

{
  ** # trailing comment 1
  foo # trailing comment 2
}

{
  # leading comment 1
  ** # trailing comment 1
  # trailing comment 2
  foo # trailing comment 3
  # trailing_comment_4
}

# Empty hash with splat
{ **{} }
