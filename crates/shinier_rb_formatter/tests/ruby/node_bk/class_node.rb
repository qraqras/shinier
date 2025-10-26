# bad
FooError = Class.new(StandardError)

# okish
class FooError < StandardError; end

# ok
class FooError < StandardError
end

class FooError < StandardError
    include Bar
end

class Foo::Bar < baz
end
