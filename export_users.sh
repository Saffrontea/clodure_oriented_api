#!/bin/bash

# 設定
API_URL="http://127.0.0.1:3030/users"
OUTPUT_FILE="users_export.csv"

echo "ユーザーデータのエクスポートを開始します..."

# CSVヘッダーを作成
echo "ID,名前,メールアドレス" > "${OUTPUT_FILE}"

# APIから全ユーザーデータを取得
echo "APIからデータを取得中..."
response=$(curl -s "${API_URL}")

# レスポンスからユーザー数を取得
user_count=$(echo "${response}" | jq -r '.data | length')

if [[ ${user_count} -eq 0 ]]; then
    echo "ユーザーデータが見つかりません。"
    exit 1
fi

echo "${user_count} 件のユーザーデータを取得しました。CSVに変換中..."

# ユーザーデータをCSVに変換
for ((i=0; i<user_count; i++)); do
    user_id=$(echo "${response}" | jq -r ".data[${i}].id")
    name=$(echo "${response}" | jq -r ".data[${i}].name")
    email=$(echo "${response}" | jq -r ".data[${i}].email")

    # CSVに追加（カンマを含む可能性があるのでダブルクォートで囲む）
    echo "\"${user_id}\",\"${name}\",\"${email}\"" >> "${OUTPUT_FILE}"

    # 進捗表示
    if ((i % 10 == 0)); then
        echo -ne "\r処理中... $((i+1))/${user_count} ($(( (i+1) * 100 / user_count ))%)"
    fi
done

echo -e "\n\nエクスポート完了! ファイル: ${OUTPUT_FILE}"
echo "総レコード数: ${user_count}"

# ファイルの先頭5行を表示
echo -e "\nファイルのサンプル (先頭5行):"
head -n 5 "${OUTPUT_FILE}"
