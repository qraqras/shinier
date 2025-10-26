def method
end

# Test case: method with parameters
def method_with_params(a, b = 1, *rest, c, d:, **kwargs, &block)
  puts a
end

# Test case: class method
def self.class_method
  "class method"
end

# Test case: method with receiver
def obj.instance_method
  "instance method"
end

# Test case: empty method with locals
def method_with_locals
  local_var = 42
  another_var = "hello"
end

# Test case: method with complex body
def complex_method(x)
  if x > 0
    return x * 2
  else
    raise "negative"
  end
end
