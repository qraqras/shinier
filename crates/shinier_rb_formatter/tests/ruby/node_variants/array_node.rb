[1, 2, 3]

# empty array
[]

# single element array
[1]

# unnecessary commas
[1,]

# unnecessary line breaks
[
  1
]

# long array
[1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 11000, 12000, 13000, 14000, 15000, 16000, 17000, 18000, 19000, 20000]

[
  # dangling comment
]

[
  1, # trailing comment 1
  2, # trailing comment 2
  # owning comment
  3, # trailing comment 3
  # dangling comment
] # trailing comment 4

# percent literal arrays
%w(foo bar)
%W[foo #{bar}]
%i{foo bar}
%I\foo #{bar}\

# excessive percent literal arrays
%w()
%W(foo, bar)
%i()
%I(foo, bar)

# recommended percent literal arrays
["foo", "bar"]
[:foo, :bar]

# not recommended percent literal arrays
["foo"]
[:foo]
["foo", :bar]

# escape sequences in percent literal arrays
%w(foo\ bar\t baz\n)
