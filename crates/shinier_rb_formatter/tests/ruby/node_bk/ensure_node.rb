# simple ensure
begin
  foo
ensure
  cleanup
end

# ensure with rescue
begin
  risky
rescue
  handle
ensure
  always_cleanup
end

# single-line ensure
begin; foo; ensure; cleanup; end

# nested ensure
begin
  begin
    inner
  ensure
    inner_cleanup
  end
ensure
  outer_cleanup
end

# ensure that contains an explicit return (formatting only)
def with_return
  begin
    1
  ensure
    return 2
  end
end
