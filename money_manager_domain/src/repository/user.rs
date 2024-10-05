use async_trait::async_trait;

use crate::model::user::User;

#[async_trait]
pub trait UserRepository {
    async fn get_user_by_user_id(&self, user_id: String) -> anyhow::Result<Option<User>>;
}
