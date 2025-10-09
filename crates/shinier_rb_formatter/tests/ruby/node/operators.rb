# Arithmetic
1 + 2
1 - 2
1 * 2
1 / 2
1 % 2
2 ** 3

# Unary
-1
 +1

# Bitwise
1 & 2
1 | 2
1 ^ 2
~1
1 << 2
1 >> 2

# Comparison
1 == 2
1 != 2
1 < 2
1 <= 2
1 > 2
1 >= 2
1 <=> 2
case x
when 1..3
  :in_range
end

# Equality operators
obj === klass
/foo/ === str

# Regex match
"abc" =~ /b/
"abc" !~ /z/

# Logical
true && false
true || false
!true

# Ternary
cond ? a : b

# Assignment
foo = 1
foo += 2
foo -= 2
foo *= 2
foo /= 2
foo %= 2
foo **= 2

# Bitwise assignment
foo <<= 1
foo >>= 1
foo &= 1
foo |= 1
foo ^= 1

# Indexing and assignment
arr[0]
arr[1,2]
arr[0] = 1
hash[:a]
hash[:a] = 1

# Range operators
1..5
1...5

# Safe navigation and member access
obj.method
obj&.method

# Operator as method call
foo + bar
foo - bar

# Assignment to attribute
obj.attr = value
obj.attr += 1

# Compound logical assignment (Ruby 2.3+)
obj.attr ||= default
obj.attr &&= other

# Proc/block passing
arr.map { |x| x * 2 }
arr.map(&:to_s)

# Defined? operator
defined?(some_var)

# Splats and keyword args with operators in calls
func(a, *rest, b: 1, **kw)
