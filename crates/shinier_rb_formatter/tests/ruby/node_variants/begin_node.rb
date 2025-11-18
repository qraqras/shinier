begin
  do_something
end

begin
  do_something
rescue
  recover
end

begin
  do_something
ensure
  must_to_do
end

begin
  do_something
rescue
  recover
else
  handle_else
end

begin
  do_something
rescue
  recover
ensure
  must_to_do
end

begin
  do_something
rescue
  recover
else
  handle_else
ensure
  must_to_do
end

# comment 1
begin # comment 2
  # comment 3
  do_something # comment 4
  # comment 5
# comment 6
rescue # comment 7
  # comment 8
  recover # comment 9
  # comment 10
# comment 11
else # comment 12
  # comment 13
  handle_else # comment 14
  # comment 15
# comment 16
ensure # comment 17
  # comment 18
  must_to_do # comment 19
  # comment 20
# comment 21
end
