# 要件定義書

## 1. はじめに
この文書は、個人財務管理アプリの開発における要件を定義するものです。

## 2. 目的
本アプリの主な目的は、ユーザーが自身の財務を簡単に管理できるようにすることです。

## 3. 機能要件
### 3.1 収入の管理 (v1)
- ユーザーは収入を記録できる
- 収入源のカテゴリーを設定できる

### 3.2 支出の管理 (v1)
- ユーザーは支出を記録できる
- 支出のカテゴリーを設定できる

### 3.3 予算の管理 (v1)
- ユーザーは月ごとの予算を設定できる
- 予算の使用状況を確認できる

### 3.4 収支の分析 (v1.1)
- ユーザーは収支の傾向を分析できる
- 収入と支出の比較をグラフで表示できる

### 3.5 グループ管理機能 (v2)
- ユーザーは複数のグループを作成できる
- 各グループで収入と支出を管理できる
- グループごとに予算を設定できる
- グループメンバー間での財務情報を共有できる

## 4. 非機能要件
### 4.1 セキュリティ (v1)
- ユーザー情報は安全に保管される
- トランザクションデータは暗号化される

### 4.2 パフォーマンス (v1)
- アプリは迅速に応答する

### 4.3 互換性 (v1.1)
- 主要なスマートフォンOSで動作する

## 5. 制約事項 (v1)
- インターネット接続が必要

## 6. 用語集
- 収入: ユーザーが得た金銭
- 支出: ユーザーが支払った金銭
- 予算: ユーザーが設定した期間内の支出限度額

## 7. その他
### 技術スタック
- バックエンド: Rust
- データベース: PostgreSQL
- 認証: OAuth 2.0
- フロントエンド: sveltekit + tauri
