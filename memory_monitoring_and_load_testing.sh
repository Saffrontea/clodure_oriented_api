#!/usr/bin/env bash

# APIサーバー起動
./clodure_oriented_api &
API_PID=$!

# メモリ監視（バックグラウンド）
while true; do
  echo "$(date): $(ps -p $API_PID -o rss= | tr -d ' ') KB" >> ./log/memory_usage.log
  sleep 10
done 
# LOOP_PID=$!

# echo "API PID: $API_PID"
# echo "MEMORY MONITOR PID: $LOOP_PID"

# # APIの起動待ち（必要に応じて調整）
# sleep 3

# # ランダムユーザー生成＆投入スクリプトを実行
# echo "generate_random_users.sh を実行します..."
# bash generate_random_users.sh > user_generation.log 2>&1

# echo "ユーザー生成スクリプトの実行が完了しました。"

# # APIとメモリ監視を停止
# kill $API_PID $LOOP_PID

# # ログ出力
# echo "--- ユーザー生成ログ（末尾20行） ---"
# tail -20 user_generation.log

# echo "--- メモリ使用量ログ（末尾20行） ---"
# tail -20 memory_usage.log
