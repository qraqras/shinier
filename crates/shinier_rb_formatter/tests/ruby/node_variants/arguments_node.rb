return foo, bar, baz

# unnecessary line breaks
return foo, bar,
baz
return foo,
bar,
baz

# long argument list
return foo, bar, baz, foo, bar, baz, foo, bar, baz, foo, bar, baz, foo, bar, baz, foo, bar, baz, foo, bar, baz, foo, bar, baz

return foo,
  # leading comment 1
  bar, # trailing comment 1
  baz # trailing comment 2
