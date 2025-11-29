class Foo; end
class Foo < Bar; end
class Foo
end
class Foo < Bar
end

class Foo; puts 0; end
class Foo < Bar; puts 0; end
class Foo
  puts 0
end
class Foo < Bar
  puts 0
end

# unnecessary line breaks
class
  Foo
end
class
  Foo <
    Bar
end

# long expression
class FooFooFooFooFooFooFooFooFooFooFooFooFooFooFooFoo < BarBarBarBarBarBarBarBarBarBarBarBarBarBarBarBar; end

class # traling comment 1
  Foo
end
class
  # leading comment 1
  Foo
end
class Foo # traling comment 1
end
class Foo
  # tlailing comment
end

class Foo < Bar
  # trailing comment 1
end

class Foo < # trailing comment 1euaoeuauao
  Bar
end

class
  # leading comment 1
  # leading comment 2
  Foo <
    Bar
end
