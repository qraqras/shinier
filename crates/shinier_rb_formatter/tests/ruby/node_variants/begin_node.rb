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
  # comment 7
rescue ArgumentError => e # comment 8
  # comment 9
  recover_argument_error # comment 10
  # comment 11
# comment 12
  # comment 13
rescue ZeroDivisionError => e # comment 14
  # comment 15
  recover_zero_division_error # comment 16
  # comment 17
# comment 18
  # comment 19
else # comment 20
  # comment 21
  handle_else # comment 22
  # comment 23
# comment 24
  # comment 25
ensure # comment 26
  # comment 27
  must_to_do # comment 28
  # comment 29
# comment 30
  # comment 31
end # comment 32

begin # begin 1
  # begin 2
# rescue 1
  # rescue 2
rescue # rescue 3
  # rescue 4
# else 1
  # else 2
else # else 3
  # else 4
# ensure 1
  # ensure 2
ensure # ensure 3
  # ensure 4
# ensure 5
end
