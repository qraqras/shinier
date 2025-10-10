def a(b, c, d)
end

# Test case: required parameters only
def required_only(x, y, z)
end

# Test case: optional parameters
def with_optionals(a, b = 1, c = "default")
end

# Test case: rest parameter
def with_rest(*args)
end

# Test case: post parameters
def with_posts(a, *rest, b)
end

# Test case: keyword parameters
def with_keywords(a:, b: 2)
end

# Test case: keyword rest
def with_keyword_rest(**kwargs)
end

# Test case: block parameter
def with_block(&block)
end

# Test case: all parameters combined
def all_params(req, opt = 1, *rest, post, kw:, kw_opt: 3, **kw_rest, &blk)
end

# Test case: no parameters
def no_params
end
