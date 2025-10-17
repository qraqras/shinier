# 基本的な forwarding super
super

# ブロック付き - 親メソッドにブロックを渡す
super { |item| puts item }

# ブロック付き - 複数行
super do |item|
  process(item)
  yield item
end

# ブロック付き - 複雑な処理
super { |x, y|
  result = x + y
  result * 2
}

# ブロック付き - ブロックパラメータなし
super { puts "no parameters" }

# ブロック付き - ネストしたブロック
super { |items|
  items.each { |item|
    puts item
  }
}

# メソッドチェーンとブロック
super { |x| x * 2 }.map(&:to_s)

# 条件式内での super とブロック
result = if condition
  super { |x| x + 1 }
else
  default_value
end
