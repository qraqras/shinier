left and right
left && right

# unnecessary line break
left and
  right
left &&
  right

# long expression
foo and bar and baz and foo and bar and baz and foo and bar and baz and foo and bar and baz and foo and bar and baz
foo && bar && baz && foo && bar && baz && foo && bar && baz && foo && bar && baz && foo && bar && baz

# long expression with unnecessary line breaks
foo and bar and baz and
foo and bar and baz and
foo and bar and baz and
foo and bar and baz and
foo and bar and baz
foo && bar && baz &&
foo && bar && baz &&
foo && bar && baz &&
foo && bar && baz &&
foo && bar && baz

a and b and c and
d and # trailing comment 1
e
# leading comment 1
f && # trailing comment 1
# leading comment 2
g # trailing comment 2
# trailing comment 3
