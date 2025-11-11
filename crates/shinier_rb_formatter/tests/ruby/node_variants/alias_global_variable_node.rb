alias $foo $bar

# unnecessary line break
alias
  $foo $bar

# long expression
alias $foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo $barbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbar

alias # owning inline comment
  $foo $bar
alias
  # owning inline comment 1
  # owning inline comment 2
  $foo $bar
alias
=begin
owning block comment 1
owning block comment 2
=end
  $foo $bar
alias $foo $bar # trailing comment
alias $foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo $barbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbar # trailing comment
