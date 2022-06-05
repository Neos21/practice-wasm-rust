#!/bin/bash

echo '========================================'
echo '動作する最終成果物だけを取り出すためのスクリプト'
echo '========================================'

# 最終成果物を配置するディレクトリ名
DIST_DIR='./docs'
# WASM ファイルの名前 (プロジェクト名)
WASM_NAME='practice_wasm_rust'

# WASM をビルドする
wasm-pack build --target web

# 最終成果物用のディレクトリを作り直す
rm -rf "${DIST_DIR}"
mkdir -p "${DIST_DIR}/pkg"

# 最終成果物となるファイルをコピーする
cp ./index.html                 "${DIST_DIR}"
cp "./pkg/${WASM_NAME}.js"      "${DIST_DIR}/pkg"
cp "./pkg/${WASM_NAME}_bg.wasm" "${DIST_DIR}/pkg"

echo '========================================'
echo 'Finished. Try This Command :'
echo "  $ npx sirv-cli '${DIST_DIR}'" --host
echo "  $ npx live-server --host=0.0.0.0 '${DIST_DIR}'"
echo '========================================'
