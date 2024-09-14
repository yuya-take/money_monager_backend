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
- 削除済みユーザーの場合は、エラーメッセージを返す
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
- 削除済みの収入情報は取得しない
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
- 削除済みの支出情報は取得しない
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
- 削除済みのカテゴリ情報は取得しない
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
- 削除済みの予算情報は取得しない
- 月予算と年予算の両方を取得する

#### 18. patchBudget (api/budget/{budgetId})
- ユーザーが登録した予算情報を更新する
- 更新情報には金額が含まれる
- アクセストークン認証によって得られるユーザー情報とパスパラメータのbudgetIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

#### 19. deleteBudget (api/budget/{budgetId})
- ユーザーが登録した予算情報を削除する
- アクセストークン認証によって得られるユーザー情報とパスパラメータのbudgetIdのレコードに該当するユーザーIDが一致しない場合は、エラーメッセージを返す

## テーブル定義書

| PK / GSI-2-SK / GSI-3-PK / GSI-4-PK | SK                      | GSI-1-PK                         | GSI-1-SK / GSI-4-SK | GSI-2-PK          | GSI-3-SK        | Attributes                                                                                 |
| ----------------------------------- | ----------------------- | -------------------------------- | ------------------- | ----------------- | --------------- | ------------------------------------------------------------------------------------------ |
| UserID                              | DateType                | CategoryID                       | Date                | IsDeleted         | Amount          | -                                                                                          |
| `USER#<ユーザーID>`                 | `PROFILE`               | -                                | -                   | `isDeleted#false` | -               | `email`, `providerName`, `providerId`, `activeFlag`, `createdAt`, `updatedAt`, `isDeleted` |
| `USER#<ユーザーID>`                 | `OTP#<コードID>`        | -                                | -                   | `isDeleted#false` | -               | `otpCode`, `generatedAt`, `expiresAt`, `usedFlag`, `isDeleted`                             |
| `USER#<ユーザーID>`                 | `INCOME#<収入ID>`       | `CATEGORY#<カテゴリID>`          | `DATE#<日付>`       | `isDeleted#false` | `AMOUNT#<金額>` | `categoryId`, `amount`, `date`, `description`, `createdAt`, `updatedAt`, `isDeleted`       |
| `USER#<ユーザーID>`                 | `EXPENSE#<支出ID>`      | `CATEGORY#<カテゴリID>`          | `DATE#<日付>`       | `isDeleted#false` | `AMOUNT#<金額>` | `categoryId`, `amount`, `date`, `description`, `createdAt`, `updatedAt`, `isDeleted`       |
| `USER#<ユーザーID>`                 | `CATEGORY#<カテゴリID>` | `CATEGORY_TYPE#<カテゴリタイプ>` | -                   | `isDeleted#false` | -               | `name`, `userId`, `categoryType`, `createdAt`, `updatedAt`, `isDeleted`                    |
| `USER#<ユーザーID>`                 | `BUDGET#<予算ID>`       |                                  | `DATE#<年>#<月>`    | `isDeleted#false` | -               | `amount`, `year`, `month`, `createdAt`, `updatedAt`, `isDeleted`                           |


### 各キーの説明

#### PK (Partition Key)
- **PK**（パーティションキー）は、DynamoDB でアイテムをグループ化するキーです。この設計では、ユーザーごとにデータをグループ化し、それぞれのユーザーに固有の情報をまとめます。
  - 例: `USER#12345` はユーザーID 12345 に対応するデータをグループ化します。

#### SK (Sort Key)
- **SK**（ソートキー）は、パーティション内でデータを特定し、さらに詳細な情報をソートします。各エンティティ（プロフィール、収入、支出など）の区別に使います。
  - 例: `PROFILE`、`INCOME#INCOME001`、`EXPENSE#EXPENSE001`など。

