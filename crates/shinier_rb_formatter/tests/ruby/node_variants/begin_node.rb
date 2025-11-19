begin
end

begin
  do_something
end

# with rescue
begin
  do_something
rescue
  recover
end

# with ensure
begin
  do_something
ensure
  must_to_do
end

# with rescue and else
begin
  do_something
rescue
  recover
else
  handle_else
end

# with rescue and ensure
begin
  do_something
rescue
  recover
ensure
  must_to_do
end

# with rescue and else and ensure
begin
  do_something
rescue
  recover
else
  handle_else
ensure
  must_to_do
end

begin # comment 1
  # comment 2
end # comment 3

# comment 1
begin # comment 2
  # comment 3
  do_something # comment 4
  # comment 5
# comment 6
rescue ArgumentError => e # comment 7
  # comment 8
  recover_argument_error # comment 9
  # comment 10
# comment 11
rescue ZeroDivisionError => e # comment 12
  # comment 13
  recover_zero_division_error # comment 14
  # comment 15
# comment 16
else # comment 17
  # comment 18
  handle_else # comment 19
  # comment 20
# comment 21
ensure # comment 22
  # comment 23
  must_to_do # comment 24
  # comment 25
# comment 26
# comment 27
  # comment 28
end # comment 27
