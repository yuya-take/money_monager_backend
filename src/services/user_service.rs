use diesel::prelude::*;

use crate::models::{
    User,
    NewUserFromEmail,
    NewUserFromOAuth,
};
use crate::schema::users;
use crate::db::establish_connection;

pub fn get_user(user_id: i64) -> User {
    let connection = establish_connection();
    users::table.find(user_id).get_result::<User>(&connection).expect("Error loading user")
}
