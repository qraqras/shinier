begin
rescue Foo, *splat, Bar => ex
  foo
end

begin
rescue Foo => ex
  foo
rescue Bar => ex
  bar
end
