foo => bar | baz

# unnecessary line break
foo =>
  bar | baz
foo => bar |
  baz

# long expression
foo => bar | barbar | barbarbarbar | barbarbarbarbar | barbarbarbarbarbar | barbarbarbarbarbarbar | barbarbarbarbarbarbarbar

# long expression with unnecessary line break
foo => bar | barbar | barbarbarbar | barbarbarbarbar |
barbarbarbarbarbar | barbarbarbarbarbarbar | barbarbarbarbarbarbarbar

foo =>
  # leading comment
  bar | baz
foo => bar | # trailing comment 1
  # leading comment 1
  # leading comment 2
  baz # trailing comment 2
