# データベース設計書

## RDB風
### ユーザー管理 (googleログインのみ許容)
- ユーザーID
- メールアドレス
- プロバイダー名
- プロバイダーID
- アクティブフラグ
- 作成日
- 更新日

### 二重認証コード管理
- コードID
- ユーザーID
- ワンタイムコード
- 生成日時
- 有効期限
- 使用済みフラグ

### 収入管理
- 収入ID
- ユーザーID
- カテゴリID
- 金額
- 日付
- 説明
- 作成日
- 更新日

### 支出管理
- 支出ID
- ユーザーID
- カテゴリID
- 金額
- 日付
- 説明
- 作成日
- 更新日

### カテゴリ管理
- カテゴリID
- 名称
- ユーザーID (オプション)
- カテゴリタイプ(収入or支出)

### 予算管理
- 予算ID
- ユーザーID
- 金額
- 月
- 年
- 作成日
- 更新日


## ユースケース
### ユースケース一覧
| #   | Entity   | UseCase        | Description          |
| --- | -------- | -------------- | -------------------- |
| 1   | Login    | postLogin      | Googleログイン       |
| 2   | OTP      | postOTP        | ワンタイムパスワード |
| 3   | User     | getUser        | ユーザー情報取得     |
| 4   | Income   | postIncome     | 収入登録             |
| 5   | Income   | getIncomes     | 収入一覧取得         |
| 6   | Income   | patchIncome    | 収入更新             |
| 7   | Income   | deleteIncome   | 収入削除             |
| 8   | Expense  | postExpense    | 支出登録             |
| 9   | Expense  | getExpenses    | 支出一覧取得         |
| 10  | Expense  | patchExpense   | 支出更新             |
| 11  | Expense  | deleteExpense  | 支出削除             |
| 12  | Category | postCategory   | カテゴリ登録         |
| 13  | Category | getCategories  | カテゴリ一覧取得     |
| 14  | Category | patchCategory  | カテゴリ更新         |
| 15  | Category | deleteCategory | カテゴリ削除         |
| 16  | Budget   | postBudget     | 予算登録             |
| 17  | Budget   | getBudgets     | 予算一覧取得         |
| 18  | Budget   | patchBudget    | 予算更新             |
| 19  | Budget   | deleteBudget   | 予算削除             |

### ユースケース詳細
#### 1. postLogin (api/auth/login)
- ユーザーがGoogleログインを行う
- ユーザーがログインに成功すると、ユーザー情報を返す
- 新規ユーザーの場合は、ユーザー情報を登録しOTPを生成する
- ユーザー情報が登録済みの場合は、OTPを生成する

#### 2. postOTP (api/auth/otp)
- ユーザーがワンタイムパスワードを入力する
- ワンタイムパスワードが正しい場合は、アクセストークン・リフレッシュトークンを返す
- ワンタイムパスワードが正しくない場合は、エラーメッセージを返す

#### 3. getUser (api/user/{userId})
- アクセストークンが正しい場合は、ユーザー情報を返す
- アクセストークンが正しくない場合は、エラーメッセージを返す
- アクセストークン認証によって得られるユーザー情報とパスパラメータのユーザーIDが一致しない場合は、エラーメッセージを返す
- ユーザー情報が存在しない場合は、エラーメッセージを返す

#### 4. postIncome (api/income)
- ユーザーが収入を登録する
- 登録情報にはユーザーID、カテゴリID、金額、日付、説明が含まれる
- アクセストークン認証によって得られるユーザー情報とリクエストボディのユーザーIDが一致しない場合は、エラーメッセージを返す
- 登録情報が不正な場合は、エラーメッセージを返す

#### 5. getIncomes (api/income)
- ユーザーが登録した収入一覧を取得する
- 日時によるソートが可能
- 金額によるソートが可能
- カテゴリによるフィルタリングが可能

#### 6. patchIncome (api/income/{incomeId})
- ユーザーが登録した収入情報を更新する
- 更新情報にはカテゴリID、金額、日付、説明が含まれる
- アクセストークン認証によって得られるユーザー情報とパスパラメータのincomeIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

#### 7. deleteIncome (api/income/{incomeId})
- ユーザーが登録した収入情報を削除する
- アクセストークン認証によって得られるユーザー情報とパスパラメータのincomeIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

#### 8. postExpense (api/expense)
- ユーザーが支出を登録する
- 登録情報にはユーザーID、カテゴリID、金額、日付、説明が含まれる
- アクセストークン認証によって得られるユーザー情報とリクエストボディのユーザーIDが一致しない場合は、エラーメッセージを返す
- 登録情報が不正な場合は、エラーメッセージを返す

#### 9. getExpenses (api/expense)
- ユーザーが登録した支出一覧を取得する
- 日時によるソートが可能
- 金額によるソートが可能
- カテゴリによるフィルタリングが可能

#### 10. patchExpense (api/expense/{expenseId})
- ユーザーが登録した支出情報を更新する
- 更新情報にはカテゴリID、金額、日付、説明が含まれる
- アクセストークン認証によって得られるユーザー情報とパスパラメータのexpenseIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

#### 11. deleteExpense (api/expense/{expenseId})
- ユーザーが登録した支出情報を削除する
- アクセストークン認証によって得られるユーザー情報とパスパラメータのexpenseIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

#### 12. postCategory (api/category)
- ユーザーがカテゴリを登録する
- 登録情報にはユーザーID、名称、カテゴリタイプが含まれる
- アクセストークン認証によって得られるユーザー情報とリクエストボディのユーザーIDが一致しない場合は、エラーメッセージを返す
- 登録情報が不正な場合は、エラーメッセージを返す

