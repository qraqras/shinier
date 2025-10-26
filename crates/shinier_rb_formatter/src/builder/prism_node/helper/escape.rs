pub fn escape(input: &[u8]) -> String {
    let mut result = String::new();
    for &byte in input {
        match byte {
            b'\\' => result.push_str("\\\\"),  // "\": バックスラッシュ
            b'\x09' => result.push_str("\\t"), // "\t": タブ
            b'\x0b' => result.push_str("\\v"), // "\v": 垂直タブ
            b'\x0a' => result.push_str("\\n"), // "\n": 改行
            b'\x0d' => result.push_str("\\r"), // "\r": キャリッジリターン
            b'\x0c' => result.push_str("\\f"), // "\f": 改ページ
            b'\x08' => result.push_str("\\b"), // "\b": バックスペース
            b'\x07' => result.push_str("\\a"), // "\a": ベル
            b'\x1b' => result.push_str("\\e"), // "\e": エスケープ
            // b'\x20' => result.push_str("\\s"), // "\s": 空白
            other => result.push(other as char),
        }
    }
    result
}
