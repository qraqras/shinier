-> (a, b = 1; local) { }
-> (
  a, b = 1
  ;
  local
) { }

foo { |a, b = 1; local| }
foo { |
  a, b = 1; local
| }
foo { |
  a,
  b = 1;
  local
| }

foo { |a, b = 1| }
foo { |; local| }
foo { || }


-> (
  # leading comment 1
  a, b = 1; local
) { }
foo {
  # leading comment 1
  |a, b = 1; local|
}
foo {
  |
    # leading comment 1
    a, b = 1; local
  |
}
foo {
  |
    a, b = 1; # trailing comment 1
    local
  |
}
foo {
  |
    a, b = 1;
    # trailing comment 1
    local
  |
}
# FIXME: semicolon placement
foo {
  |
    a, b = 1
    # trailing comment 1
    ;
    local
  |
}
foo {
  |
    a, b = 1 # trailing comment 1
    ;
    local
  |
}

foo do |a, b = 1; local| end