#### 13. getCategories (api/category)
- ユーザーが登録したカテゴリ一覧を取得する
- カテゴリタイプによるフィルタリングが可能
- カテゴリ名による検索が可能

#### 14. patchCategory (api/category/{categoryId})
- ユーザーが登録したカテゴリ情報を更新する
- 更新情報には名称が含まれる
- アクセストークン認証によって得られるユーザー情報とパスパラメータのcategoryIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

#### 15. deleteCategory (api/category/{categoryId})
- ユーザーが登録したカテゴリ情報を削除する
- アクセストークン認証によって得られるユーザー情報とパスパラメータのcategoryIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

#### 16. postBudget (api/budget)
- ユーザーが予算を登録する
- 登録情報にはユーザーID、金額、月、年が含まれる
- アクセストークン認証によって得られるユーザー情報とリクエストボディのユーザーIDが一致しない場合は、エラーメッセージを返す
- 予算は月予算と年予算の2種類がある
- 月予算の合計が年予算を超えることはできない
- 登録情報が不正な場合は、エラーメッセージを返す

#### 17. getBudgets (api/budget)
- ユーザーが登録した予算一覧を取得する
- 月予算と年予算の両方を取得する

#### 18. patchBudget (api/budget/{budgetId})
- ユーザーが登録した予算情報を更新する
- 更新情報には金額が含まれる
- アクセストークン認証によって得られるユーザー情報とパスパラメータのbudgetIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

#### 19. deleteBudget (api/budget/{budgetId})
- ユーザーが登録した予算情報を削除する
- アクセストークン認証によって得られるユーザー情報とパスパラメータのbudgetIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

## テーブル定義書

| PK                  | SK                      | GSI-1-PK                         | GSI-1-SK      | GSI-2-PK            | GSI-2-SK        | GSI-3-PK | GSI-3-SK | Attributes                                                                    |
| ------------------- | ----------------------- | -------------------------------- | ------------- | ------------------- | --------------- | -------- | -------- | ----------------------------------------------------------------------------- |
| `USER#<ユーザーID>` | `PROFILE`               | -                                | -             | -                   | -               | -        | -        | `email`, `providerName`, `providerId`, `activeFlag`, `createdAt`, `updatedAt` |
| `USER#<ユーザーID>` | `OTP#<コードID>`        | -                                | -             | -                   | -               | -        | -        | `otpCode`, `generatedAt`, `expiresAt`, `usedFlag`                             |
| `USER#<ユーザーID>` | `INCOME#<収入ID>`       | `CATEGORY#<カテゴリID>`          | `DATE#<日付>` | `USER#<ユーザーID>` | `AMOUNT#<金額>` | -        | -        | `categoryId`, `amount`, `date`, `description`, `createdAt`, `updatedAt`       |
| `USER#<ユーザーID>` | `EXPENSE#<支出ID>`      | `CATEGORY#<カテゴリID>`          | `DATE#<日付>` | `USER#<ユーザーID>` | `AMOUNT#<金額>` | -        | -        | `categoryId`, `amount`, `date`, `description`, `createdAt`, `updatedAt`       |
| `USER#<ユーザーID>` | `CATEGORY#<カテゴリID>` | `CATEGORY_TYPE#<カテゴリタイプ>` | -             | -                   | -               | -        | -        | `name`, `userId`, `categoryType`, `createdAt`, `updatedAt`                    |
| `USER#<ユーザーID>` | `BUDGET#<予算ID>`       | `DATE#<年>`                      | `MONTH#<月>`  | `USER#<ユーザーID>` | `AMOUNT#<金額>` | -        | -        | `amount`, `year`, `month`, `createdAt`, `updatedAt`                           |

### 各フィールドの説明
- **PK** (Partition Key): ユーザーIDに基づいた共通の識別子 (`USER#<ユーザーID>`) を使用してデータをグループ化します。
- **SK** (Sort Key): データの種類（プロフィール、収入、支出、カテゴリ、予算など）を識別します。
- **GSI-1**: カテゴリIDや日付を基に、カテゴリごとの収入や支出のクエリを効率化します。
- **GSI-2**: 金額ベースで収入や支出をソートするためのインデックスです。
- **Attributes**: 各エンティティごとの属性。

### 具体例

#### ユーザー管理
| PK           | SK        | email            | providerName | providerId | activeFlag | createdAt           | updatedAt           |
| ------------ | --------- | ---------------- | ------------ | ---------- | ---------- | ------------------- | ------------------- |
| `USER#12345` | `PROFILE` | user@example.com | Google       | 12345      | true       | 2024-09-01 10:00:00 | 2024-09-01 12:00:00 |

#### 収入管理
| PK           | SK                 | categoryId      | amount | date       | description   | createdAt           | updatedAt           |
| ------------ | ------------------ | --------------- | ------ | ---------- | ------------- | ------------------- | ------------------- |
| `USER#12345` | `INCOME#INCOME001` | `CATEGORY#FOOD` | 500    | 2024-09-01 | Grocery Store | 2024-09-01 10:00:00 | 2024-09-01 12:00:00 |

#### 支出管理
| PK           | SK                   | categoryId      | amount | date       | description | createdAt           | updatedAt           |
| ------------ | -------------------- | --------------- | ------ | ---------- | ----------- | ------------------- | ------------------- |
| `USER#12345` | `EXPENSE#EXPENSE001` | `CATEGORY#FOOD` | 300    | 2024-09-01 | Restaurant  | 2024-09-01 10:00:00 | 2024-09-01 12:00:00 |

