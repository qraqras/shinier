[1, 2, 3].each { |i| puts x }

[1, 2, 3].each {
  |i| puts x
}

[1, 2, 3].each {
  |i|
  puts x
}

[1, 2, 3].each {
  |i|
  puts x
  puts x
}

[1, 2, 3].each { |i|
  puts x
  puts x
}

[1, 2, 3].each { |a, b, c, d, e, f, g, h, i| puts x }

[1, 2, 3].each { |a, b, c, d, e, f, g, h, i|
  puts x
}

[1, 2, 3].each { |a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z| puts x }

[1, 2, 3].each { |a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z|
  puts x
}

[1, 2, 3].each { |i| puts x; puts y; puts z }

[1, 2, 3].each { # trailing comment 1
  |i|
  puts x
}

[1, 2, 3].each {
  |i| # trailing comment 1
  puts x
}

[1, 2, 3].each {
  |i|
  puts x # trailing comment 1
}

[1, 2, 3].each { # trailing comment 1
  |i| # trailing comment 2
  puts x # trailing comment 3
}

[1, 2, 3].each {
  |i|
  # leading comment 1
  puts x
}

[1, 2, 3].each {
  |i|
  puts x
  # trailing comment 1
}

[1, 2, 3].each { # trailing comment 1
  # leading comment 1
  |i| # trailing comment 2
  # leading comment 2
  puts x # trailing comment 3
  # trailing comment 4
}
