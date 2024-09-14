#!/bin/bash
AWS_PROFILE="localstack"

# テーブル名
TABLE_NAME="MoneyManager"

# DynamoDBローカルエンドポイント
ENDPOINT_URL="http://localhost:8000"

# テーブルの存在確認
if aws dynamodb describe-table --table-name $TABLE_NAME --endpoint-url $ENDPOINT_URL 2>&1 | grep -q "ResourceNotFoundException"
then
    echo "テーブルが存在しません。"
else
    echo "テーブルが存在します。削除を実行します。"
    aws dynamodb delete-table --table-name $TABLE_NAME --endpoint-url $ENDPOINT_URL
fi

# テーブル作成
aws dynamodb create-table --table-name MoneyManager \
    --cli-input-json file://meta.json \
    --endpoint-url http://localhost:8000

# dummyデータの登録
# cargo run
# aws dynamodb batch-write-item --request-items file://dummy/dummy_data.json --endpoint-url http://localhost:8000
