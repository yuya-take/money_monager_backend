use money_manager_domain::model::user::User;

#[derive(Debug, Clone)]
pub struct UserView {
    pub user_id: String,
    pub username: String,
}

impl From<User> for UserView {
    fn from(user: User) -> Self {
        UserView {
            user_id: user.user_id,
            username: user.username,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_view_from_user() {
        let user = User {
            user_id: "user_id".to_string(),
            username: "username".to_string(),
        };
        let user_view = UserView::from(user);
        assert_eq!(user_view.user_id, "user_id");
        assert_eq!(user_view.username, "username");
    }
}
