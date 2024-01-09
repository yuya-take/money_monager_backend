use serde::{Deserialize, Serialize};

use super::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUserFromEmail {
    pub username: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUserFromOAuth {
    pub username: String,
    pub email: String,
    pub oauth_provider: String,
    pub oauth_provider_id: String,
}