#### GSI-1 (Global Secondary Index - 1)
- **GSI-1-PK** と **GSI-1-SK** は、カテゴリや日付を基にしたクエリをサポートします。
  - **GSI-1-PK**: カテゴリによるフィルタリングに使用され、`CATEGORY#<カテゴリID>` が設定されています。
  - **GSI-1-SK**: 日付でのソートをサポートし、`DATE#<日付>` が設定されています。
  - 例: 収入データをカテゴリごとに日付順でクエリ可能にします。

#### GSI-2 (Global Secondary Index - 2)
- **GSI-2-PK** と **GSI-2-SK** は、削除フラグに基づくクエリをサポートします。
  - **GSI-2-PK**: 削除されていないデータ（`isDeleted: false`）のみに基づいてクエリを行うために使用します。`isDeleted#false` が設定されています。
  - **GSI-2-SK**: 金額によるソートをサポートし、`AMOUNT#<金額>` が設定されています。
  - 例: 削除されていないデータを金額順に取得できます。

#### GSI-3 (Global Secondary Index - 3)
- **GSI-3-PK** と **GSI-3-SK** は、将来的に追加のフィルタリングやソート要件に対応できるように、柔軟に設定可能です。現在は使用されていませんが、特定の要件が発生した際に拡張できます。

---

### 具体例

#### ユーザー管理

| PK           | SK        | email            | providerName | providerId | activeFlag | createdAt           | updatedAt           | isDeleted | GSI-2-PK          |
| ------------ | --------- | ---------------- | ------------ | ---------- | ---------- | ------------------- | ------------------- | --------- | ----------------- |
| `USER#12345` | `PROFILE` | user@example.com | Google       | 12345      | true       | 2024-09-01 10:00:00 | 2024-09-01 12:00:00 | false     | `isDeleted#false` |

#### 二重認証コード管理

| PK           | SK           | otpCode | generatedAt         | expiresAt           | usedFlag | isDeleted | GSI-2-PK          |
| ------------ | ------------ | ------- | ------------------- | ------------------- | -------- | --------- | ----------------- |
| `USER#12345` | `OTP#OTP001` | 123456  | 2024-09-01 10:00:00 | 2024-09-01 10:05:00 | false    | false     | `isDeleted#false` |

#### 収入管理

| PK           | SK                 | categoryId      | amount | date       | description   | createdAt           | updatedAt           | isDeleted | GSI-1-PK        | GSI-1-SK          | GSI-2-PK          | GSI-2-SK     |
| ------------ | ------------------ | --------------- | ------ | ---------- | ------------- | ------------------- | ------------------- | --------- | --------------- | ----------------- | ----------------- | ------------ |
| `USER#12345` | `INCOME#INCOME001` | `CATEGORY#FOOD` | 500    | 2024-09-01 | Grocery Store | 2024-09-01 10:00:00 | 2024-09-01 12:00:00 | false     | `CATEGORY#FOOD` | `DATE#2024-09-01` | `isDeleted#false` | `AMOUNT#500` |

#### 支出管理

| PK           | SK                   | categoryId      | amount | date       | description | createdAt           | updatedAt           | isDeleted | GSI-1-PK        | GSI-1-SK          | GSI-2-PK          | GSI-2-SK     |
| ------------ | -------------------- | --------------- | ------ | ---------- | ----------- | ------------------- | ------------------- | --------- | --------------- | ----------------- | ----------------- | ------------ |
| `USER#12345` | `EXPENSE#EXPENSE001` | `CATEGORY#FOOD` | 300    | 2024-09-01 | Restaurant  | 2024-09-01 10:00:00 | 2024-09-01 12:00:00 | false     | `CATEGORY#FOOD` | `DATE#2024-09-01` | `isDeleted#false` | `AMOUNT#300` |

#### カテゴリ管理

