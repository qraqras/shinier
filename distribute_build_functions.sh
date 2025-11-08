#!/bin/bash

SOURCE_FILE="/workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/_new_build_node.rs"
TARGET_DIR="/workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant"

if [ ! -f "$SOURCE_FILE" ]; then
    echo "Error: Source file not found: $SOURCE_FILE"
    exit 1
fi

if [ ! -d "$TARGET_DIR" ]; then
    echo "Error: Target directory not found: $TARGET_DIR"
    exit 1
fi

echo "Extracting build functions from _new_build_node.rs..."

# Python スクリプトを使用
python3 << 'PYTHON_END'
import re
import os

source_file = "/workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/_new_build_node.rs"
target_dir = "/workspaces/shinier/crates/shinier_rb_formatter/src/builder/prism/new_build_node_variant"

with open(source_file, 'r') as f:
    content = f.read()

# 各 build_*_node 関数を抽出
pattern = r'(pub fn build_(\w+)_node\([^)]+\)[^{]*\{(?:[^{}]|{[^{}]*})*\})'
matches = re.finditer(pattern, content, re.MULTILINE | re.DOTALL)

count = 0
for match in matches:
    func_code = match.group(1)
    node_name = match.group(2)
    
    # ファイル名を生成
    file_name = f"{node_name}_node.rs"
    file_path = os.path.join(target_dir, file_name)
    
    # ファイルに書き込み
    if os.path.exists(file_path):
        with open(file_path, 'w') as f:
            f.write(f"// filepath: {file_path}\n\n")
            f.write(func_code)
            f.write("\n")
        print(f"Wrote: {file_name}")
        count += 1

print(f"\nDone! Distributed {count} functions.")
PYTHON_END

