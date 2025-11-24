alias foo bar

# unnecessary line break
alias
  foo bar

# long expression
alias foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo barbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbar

alias # trailing comment 1
  foo bar
alias
  # leading comment 1
  # leading comment 2
  foo bar
alias
=begin
leading block comment 1
leading block comment 2
=end
  foo bar
alias
=begin
leading block comment 1
leading block comment 2
=end
  foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo barbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbar
alias foo bar # trailing comment 1
alias
  foo bar # trailing comment 1
alias foofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoofoo barbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbarbar # trailing comment 1
