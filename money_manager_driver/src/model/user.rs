use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use money_manager_app::model::user::UserView;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user_id: String,
    pub username: String,
}
impl From<UserView> for UserResponse {
    fn from(user_view: UserView) -> Self {
        UserResponse {
            user_id: user_view.user_id,
            username: user_view.username,
        }
    }
}
