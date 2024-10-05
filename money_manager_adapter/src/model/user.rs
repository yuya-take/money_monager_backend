use money_manager_domain::model::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StoredUser {
    pub user_id: String,
    pub data_type: String,
    pub active_flag: String,
    pub username: String,
}
impl TryFrom<StoredUser> for User {
    type Error = anyhow::Error;

    fn try_from(value: StoredUser) -> Result<Self, Self::Error> {
        Ok(User {
            user_id: value.user_id,
            username: value.username,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_stored_user() {
        let stored_user = StoredUser {
            user_id: "user_id".to_string(),
            data_type: "data_type".to_string(),
            active_flag: "active_flag".to_string(),
            username: "username".to_string(),
        };
        let user = User::try_from(stored_user).unwrap();
        assert_eq!(user.user_id, "user_id");
        assert_eq!(user.username, "username");
    }
}
