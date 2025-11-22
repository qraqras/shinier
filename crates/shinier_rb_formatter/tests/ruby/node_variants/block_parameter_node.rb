def foo(&bar)
end

def foo(&
  bar)
end

def foo(
  # leading comment
  &bar # trailing comment
  # dangling comment
)
end

def foo(
  # leading comment 1
  & # trailing comment 1
  # leading comment 2
  bar # trailing comment 2
  # dangling comment 2
)
end

def foo(&bar # trailing comment
  # dangling comment
)
end

def foo(&
  # leading comment
  bar # trailing comment
  # dangling comment
)
end

def foo(
  # comment
)
end

def foo
  # comment
end
