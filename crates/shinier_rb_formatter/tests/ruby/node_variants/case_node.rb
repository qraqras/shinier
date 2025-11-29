case true
when false
end

# unnecessary line breaks
case
  true
when
  false
end

# long expressions
case very_long_expression_name_very_long_expression_name
when another_very_long_expression_name_another_very_long_expression_name
end

case very_long_expression_name_very_long_expression_name
when 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
end

case foo
  # comment 1
# comment 2
when 1
  # comment 3
# comment 4
when 2 then
  # comment 5
# comment 6
when 3
  puts 3
  # comment 7
# comment 8
when 4 then
  puts 4
  # comment 9
# comment 10
else
  # comment 11
# comment 12
end

case foo # comment 1
when 1 # comment 2
when 2 # comment 3
  puts 2
when 3 then # comment 4
when 4 then # comment 5
  puts 4
else # comment 6
end

case
  foo # comment
when
  1 # comment
end

case # comment
  foo
when # comment
  1
end

case
  # comment
  foo
when
  # comment
  1
end

case foo
when 1 then # comment
  puts 1
when 2
  then # comment
  puts 2
when
  3 then # comment
  puts 3
when # comment
  4 then # comment
  puts 4
when 5 # comment
  then # comment
  puts 5
when 6
  # comment
  then
  puts 6
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


case foo
when 1, 2 # comment
  puts 0
end

case foo
when 1,
  2 # comment
  puts 0
end

case foo
when 1, 2
  # comment
  puts 0
end

case foo
when 1, # comment
  2
  puts 0
end

case foo
when
  # comment
  1, 2
  puts 0
end
