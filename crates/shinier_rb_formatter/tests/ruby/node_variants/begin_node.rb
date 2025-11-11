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
ensure # comment 12
  # comment 13
  must_to_do # comment 14
  # comment 15
# comment 16
end
