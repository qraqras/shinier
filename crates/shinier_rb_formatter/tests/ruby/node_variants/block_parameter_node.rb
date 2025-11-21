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

def foo(&bar
  # aaaaaa
)
end
