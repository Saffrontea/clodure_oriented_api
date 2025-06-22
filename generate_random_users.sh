#!/bin/bash

# 設定
API_URL="http://localhost:3030/users"
NUM_USERS=100000  # 生成するユーザー数
DELAY=0      # リクエスト間の遅延（秒）
MAX_PARALLEL=100  # 最大並列リクエスト数

# 日本の姓のリスト
LAST_NAMES=("佐藤" "鈴木" "高橋" "田中" "伊藤" "渡辺" "山本" "中村" "小林" "加藤" 
           "吉田" "山田" "佐々木" "山口" "松本" "井上" "木村" "林" "斎藤" "清水" 
           "山崎" "阿部" "森" "池田" "橋本" "山下" "石川" "中島" "前田" "藤田")

# 日本の名のリスト
FIRST_NAMES=("翔太" "大輔" "健太" "拓也" "達也" "直樹" "明" "浩" "誠" "大樹" 
            "美咲" "結衣" "優子" "友美" "恵" "真由美" "明美" "愛" "由美子" "裕子" 
            "一郎" "次郎" "三郎" "四郎" "五郎" "花子" "葵" "さくら" "陽子" "智子")

# ドメインのリスト
DOMAINS=("example.com" "sample.jp" "test.co.jp" "demo.jp" "mail.com" 
        "company.co.jp" "bizmail.jp" "testmail.com" "mymail.jp" "email.co.jp")

# カウンター表示関数
show_progress() {
    local current=$1
    local total=$2
    local percent=$((current * 100 / total))
    local bar_length=50
    local completed=$((bar_length * current / total))
    local remaining=$((bar_length - completed))

    # プログレスバーを作成
    local bar="["
    for ((i=0; i<completed; i++)); do bar+="="; done
    for ((i=0; i<remaining; i++)); do bar+=" "; done
    bar+="]"

    # 前の行を消去して新しい情報を表示
    echo -ne "\r${bar} ${percent}% (${current}/${total}) 完了"
}

# 結果記録用の変数
SUCCESS_COUNT=0
FAIL_COUNT=0
CREATED_USERS=()
TMP_DIR=$(mktemp -d)
trap "rm -rf ${TMP_DIR}" EXIT

# ユーザーデータを生成してAPIに投入
run_create_user() {
    local k=$1
    local tmpfile="${TMP_DIR}/result_${k}.txt"
    # ランダムな名前を生成
    last_name=${LAST_NAMES[$((RANDOM % ${#LAST_NAMES[@]}))]}  
    first_name=${FIRST_NAMES[$((RANDOM % ${#FIRST_NAMES[@]}))]}  
    full_name="${last_name}${first_name}"

    # ランダムなメールアドレスを生成（英数字のみで安全に）
    domain=${DOMAINS[$((RANDOM % ${#DOMAINS[@]}))]}  
    # ランダムな英字ユーザー名を生成 (日本語名をローマ字変換する代わりに)
    random_chars="abcdefghijklmnopqrstuvwxyz"
    username=""
    for ((j=0; j<8; j++)); do
        username+=${random_chars:$((RANDOM % ${#random_chars})):1}
    done
    username+="$((RANDOM % 1000))"
    email="${username}@${domain}"

    # JSONデータを作成
    json_data="{\"name\":\"${full_name}\",\"email\":\"${email}\"}"
    
    # APIにPOSTリクエストを送信し、HTTPステータスコードも取得
    response=$(curl -s -w "\n%{http_code}" -X POST "${API_URL}" \
                 -H "Content-Type: application/json" \
                 -d "${json_data}")
    
    # レスポンスからHTTPステータスコードを分離
    http_code=$(echo "${response}" | tail -n1)
    response_body=$(echo "${response}" | head -n -1)

    # HTTPステータスコードでレスポンスを判定
    if [[ "${http_code}" == "201" ]]; then
        # 成功時は直接ユーザーデータが返される
        user_id=$(echo "${response_body}" | jq -r '.id')
        echo "SUCCESS:${user_id}" > "${tmpfile}"
    else
        # エラー時は{"error": "エラーメッセージ"}の形式
        error_msg=$(echo "${response_body}" | jq -r '.error // "不明なエラー"')
        echo "FAIL:${full_name}:${http_code}:${error_msg}" > "${tmpfile}"
    fi
}

echo "ランダムユーザーデータの生成と投入を開始します..."
echo "合計 ${NUM_USERS} 人のユーザーを作成します"
echo "-------------------------------------------------"

job_count=0
# ユーザーデータを生成してAPIに投入
for ((k=1; k<=NUM_USERS; k++)); do
    run_create_user "$k" &
    job_count=$((job_count + 1))
    # MAX_PARALLELごとにwait
    if (( job_count % MAX_PARALLEL == 0 )); then
        wait
        show_progress "$k" "${NUM_USERS}"
    fi
done
wait
show_progress "${NUM_USERS}" "${NUM_USERS}"

# 結果集計
for f in "${TMP_DIR}"/result_*.txt; do
    if grep -q '^SUCCESS:' "$f"; then
        user_id=$(cut -d: -f2 "$f")
        CREATED_USERS+=("${user_id}")
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        # 失敗時はエラー内容を表示
        IFS=: read _ name code msg < "$f"
        echo -e "\r\033[K❌ ユーザー作成失敗: ${name} (HTTP ${code}: ${msg})"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done

echo -e "\n\n作成完了! 成功: ${SUCCESS_COUNT}, 失敗: ${FAIL_COUNT}"

# 作成したユーザーのサンプルを表示
echo -e "\n作成したユーザーのサンプル (最大5件):"
SAMPLE_COUNT=$(( ${#CREATED_USERS[@]} > 5 ? 5 : ${#CREATED_USERS[@]} ))

for ((i=0; i<SAMPLE_COUNT; i++)); do
    user_id=${CREATED_USERS[$i]}
    echo "ID: ${user_id}"

    # ユーザー情報を取得して表示
    user_response=$(curl -s -w "\n%{http_code}" "${API_URL}/${user_id}")
    user_http_code=$(echo "${user_response}" | tail -n1)
    user_body=$(echo "${user_response}" | head -n -1)
    
    if [[ "${user_http_code}" == "200" ]]; then
        # 成功時は直接ユーザーデータが返される
        name=$(echo "${user_body}" | jq -r '.name')
        email=$(echo "${user_body}" | jq -r '.email')
        echo "  名前: ${name}"
        echo "  メール: ${email}"
    else
        # エラー時
        error_msg=$(echo "${user_body}" | jq -r '.error // "不明なエラー"')
        echo "  エラー: HTTP ${user_http_code}: ${error_msg}"
    fi
done

echo -e "\n全ユーザーリストの取得:"
echo "curl ${API_URL} | jq"

echo -e "\nスクリプト実行完了!"