alias foo bar

# unnecessary line break
alias
  foo bar

# long expression
alias foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo barbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbar

alias # owning comment
  foo bar
alias
  # owning comment 1
  # owning comment 2
  foo bar
alias
=begin
block comment 1
block comment 2
=end
  foo bar
alias
=begin
block comment 1
block comment 2
=end
  foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo barbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbar
alias foo bar # trailing comment
alias
  foo bar # trailing comment
alias foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo barbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbar # trailing comment
