bar(&args)

bar(
  &
  args
)

bar(
  # leading comment 1
  &args
)

bar(&args # trailing comment 1
)

bar(&args
  # trailing comment 1
)


bar(
  # leading comment 1
  &args # trailing comment 1
  # trailing comment 2
)

bar(& # trailing comment 1
  args # trailing comment 2
)

bar(
  # leading comment 1
  & # trailing comment 1
  # leading comment 2
  args # trailing comment 2
  # trailing comment 3
)

def foo(&)
  bar(&)
end

def foo(&)
  bar(
    &
  )
end

def foo(&)
  bar(
    # leading comment 1
    & # trailing comment 1
    # trailing comment 2
  )
end
