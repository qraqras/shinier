while true
  break
end

while true
  break 0
end

while true
  break 0, 1
end

while true
  break 0,
    1
end

while true
  break 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000
end

while true
  # leading comment 1
  break 0, # trailing comment 1
    # leading comment 2
    1 # trailing comment 2
    # trailing comment 3
end
