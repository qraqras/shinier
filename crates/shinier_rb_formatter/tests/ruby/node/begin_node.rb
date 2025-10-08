# empty begin
begin
end

# single-line begin
begin; foo; end

# with rescue
begin
  foo
rescue => e
  handle(e)
end

# with ensure
begin
  foo
ensure
  cleanup
end

# with else
begin
  try_block
rescue
  rescue_block
else
  else_block
end

# nested begin
begin
  begin
    inner
  rescue
    inner_rescue
  end
rescue
  outer_rescue
end
