case true
when true
end

# unnecessary line breaks
case
  true
when
  true
end

# long expressions
case very_long_expression_name_very_long_expression_name
when another_very_long_expression_name_another_very_long_expression_name
end

case foo
  # trailing comment 1
when 1
  # dangling comment 1
end

case # trailing comment 1
  foo # trailing comment 2
  # trailing comment 3
# leading comment 1
when 1 # trailing comment 4
  # trailing comment 5
# leading comment 2
when 2 # trailing comment 6
  # leading comment 3
  put 2 # trailing comment 7
  # trailing comment 8
# leading comment 4
else # trailing comment 9
  # leading comment 5
  puts bar # trailing comment 10
  # trailing comment 11
# leading comment 12
end
