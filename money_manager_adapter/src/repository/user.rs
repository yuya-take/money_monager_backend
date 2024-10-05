use crate::{model::user::StoredUser, repository::DynamoDBRepositoryImpl};
use money_manager_domain::model::user::User;

use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use money_manager_domain::repository::user::UserRepository;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;

#[async_trait]
impl UserRepository for DynamoDBRepositoryImpl<User> {
    async fn get_user_by_user_id(&self, user_id: String) -> anyhow::Result<Option<User>> {
        let pk_value = format!("User#{}", user_id);
        let stored_user_row = self
            .dynamo_db
            .client
            .query()
            .table_name("MoneyManager")
            .key_condition_expression("#pk = :pk_value")
            .expression_attribute_names("#pk", "UserId")
            .expression_attribute_values(":pk_value", AttributeValue::S(pk_value))
            .limit(1)
            .send()
            .await?;

        if let Some(items) = stored_user_row.items {
            if let Some(first_item) = items.into_iter().next() {
                let stored_user: StoredUser = from_item(first_item)?;
                let user = User::try_from(stored_user)?;
                Ok(Some(user))
            } else {
                // items が空の場合
                Ok(None)
            }
        } else {
            // items フィールドが None の場合
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use money_manager_domain::repository::user::UserRepository;

    use crate::{
        persistence::dynamodb::init_client,
        repository::{DynamoDB, DynamoDBRepositoryImpl},
    };

    #[tokio::test]
    async fn test_get_user_by_user_id() {
        let user_id = "user_id".to_string();
        let client = init_client().await;
        let dynamo_db = DynamoDB::new(client);
        let repository = DynamoDBRepositoryImpl::new(dynamo_db);
        let _user = repository.get_user_by_user_id(user_id).await.unwrap();
        // assert_eq!(user.unwrap(), user);
    }
}
