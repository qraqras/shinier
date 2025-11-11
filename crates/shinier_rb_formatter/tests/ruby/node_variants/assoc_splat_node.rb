{ **foo }

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
{ very_long_key_name: value, **very_long_hash_variable_name }
{ **very_long_hash_variable_name, another_very_long_key_name: value }

# Comments
{ **foo } # trailing comment
{
  a: 1, # trailing comment 1
  **foo # trailing comment 2
}
{
  # owning comment
  **foo
}

# Multiline
{
  a: 1,
  **foo,
  b: 2
}

# Empty hash with splat
{ **{} }
