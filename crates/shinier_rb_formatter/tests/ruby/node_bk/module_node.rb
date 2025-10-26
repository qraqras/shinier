module Foo end

module Bar
end

module Baz
  def hello
    puts "Hello"
  end

  def world
    puts "World"
  end
end

module Nested::Module::Path
end

module VeryLongModuleNameThatShouldBeFormattedNicely
  include SomeModule
  extend AnotherModule

  CONSTANT = 42

  def method_one
    # implementation
  end

  def method_two
    # implementation
  end
end
