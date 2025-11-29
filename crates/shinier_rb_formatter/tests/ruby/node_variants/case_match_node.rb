case true
in false
end

# unnecessary line breaks
case
  true
in
  false
end

# long expressions
case very_long_expression_name_very_long_expression_name
in another_very_long_expression_name_another_very_long_expression_name
end

case foo
  # comment 1
# comment 2
in 1
  # comment 3
# comment 4
in 2 then
  # comment 5
# comment 6
in 3
  puts 3
  # comment 7
# comment 8
in 4 then
  puts 4
  # comment 9
# comment 10
else
  # comment 11
# comment 12
end

case foo # comment 1
in 1 # comment 2
in 2 # comment 3
  puts 2
in 3 then # comment 4
in 4 then # comment 5
  puts 4
else # comment 6
end

case
  foo # comment
in
  1 # comment
end

case # comment
  foo
in # comment
  1
end

case
  # comment
  foo
in
  # comment
  1
end

case foo
in 1 then # comment
  puts 1
in 2
  then # comment
  puts 2
in
  3 then # comment
  puts 3
in # comment
  4 then # comment
  puts 4
in 5 # comment
  then # comment
  puts 5
in 6
  # comment
  then
  puts 6
end

case # trailing comment 1
  foo # trailing comment 2
  # trailing comment 3
# leading comment 1
in 1 # trailing comment 4
  # trailing comment 5
# leading comment 2
in 2 # trailing comment 6
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
