use std::sync::Arc;

use derive_new::new;

use money_manager_adapter::modules::RepositoriesModuleExt;
use money_manager_domain::repository::user::UserRepository;

use crate::model::user::UserView;

#[derive(new)]
pub struct UserUseCase<R: RepositoriesModuleExt> {
    repository: Arc<R>,
}

impl<R: RepositoriesModuleExt> UserUseCase<R> {
    pub async fn get_user(&self, user_id: String) -> anyhow::Result<UserView> {
        let user = self
            .repository
            .user_repository()
            .get_user_by_user_id(user_id)
            .await?;
        // userがNoneではない場合は、UserViewに変換して返す
        if let Some(user) = user {
            Ok(UserView::from(user))
        } else {
            // userがNoneの場合は、エラーを返す
            tracing::error!("User not found");
            anyhow::bail!("User not found")
        }
    }
}
