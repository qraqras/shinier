def foo(&)
  bar(&)
end

bar(&args)

bar(
  &
  args
)

bar(
  # leading comment
  &args # trailing comment
  # dangling comment
)

bar(
  # leading comment
  & # trailing comment 1
  # dangling comment 1
  args # trailing comment 2
  # dangling comment 2
)
