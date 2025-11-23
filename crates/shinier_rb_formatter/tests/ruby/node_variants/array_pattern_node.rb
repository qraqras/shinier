foo in 1, 2
foo in [1, 2]
foo in *bar
foo in Bar[]
foo in Bar[1, 2, 3]

foo in 1, 2, *bar
foo in 1, *bar, 3
foo in *bar, 2, 3

# unnecessary line breaks
foo in
  1, 2
foo in [
  1,
  2
]
foo in 1,
  2

# long array patterns
foo in 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 11000, 12000, 13000, 14000, 15000, 16000, 17000, 18000, 19000, 20000

# implicit rest pattern
foo in [1, 2,]

foo in
  # owning comment 1
  1, # trailing comment 1
  # owning comment 2
  *bar, # trailing comment 2
  # owning comment 3
  3 # trailing comment 3

foo in Bar # trailing comment
# leading comment
[
  # owning comment 1
  1, # trailing comment 1
  # owning comment 2
  *bar, # trailing comment 2
  # owning comment 3
  3, # trailing comment 3
]
