# simple begin/rescue/else
begin
  try_block
rescue
  rescue_block
else
  else_block
end

# multiple rescues with else and ensure
begin
  may_raise
rescue Foo => e
  handle_foo(e)
rescue
  handle_any
else
  no_exception_path
ensure
  always_cleanup
end

# single-line form
begin; foo; rescue; bar; else; baz; end

# nested begin with inner else
begin
  begin
    inner_try
  rescue
    inner_rescue
  else
    inner_else
  end
rescue
  outer_rescue
end