| PK           | SK                     | name      | userId       | categoryType | createdAt           | updatedAt           | isDeleted | GSI-2-PK          |
| ------------ | ---------------------- | --------- | ------------ | ------------ | ------------------- | ------------------- | --------- | ----------------- |
| `USER#12345` | `CATEGORY#CATEGORY001` | Groceries | `USER#12345` | `INCOME`     | 2024-09-01 10:00:00 | 2024-09-01 12:00:00 | false     | `isDeleted#false` |

#### 予算管理

| PK           | SK                 | amount | year | month | createdAt           | updatedAt           | isDeleted | GSI-2-PK          |
| ------------ | ------------------ | ------ | ---- | ----- | ------------------- | ------------------- | --------- | ----------------- |
| `USER#12345` | `BUDGET#BUDGET001` | 20000  | 2024 | 9     | 2024-09-01 10:00:00 | 2024-09-01 12:00:00 | false     | `isDeleted#false` |


### ユースケースの確認
ユースケースごとの確認
1. postLogin:
   
    ユーザー管理テーブル (USER#<ユーザーID>, PROFILE) はGoogleログインに対応しています。削除されたユーザーを考慮する場合、isDeleted フラグをチェックすることで、削除済みユーザーの処理も適切に行えます。

2. postOTP:

    二重認証コード管理 (OTP#<コードID>) に対しても、isDeleted フラグをチェックしてクエリできます。OTPの生成日時や有効期限の管理も既に含まれています。

3. getUser:

    USER#<ユーザーID> の PROFILE を取得するためのクエリは、この設計で対応可能です。アクセストークンの認証を行い、適切にユーザー情報が取得できます。

4. postIncome:

    収入管理 (INCOME#<収入ID>) に対して、ユーザーIDとカテゴリIDでのフィルタリングが可能です。isDeleted フラグを使って削除済みデータを除外し、収入データの登録が可能です。

5. getIncomes:

    収入データの取得に関しては、CATEGORY#<カテゴリID> や DATE#<日付> に基づいてフィルタリングできます。また、isDeleted フラグを使って削除済みデータを除外してクエリできます。

6. patchIncome:

    収入情報の更新も、該当する INCOME#<収入ID> アイテムに対して行うことができ、isDeleted フラグを適切に管理できます。

7. deleteIncome:

    削除操作は、物理削除ではなく、isDeleted フラグを true に更新することで対応可能です。このフラグは GSI を使って効率的に管理されます。

8. postExpense:

    支出管理 (EXPENSE#<支出ID>) に関しても、収入管理と同様の方法でフィルタリングと削除フラグの管理が行えます。

9. getExpenses:

    支出データの取得においても、カテゴリや日付でのフィルタリングが可能で、isDeleted を使って削除済みデータを除外することができます。

10. patchExpense:

    支出情報の更新は、収入と同様に EXPENSE#<支出ID> をターゲットとし、削除フラグを使った管理が可能です。

11. deleteExpense:

    削除操作は、支出データに対しても isDeleted を使った論理削除で対応できます。

12. postCategory:

    カテゴリの登録・管理に関しては、カテゴリタイプや名前でのフィルタリングが可能です。

13. getCategories:

    カテゴリ取得も、削除フラグを除外しながら、カテゴリ名やタイプに基づいた検索が可能です。

14. patchCategory:

    カテゴリの更新は、CATEGORY#<カテゴリID> に対して行います。更新情報は、この設計で問題なく管理できます。

15. deleteCategory:

    削除フラグを使った論理削除が可能で、GSI も削除フラグを用いたクエリに対応しています。

16. postBudget:

    予算の登録は BUDGET#<予算ID> に対して行い、ユーザーごとの予算データをカテゴリや年・月で管理できます。

17. getBudgets:

    削除されていない予算データを、削除フラグを用いて効率的に取得することが可能です。

18. patchBudget:

    予算の更新は、該当する BUDGET#<予算ID> に対して行うことができ、削除フラグを適切に管理できます。

19. deleteBudget:

    削除フラグを使った論理削除に対応しています。GSI を使用して削除済みデータのクエリにも対応できます。
