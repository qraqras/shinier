case true
in false
end

case true
in false
  puts "false"
end

case true
in falne
  puts "false"
else
  puts "true"
end

case [true, false]
in [true, true]
  puts "t-t"
in [true, false]
  puts "t-f"
in [false, true]
  puts "f-t"
in [false, false]
  puts "f-f"
end
