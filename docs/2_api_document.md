# API設計書

## API一覧

- [API設計書](#api設計書)
  - [API一覧](#api一覧)
  - [API-001: ログイン](#api-001-ログイン)
  - [API-002: 欠番](#api-002-欠番)
  - [API-003: ユーザー情報詳細取得](#api-003-ユーザー情報詳細取得)
  - [API-004: 収入登録](#api-004-収入登録)
  - [API-005: 収入一覧取得](#api-005-収入一覧取得)
  - [API-006: 収入更新](#api-006-収入更新)
  - [API-007: 収入削除](#api-007-収入削除)
  - [API-008: 支出登録](#api-008-支出登録)
  - [API-009: 支出一覧取得](#api-009-支出一覧取得)
  - [API-010: 支出更新](#api-010-支出更新)
  - [API-011: 支出削除](#api-011-支出削除)
  - [API-012: カテゴリ登録](#api-012-カテゴリ登録)
  - [API-013: カテゴリ一覧取得](#api-013-カテゴリ一覧取得)
  - [API-014: カテゴリ更新](#api-014-カテゴリ更新)
  - [API-015: カテゴリ削除](#api-015-カテゴリ削除)



## API-001: ログイン
**メソッド**: POST  
**エンドポイント**: /api/auth/login  
**名前**: Login  
**説明**: ログインする

**DB操作**: Usersテーブルに新しいレコードを作成します。

**リクエストパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| authCode | string | Yes | google認証から得られるコード |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|
| userId | integer | ユーザーID |
| email | string | メールアドレス |
| accessToken | string | アクセストークン |
| refreshToken | string | リフレッシュトークン |
| providerName | string | プロバイダー名 |

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 500 | サーバーエラー |

---

## API-002: 欠番

---

## API-003: ユーザー情報詳細取得

**メソッド**: GET  
**エンドポイント**: /user/{userId}
**Authorization**: Bearer <accessToken>
**名前**: GetUser  
**説明**: 指定したIDのユーザー情報を取得します。  

**DB操作**: Usersテーブルから指定したIDのレコードを取得します。


**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|
| userId | integer | ユーザーID |
| username | string | ユーザー名 |
| email | string | メールアドレス |
| userIcon | string | ユーザーアイコン |

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | ユーザーが見つからない |
| 500 | サーバーエラー |

---

## API-004: 収入登録

**メソッド**: POST
**エンドポイント**: /income
**Authorization**: Bearer <accessToken>
**名前**: CreateIncome
**説明**: 収入を登録します。

**DB操作**: 新規のIncomeに関するレコードを作成します。

**リクエストパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| userId | string | Yes | ユーザーID |
| categoryId | string | No | カテゴリID |
| amount | integer | YES | 金額 |
| registeredAt | string | Yes | 登録日時 |
| description | string | No | 説明 |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | ユーザーが見つからない |
| 500 | サーバーエラー |

---

## API-005: 収入一覧取得

**メソッド**: GET
**エンドポイント**: /income
**Authorization**: Bearer <accessToken>
**名前**: GetIncomeList
**説明**: 収入一覧を取得します。

**DB操作**: Incomeテーブルから指定したユーザーIDのレコードを取得します。

**クエリパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| registerdAtStart | string | No | 登録日時の範囲指定(開始) |
| registerdAtEnd | string | No | 登録日時の範囲指定(終了) |
| AmountStart | integer | No | 金額の範囲指定(開始) |
| AmountEnd | integer | No | 金額の範囲指定(終了) |
| categoryId | string | No | カテゴリIDをカンマ区切り |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|
| incomeId | integer | 収入ID |
| incomeName | string | 収入名 |
| amount | integer | 金額 |
| registeredAt | string | 登録日時 |
| description | string | 説明 |
| categoryId | integer | カテゴリID |

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | ユーザーが見つからない |
| 500 | サーバーエラー |

---

## API-006: 収入更新

**メソッド**: PUT
**エンドポイント**: /income/{incomeId}
**Authorization**: Bearer <accessToken>
**名前**: UpdateIncome
**説明**: 収入を更新します。

**DB操作**: Incomeテーブルから指定した収入IDのレコードを更新します。

**リクエストパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| categoryId | integer | No | カテゴリID |
| amount | integer | No | 金額 |
| registeredAt | string | No | 登録日時 |
| description | string | No | 説明 |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|   

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | 収入が見つからない |
| 500 | サーバーエラー |

---

## API-007: 収入削除

**メソッド**: DELETE
**エンドポイント**: /income/{incomeId}
**Authorization**: Bearer <accessToken>
**名前**: DeleteIncome
**説明**: 収入を削除します。

**DB操作**: Incomeテーブルから指定した収入IDのレコードを削除します。

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 404 | 収入が見つからない |
| 500 | サーバーエラー |

---

## API-008: 支出登録

**メソッド**: POST
**エンドポイント**: /expense
**Authorization**: Bearer <accessToken>
**名前**: CreateExpense
**説明**: 支出を登録します。

**DB操作**: 新規のExpenseに関するレコードを作成します。

**リクエストパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| userId | string | Yes | ユーザーID |
| categoryId | string | No | カテゴリID |
| amount | integer | YES | 金額 |
| registeredAt | string | Yes | 登録日時 |
| description | string | No | 説明 |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | ユーザーが見つからない |
| 500 | サーバーエラー |

---

## API-009: 支出一覧取得

**メソッド**: GET
**エンドポイント**: /expense
**Authorization**: Bearer <accessToken>
**名前**: GetExpenseList
**説明**: 支出一覧を取得します。

**DB操作**: Expenseテーブルから指定したユーザーIDのレコードを取得します。

**クエリパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| registerdAtStart | string | No | 登録日時の範囲指定(開始) |
| registerdAtEnd | string | No | 登録日時の範囲指定(終了) |
| AmountStart | integer | No | 金額の範囲指定(開始) |
| AmountEnd | integer | No | 金額の範囲指定(終了) |
| categoryId | string | No | カテゴリIDをカンマ区切り |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|
| expenseId | integer | 支出ID |
| expenseName | string | 支出名 |
| amount | integer | 金額 |
| registeredAt | string | 登録日時 |
| description | string | 説明 |
| categoryId | integer | カテゴリID |

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | ユーザーが見つからない |
| 500 | サーバーエラー |

---

## API-010: 支出更新

**メソッド**: PUT
**エンドポイント**: /expense/{expenseId}
**Authorization**: Bearer
**名前**: UpdateExpense
**説明**: 支出を更新します。

**DB操作**: Expenseテーブルから指定した支出IDのレコードを更新します。

**リクエストパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| categoryId | integer | No | カテゴリID |
| amount | integer | No | 金額 |
| registeredAt | string | No | 登録日時 |
| description | string | No | 説明 |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | 支出が見つからない |
| 500 | サーバーエラー |

---

## API-011: 支出削除

**メソッド**: DELETE
**エンドポイント**: /expense/{expenseId}
**Authorization**: Bearer <accessToken>
**名前**: DeleteExpense
**説明**: 支出を削除します。

**DB操作**: Expenseテーブルから指定した支出IDのレコードを削除します。

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 404 | 支出が見つからない |
| 500 | サーバーエラー |

---

## API-012: カテゴリ登録

**メソッド**: POST
**エンドポイント**: /category
**Authorization**: Bearer <accessToken>
**名前**: CreateCategory
**説明**: カテゴリを登録します。

**DB操作**: 新規のCategoryに関するレコードを作成します。

**リクエストパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| userId | string | Yes | ユーザーID |
| categoryName | string | YES | カテゴリ名 |
| categoryType | string | YES | カテゴリタイプ(income or expense) |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | ユーザーが見つからない |
| 500 | サーバーエラー |

---

## API-013: カテゴリ一覧取得

**メソッド**: GET
**エンドポイント**: /category
**Authorization**: Bearer <accessToken>
**名前**: GetCategoryList
**説明**: カテゴリ一覧を取得します。

**DB操作**: Categoryテーブルから指定したユーザーIDのレコードを取得します。

**クエリパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| categoryType | string | NO | カテゴリタイプ(income or expense) |
| searchWord | string | NO | 検索ワード |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|
| categoryId | integer | カテゴリID |
| categoryName | string | カテゴリ名 |
| categoryType | string | カテゴリタイプ(income or expense) |

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | ユーザーが見つからない |
| 500 | サーバーエラー |

---

## API-014: カテゴリ更新

**メソッド**: PUT
**エンドポイント**: /category/{categoryId}
**Authorization**: Bearer <accessToken>
**名前**: UpdateCategory
**説明**: カテゴリを更新します。

**DB操作**: Categoryテーブルから指定したカテゴリIDのレコードを更新します。

**リクエストパラメータ**

| パラメータ名 | 型 | 必須 | 説明 |
|---|---|---|---|
| categoryName | string | YES | カテゴリ名 |

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 400 | リクエストパラメータが不正 |
| 404 | カテゴリが見つからない |
| 500 | サーバーエラー |

---

## API-015: カテゴリ削除

**メソッド**: DELETE
**エンドポイント**: /category/{categoryId}
**Authorization**: Bearer <accessToken>
**名前**: DeleteCategory
**説明**: カテゴリを削除します。

**DB操作**: Categoryテーブルから指定したカテゴリIDのレコードを削除します。

**レスポンス**

| パラメータ名 | 型 | 説明 |
|---|---|---|

**エラーレスポンス**

| HTTPステータスコード | 説明 |
|---|---|
| 404 | カテゴリが見つからない |
| 500 | サーバーエラー |

