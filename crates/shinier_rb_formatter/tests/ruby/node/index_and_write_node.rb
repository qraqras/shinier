# 基本的なインデックスアクセス
array[0] &&= value

# メソッドチェーン後のインデックスアクセス
foo.bar[baz] &&= value

# ハッシュのインデックスアクセス
hash[:key] &&= value

# 複数の引数を持つインデックスアクセス
matrix[i, j] &&= value

# セーフナビゲーション演算子を使用
array&.[0] &&= value

# セーフナビゲーション演算子とメソッドチェーン
foo&.bar[baz] &&= value

# ブロック付きのインデックスアクセス
array[find { |x| x > 0 }] &&= value

# 複雑な式をインデックスとして使用
array[x + y * z] &&= new_value

# ネストしたインデックスアクセス
matrix[0][1] &&= value
