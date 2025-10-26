:foo
%i[foo]

# 基本的なシンボル
:symbol

# アンダースコア付き
:my_symbol

# 疑問符付き
:valid?

# 感嘆符付き
:save!

# イコール付き
:name=

# 演算子シンボル
:+
:-
:*
:/
:%
:**
:==
:!=
:<
:>
:<=
:>=
:<=>
:===
:=~
:!~
:[]
:[]=
:<<
:>>
:&
:|
:^
:~
:!

# クォート付きシンボル
:'single quoted symbol'
:"double quoted symbol"

# スペースを含むシンボル
:"symbol with spaces"

# 特殊文字を含むシンボル
:"symbol-with-dashes"
:"symbol.with.dots"
:"symbol@with@at"

# エスケープシーケンスを含むシンボル
:"symbol\nwith\nnewlines"
:"symbol\twith\ttabs"

# 補間を含むシンボル
:"symbol_#{value}"
:"prefix_#{x + y}_suffix"

# 空のシンボル
:""

# Unicode文字を含むシンボル
:日本語
:émoji
:🚀

# メソッド名としてのシンボル
def foo
  :method_name
end

# ハッシュのキーとして
{ :key => :value }
{ key: :value }

# 配列内
[:first, :second, :third]

# case文で
case value
when :option1
  :result1
when :option2
  :result2
end
