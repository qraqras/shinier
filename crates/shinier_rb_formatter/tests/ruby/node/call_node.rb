foo
foo()
foo(bar)
foo(bar, baz, bazz, bar, baz, bazz, bar, baz, bazz, a: bar, b: baz, c: bazz)
+foo
foo + bar
foo.bar
foo&.bar

# operator method calls
foo | bar
foo ^ bar
foo & bar
foo <=> bar
foo == bar
foo === bar
foo =~ bar
foo > bar
foo >= bar
foo < bar
foo <= bar
foo << bar
foo >> bar
foo + bar
foo - bar
foo * bar
foo / bar
foo % bar
foo ** bar
foo ~ bar
+foo # foo +@ bar
-foo # foo -@ bar
foo[bar] # foo [] bar
foo[bar] = baz # foo []= bar
`foo` # foo ` bar
!foo # foo ! bar
foo != bar
foo !~ bar

# self-defined method calls
foo += bar
foo -= bar
foo *= bar
foo /= bar
foo %= bar
foo **= bar
foo &= bar
foo |= bar
foo ^= bar
foo <<= bar
foo >>= bar
foo &&= bar
foo ||= bar

# block
foo(&bar)
foo(bar, &baz)

# block literals (single-line and multi-line)
foo { puts "x" }
foo do |x|
	x * 2
end

# block with explicit arguments and destructuring
foo(bar) { |a, b| p [a, b] }
foo { |(a, b)| p a, b }

# method call with block (receiver method)
foo.bar { |v| v }
foo&.bar { |v| v }

# passing a Proc/lambda as a block
foo(&proc_obj)


foo and bar
foo && bar
foo or bar
foo || bar
not foo
!foo
