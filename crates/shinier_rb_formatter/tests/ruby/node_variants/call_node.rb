foo
foo()
+foo
foo + bar
foo.bar
foo&.bar

not foo
foo + bar
foo.+@(bar)
puts foo
puts (foo)
foo(&block)
foo(bar, &block)
foo(bar) { |baz| puts baz }
foo(bar) { |baz|
  puts baz
  puts baz
}

+
  foo
foo +
  foo
foo(
  bar
)
foo(bar,
  baz
)

+ # trailing comment 1
  # leading comment 1
  foo # trailing comment 2
foo + # trailing comment 1
  # leading comment 1
  foo # trailing comment 2
foo(
  # leading comment 1
  bar # trailing comment 1
  # trailing comment 2
)
foo {
  # dangling comment 1
}
